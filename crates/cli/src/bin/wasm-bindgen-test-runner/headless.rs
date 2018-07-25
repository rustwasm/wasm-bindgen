use std::cell::{Cell, RefCell};
use std::env;
use std::io::{self, Read};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::{PathBuf, Path};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Instant, Duration};

use curl::easy::Easy;
use failure::{ResultExt, Error};
use serde::{Serialize, Deserialize};
use serde_json;

use shell::Shell;

/// Execute a headless browser tests against a server running on `server`
/// address.
pub fn run(server: &SocketAddr, shell: &Shell) -> Result<(), Error> {
    let (driver, args) = Driver::find()?;
    println!("Running headless tests in {} with `{}`",
             driver.browser(),
             driver.path().display());

    // Allow tests to run in parallel (in theory) by finding any open port
    // available for our driver. We can't bind the port for the driver, but
    // hopefully the OS gives this invocation unique ports across processes
    let driver_addr = TcpListener::bind("127.0.0.1:0")?.local_addr()?;

    // Spawn the driver binary, collecting its stdout/stderr in separate
    // threads. We'll print this output later.
    shell.status("Spawning Geckodriver...");
    let mut cmd = Command::new(&driver.path());
    cmd.args(&args)
        .arg(format!("--port={}", driver_addr.port().to_string()))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::null());
    let mut child = cmd.spawn()
        .context(format!("failed to spawn {:?} binary", driver.path()))?;
    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();
    let mut stdout = Some(thread::spawn(move || read(&mut stdout)));
    let mut stderr = Some(thread::spawn(move || read(&mut stderr)));
    let print_driver_stdio = Cell::new(true);
    let _f = OnDrop(|| {
        child.kill().unwrap();
        let status = child.wait().unwrap();
        if !print_driver_stdio.get() {
            return
        }

        shell.clear();
        println!("driver status: {}", status);

        let stdout = stdout.take().unwrap().join().unwrap().unwrap();
        if stdout.len() > 0 {
            println!("driver stdout:\n{}", tab(&String::from_utf8_lossy(&stdout)));
        }
        let stderr = stderr.take().unwrap().join().unwrap().unwrap();
        if stderr.len() > 0 {
            println!("driver stderr:\n{}", tab(&String::from_utf8_lossy(&stderr)));
        }
    });

    // Wait for the driver to come online and bind its port before we try to
    // connect to it.
    let start = Instant::now();
    let max = Duration::new(5, 0);
    let mut bound = false;
    while start.elapsed() < max {
        if TcpStream::connect(&driver_addr).is_ok() {
            bound = true;
            break
        }
        thread::sleep(Duration::from_millis(100));
    }
    if !bound {
        bail!("driver failed to bind port during startup")
    }

    let client = Client {
        handle: RefCell::new(Easy::new()),
        driver_addr,
    };
    shell.status("Starting new webdriver session...");
    // Allocate a new session with the webdriver protocol, and once we've done
    // so schedule the browser to get closed with a call to `close_window`.
    let id = client.new_session(&driver)?;
    let _f = OnDrop(|| {
        if let Err(e) = client.close_window(&id) {
            warn!("failed to close window {:?}", e);
        }
    });

    // Visit our local server to open up the page that runs tests, and then get
    // some handles to objects on the page which we'll be scraping output from.
    let url = format!("http://{}", server);
    shell.status(&format!("Visiting {}...", url));
    client.goto(&id, &url)?;
    shell.status("Loading page elements...");
    let output = client.element(&id, "#output")?;
    let logs = client.element(&id, "#console_log")?;
    let errors = client.element(&id, "#console_error")?;

    // At this point we need to wait for the test to finish before we can take a
    // look at what happened. There appears to be no great way to do this with
    // the webdriver protocol today (in terms of synchronization), so for now we
    // just go with a loop.
    //
    // We periodically check the page to see if the output contains a known
    // string to only be printed when tests have finished running.
    //
    // TODO: harness failures aren't well handled here, they always force a
    //       timeout. These sorts of failures could be "you typo'd the path to a
    //       local script" which is pretty bad to time out for, we should detect
    //       this on the page and look for such output here, printing diagnostic
    //       information.
    shell.status("Waiting for test to finish...");
    let start = Instant::now();
    let max = Duration::new(20, 0);
    while start.elapsed() < max {
        if client.text(&id, &output)?.contains("test result: ") {
            break
        }
        thread::sleep(Duration::from_millis(100));
    }
    shell.clear();

    // Tests have now finished or have timed out. At this point we need to print
    // what happened on the console. Currently we just do this by scraping the
    // output of various fields and printing them out, hopefully providing
    // enough diagnostic info to see what went wrong (if anything).
    let output = client.text(&id, &output)?;
    let logs = client.text(&id, &logs)?;
    let errors = client.text(&id, &errors)?;

    if output.contains("test result: ") {
        println!("{}", output);

        // If the tests harness finished (either successfully or unsuccessfully)
        // then in theory all the info needed to debug the failure is in its own
        // output, so we shouldn't need the driver logs to get printed.
        print_driver_stdio.set(false);
    } else {
        println!("failed to detect test as having been run");
        if output.len() > 0 {
            println!("output div contained:\n{}", tab(&output));
        }
    }
    if logs.len() > 0 {
        println!("console.log div contained:\n{}", tab(&logs));
    }
    if errors.len() > 0 {
        println!("console.log div contained:\n{}", tab(&errors));
    }

    Ok(())
}

enum Driver {
    Gecko(PathBuf),
    Safari(PathBuf),
    Chrome(PathBuf),
}

impl Driver {
    fn find() -> Result<(Driver, Vec<String>), Error> {
        let env_args = |name: &str| {
            env::var(format!("{}_ARGS", name.to_uppercase()))
                .unwrap_or_default()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        };

        let drivers = [
            ("geckodriver", Driver::Gecko as fn(PathBuf) -> Driver),
            ("safaridriver", Driver::Safari as fn(PathBuf) -> Driver),
            ("chromedriver", Driver::Chrome as fn(PathBuf) -> Driver),
        ];

        // First up, if env vars like GECKODRIVER are present, use those to
        // allow forcing usage of a particular driver.
        for (driver, ctor) in drivers.iter() {
            let env = driver.to_uppercase();
            let path = match env::var_os(&env) {
                Some(path) => path,
                None => continue,
            };
            return Ok((ctor(path.into()), env_args(driver)))
        }

        // Next, check PATH. If we can find any supported driver, use that by
        // default.
        for path in env::split_paths(&env::var_os("PATH").unwrap_or_default()) {
            let found = drivers
                .iter()
                .find(|(name, _)| {
                    path.join(name)
                        .with_extension(env::consts::EXE_EXTENSION)
                        .exists()
                });
            let (name, ctor) = match found {
                Some(p) => p,
                None => continue,
            };
            return Ok((ctor(name.into()), env_args(name)))
        }

        // TODO: download an appropriate driver? How to know which one to
        //       download?

        bail!("failed to find a suitable WebDriver binary to drive headless \
               testing")
    }

    fn path(&self) -> &Path {
        match self {
            Driver::Gecko(path) => path,
            Driver::Safari(path) => path,
            Driver::Chrome(path) => path,
        }
    }

    fn browser(&self) -> &str {
        match self {
            Driver::Gecko(_) => "Firefox",
            Driver::Safari(_) => "Safari",
            Driver::Chrome(_) => "Chrome",
        }
    }
}

struct Client {
    handle: RefCell<Easy>,
    driver_addr: SocketAddr,
}

enum Method<'a> {
    Get,
    Post(&'a str),
    Delete,
}

impl Client {
    fn new_session(&self, driver: &Driver) -> Result<String, Error> {
        match driver {
            Driver::Gecko(_) => {
                #[derive(Deserialize)]
                struct Response {
                    value: ResponseValue,
                }

                #[derive(Deserialize)]
                struct ResponseValue {
                    #[serde(rename = "sessionId")]
                    session_id: String,
                }
                let request = json!({
                    "capabilities": {
                        "alwaysMatch": {
                            "moz:firefoxOptions": {
                                "args": ["-headless"],
                            }
                        }
                    }
                });
                let x: Response = self.post("/session", &request)?;
                Ok(x.value.session_id)
            }
            Driver::Safari(_) => {
                #[derive(Deserialize)]
                struct Response {
                    #[serde(rename = "sessionId")]
                    session_id: String,
                }
                let request = json!({
                    "desiredCapabilities": {
                    }
                });
                let x: Response = self.post("/session", &request)?;
                Ok(x.session_id)
            }
            Driver::Chrome(_) => {
                #[derive(Deserialize)]
                struct Response {
                    #[serde(rename = "sessionId")]
                    session_id: String,
                }
                let request = json!({
                    "desiredCapabilities": {
                    }
                });
                let x: Response = self.post("/session", &request)?;
                Ok(x.session_id)
            }
        }
    }

    fn close_window(&self, id: &str) -> Result<(), Error> {
        #[derive(Deserialize)]
        struct Response {
        }
        let x: Response = self.delete(&format!("/session/{}/window", id))?;
        drop(x);
        Ok(())
    }

    fn goto(&self, id: &str, url: &str) -> Result<(), Error> {
        #[derive(Serialize)]
        struct Request {
            url: String,
        }
        #[derive(Deserialize)]
        struct Response {
        }

        let request = Request {
            url: url.to_string(),
        };
        let x: Response = self.post(&format!("/session/{}/url", id), &request)?;
        drop(x);
        Ok(())
    }

    fn element(&self, id: &str, selector: &str) -> Result<String, Error> {
        #[derive(Serialize)]
        struct Request {
            using: String,
            value: String,
        }
        #[derive(Deserialize)]
        struct Response {
            value: Reference,
        }
        #[derive(Deserialize)]
        struct Reference {
            #[serde(rename = "element-6066-11e4-a52e-4f735466cecf")]
            gecko_reference: Option<String>,
            #[serde(rename = "ELEMENT")]
            safari_reference: Option<String>,
        }

        let request = Request {
            using: "css selector".to_string(),
            value: selector.to_string(),
        };
        let x: Response = self.post(&format!("/session/{}/element", id), &request)?;
        Ok(x.value.gecko_reference
           .or(x.value.safari_reference)
           .ok_or(format_err!("failed to find element reference in response"))?)

    }

    fn text(&self, id: &str, element: &str) -> Result<String, Error> {
        #[derive(Deserialize)]
        struct Response {
            value: String,
        }
        let x: Response = self.get(&format!("/session/{}/element/{}/text", id, element))?;
        Ok(x.value)
    }

    fn get<U>(&self, path: &str) -> Result<U, Error>
        where U: for<'a> Deserialize<'a>,
    {
        debug!("GET {}", path);
        let result = self.doit(path, Method::Get)?;
        Ok(serde_json::from_str(&result)?)
    }

    fn post<T, U>(&self, path: &str, data: &T) -> Result<U, Error>
        where T: Serialize,
              U: for<'a> Deserialize<'a>,
    {
        let input = serde_json::to_string(data)?;
        debug!("POST {} {}", path, input);
        let result = self.doit(path, Method::Post(&input))?;
        Ok(serde_json::from_str(&result)?)
    }

    fn delete<U>(&self, path: &str) -> Result<U, Error>
        where U: for<'a> Deserialize<'a>,
    {
        debug!("DELETE {}", path);
        let result = self.doit(path, Method::Delete)?;
        Ok(serde_json::from_str(&result)?)
    }

    fn doit(&self, path: &str, method: Method) -> Result<String, Error> {
        let url = format!("http://{}{}", self.driver_addr, path);
        let mut handle = self.handle.borrow_mut();
        handle.reset();
        handle.url(&url)?;
        match method {
            Method::Post(data) => {
                handle.post(true)?;
                handle.post_fields_copy(data.as_bytes())?;
            }
            Method::Delete => handle.custom_request("DELETE")?,
            Method::Get => handle.get(true)?,
        }
        let mut result = Vec::new();
        {
            let mut t = handle.transfer();
            t.write_function(|buf| {
                result.extend_from_slice(buf);
                Ok(buf.len())
            })?;
            t.perform()?
        }
        let result = String::from_utf8_lossy(&result);
        if handle.response_code()? != 200 {
            bail!("non-200 response code: {}\n{}", handle.response_code()?, result);
        }
        debug!("got: {}", result);
        Ok(result.into_owned())
    }
}

fn read<R: Read>(r: &mut R) -> io::Result<Vec<u8>> {
    let mut dst = Vec::new();
    r.read_to_end(&mut dst)?;
    Ok(dst)
}

fn tab(s: &str) -> String {
    let mut result = String::new();
    for line in s.lines() {
        result.push_str("    ");
        result.push_str(line);
        result.push_str("\n");
    }
    return result;
}

struct OnDrop<F: FnMut()>(F);

impl<F: FnMut()> Drop for OnDrop<F> {
    fn drop(&mut self) {
        (self.0)();
    }
}
