//! A test that none of our examples are broken, by opening them in a browser
//! and checking that no errors get logged to the console.
//!
//! This currently only attempts to use Firefox.

use std::collections::VecDeque;
use std::fmt::{self, Display, Formatter, Write};
use std::fs;
use std::io::ErrorKind;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use std::{env, io, str};

use anyhow::{bail, Context};
use futures_util::{future, SinkExt, StreamExt};
use mozprofile::profile::Profile;
use mozrunner::firefox_default_path;
use mozrunner::runner::{FirefoxProcess, FirefoxRunner, Runner, RunnerProcess};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::net::TcpStream;
use tokio::sync::{oneshot, Mutex};
use tokio::time::timeout;
use tokio_tungstenite::tungstenite::{self, Message};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tower::make::Shared;
use tower_http::services::ServeDir;

/// A command sent from the client to the server.
#[derive(Serialize)]
struct BidiCommand<'a, T> {
    id: u64,
    method: &'a str,
    params: T,
}

/// A message sent from the server to the client.
#[derive(Deserialize)]
#[serde(untagged)]
enum BidiMessage<R> {
    CommandResponse {
        id: u64,
        #[serde(flatten)]
        payload: CommandResult<R>,
    },
    Event(Event),
}

#[derive(Deserialize)]
#[serde(untagged)]
enum CommandResult<R> {
    Ok { result: R },
    Err(CommandError),
}

impl<R> From<CommandResult<R>> for Result<R, CommandError> {
    fn from(res: CommandResult<R>) -> Self {
        match res {
            CommandResult::Ok { result } => Ok(result),
            CommandResult::Err(e) => Err(e),
        }
    }
}

/// An error that occured while running a command.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct CommandError {
    /// The kind of error that occurred.
    error: BidiErrorKind,
    /// The message associated with the error.
    message: String,
    /// The stack trace associated with the error, if any.
    stacktrace: Option<String>,
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.error, self.message)?;
        if f.alternate() {
            // Show the stack trace.
            if let Some(stacktrace) = &self.stacktrace {
                write!(f, "\n\nStack trace:\n{stacktrace}")?;
            }
        }
        Ok(())
    }
}

impl std::error::Error for CommandError {}

/// A kind of error that can occur while running a command.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
enum BidiErrorKind {
    #[serde(rename = "unknown command")]
    /// An unknown command was issued.
    UnknownCommand,
    /// An invalid argument was passed for a command.
    #[serde(rename = "invalid argument")]
    InvalidArgument,
    /// Some other kind of error occured.
    #[serde(rename = "unknown error")]
    UnknownError,
}

impl Display for BidiErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BidiErrorKind::UnknownCommand => f.pad("unknown command"),
            BidiErrorKind::InvalidArgument => f.pad("invalid argument"),
            BidiErrorKind::UnknownError => f.pad("unknown error"),
        }
    }
}

/// An event sent from the server to the client.
#[derive(Deserialize)]
pub struct Event {
    /// The name of the event.
    method: String,
    /// The payload of the event.
    params: Value,
}

/// A connection to a WebDriver BiDi session.
struct WebDriver {
    /// The WebSocket we're connected to the WebDriver implementation with.
    ws: WebSocketStream<MaybeTlsStream<TcpStream>>,
    /// The WebDriver process.
    process: FirefoxProcess,
    /// The ID that will be used for the next command.
    next_id: u64,
    /// Unyielded events.
    events: VecDeque<Event>,
}

impl Drop for WebDriver {
    fn drop(&mut self) {
        self.process.kill().unwrap();
    }
}

impl WebDriver {
    async fn new() -> anyhow::Result<Self> {
        // Make the OS assign us a random port by asking for port 0.
        let driver_addr = TcpListener::bind("127.0.0.1:0")?.local_addr()?;

        // For the moment, we're only supporting Firefox here.
        let mut builder = FirefoxRunner::new(
            &firefox_default_path().context("failed to find Firefox installation")?,
            Some(Profile::new()?),
        );
        builder
            .arg("--remote-debugging-port")
            .arg(driver_addr.port().to_string())
            .arg("--headless")
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        let process = builder
            .start()
            // `mozprofile` doesn't guarantee that its errors are `Send + Sync`,
            // which means that they can't be converted to `anyhow::Error`.
            // So, convert them to strings as a workaround.
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        // Connect to the Firefox instance.
        let start = Instant::now();
        let ws = loop {
            match tokio_tungstenite::connect_async(format!("ws://{driver_addr}/session")).await {
                Ok((ws, _)) => break ws,
                Err(e) => {
                    if start.elapsed() > Duration::from_secs(5) {
                        return Err(e).context("failed to connect to Firefox (after 5s)");
                    }
                }
            }
        };

        let mut this = WebDriver {
            ws,
            process,
            next_id: 0,
            events: VecDeque::new(),
        };

        // Start the session.
        let _: Value = this
            .issue_cmd(
                "session.new",
                json!({ "capabilities": { "unhandledPromptBehavior": "dismiss" } }),
            )
            .await?;

        Ok(this)
    }

    async fn issue_cmd<T: Serialize, R: DeserializeOwned>(
        &mut self,
        method: &str,
        params: T,
    ) -> anyhow::Result<R> {
        let id = self.next_id;
        self.next_id += 1;
        let json = serde_json::to_string(&BidiCommand { id, method, params })
            .context("failed to serialize message")?;
        self.ws.send(Message::Text(json)).await?;
        loop {
            let msg = self
                .ws
                .next()
                .await
                .unwrap_or(Err(tungstenite::Error::AlreadyClosed))?;

            let message: BidiMessage<R> = serde_json::from_str(&msg.into_text()?)?;
            match message {
                BidiMessage::CommandResponse {
                    id: response_id,
                    payload,
                } => {
                    if response_id != id {
                        bail!("unexpected response to command {response_id} after sending command {id}")
                    }
                    return Result::from(payload).map_err(anyhow::Error::from);
                }
                BidiMessage::Event(event) => self.events.push_back(event),
            }
        }
    }

    async fn next_event(&mut self) -> anyhow::Result<Event> {
        if let Some(event) = self.events.pop_front() {
            Ok(event)
        } else {
            loop {
                let msg = self
                    .ws
                    .next()
                    .await
                    .unwrap_or(Err(tungstenite::Error::AlreadyClosed))?;

                let message: BidiMessage<Value> = serde_json::from_str(&msg.into_text()?)?;
                match message {
                    BidiMessage::CommandResponse { .. } => bail!("unexpected command response"),
                    BidiMessage::Event(event) => return Ok(event),
                }
            }
        }
    }
}

/// Run a single example example being served with the passed name at the passed
/// URL.
async fn test_example(path: &Path) -> anyhow::Result<()> {
    let mut driver = WebDriver::new().await?;

    // Serve the path.
    let server = hyper::Server::try_bind(&"127.0.0.1:0".parse().unwrap())?
        .serve(Shared::new(ServeDir::new(path)));

    let addr = server.local_addr();

    let (tx, rx) = oneshot::channel();

    let (server_result, result) = future::join(
        server.with_graceful_shutdown(async move {
            let _ = rx.await;
        }),
        async {
            #[derive(Deserialize)]
            struct BrowsingContextCreateResult {
                context: String,
            }

            let BrowsingContextCreateResult { context } = driver
                .issue_cmd("browsingContext.create", json!({ "type": "tab" }))
                .await?;

            let _: Value = driver
                .issue_cmd(
                    "session.subscribe",
                    json!({
                        "events": ["log.entryAdded"],
                        "contexts": [&context],
                    }),
                )
                .await?;

            let _: Value = driver
                .issue_cmd(
                    "browsingContext.navigate",
                    json!({
                        "context": &context,
                        "url": format!("http://{addr}"),
                    }),
                )
                .await?;

            let start = Instant::now();
            // Wait 5 seconds for any errors to occur.
            const WAIT_DURATION: Duration = Duration::from_secs(5);
            while start.elapsed() < WAIT_DURATION {
                match timeout(WAIT_DURATION - start.elapsed(), driver.next_event()).await {
                    Ok(event) => {
                        let event = event?;
                        if event.method == "log.entryAdded" {
                            #[derive(Deserialize)]
                            #[serde(rename_all = "camelCase")]
                            struct LogEntry {
                                level: LogLevel,
                                // source: Source,
                                text: Option<String>,
                                // timestamp: i64,
                                stack_trace: Option<StackTrace>,
                                // kind: LogEntryKind,
                            }

                            #[derive(Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
                            #[serde(rename_all = "lowercase")]
                            enum LogLevel {
                                Debug,
                                Info,
                                Warning,
                                Error,
                            }

                            #[derive(Deserialize)]
                            #[serde(rename_all = "camelCase")]
                            struct StackTrace {
                                call_frames: Vec<StackFrame>,
                            }

                            #[derive(Deserialize)]
                            #[serde(rename_all = "camelCase")]
                            struct StackFrame {
                                column_number: i64,
                                function_name: String,
                                line_number: i64,
                                url: String,
                            }

                            impl Display for StackFrame {
                                fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                                    write!(
                                        f,
                                        "{} (at {}:{}:{})",
                                        self.function_name,
                                        self.url,
                                        self.line_number,
                                        self.column_number
                                    )
                                }
                            }

                            let entry: LogEntry = serde_json::from_value(event.params)
                                .context("invalid log entry received")?;

                            if entry.level == LogLevel::Error {
                                if let Some(text) = entry.text {
                                    let mut msg = format!("An error occured: {text}");

                                    if let Some(stack_trace) = entry.stack_trace {
                                        write!(msg, "\n\nStack trace:").unwrap();
                                        for frame in stack_trace.call_frames {
                                            write!(msg, "\n{frame}").unwrap();
                                        }
                                    }

                                    bail!("{msg}")
                                } else {
                                    bail!("An error occured")
                                }
                            }
                        }
                    }
                    Err(_) => break,
                }
            }

            tx.send(()).unwrap();

            Ok(())
        },
    )
    .await;

    server_result.context("error running file server")?;

    result
}

fn run(command: &mut Command) -> anyhow::Result<()> {
    let status = command.status()?;
    if !status.success() {
        bail!("build failed")
    }
    Ok(())
}

async fn test_webpack_example(name: &str) -> anyhow::Result<()> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path: PathBuf = [manifest_dir, "examples".as_ref(), name.as_ref()]
        .iter()
        .copied()
        .collect();

    fn allow_already_exists(e: io::Error) -> io::Result<()> {
        if e.kind() == ErrorKind::AlreadyExists {
            Ok(())
        } else {
            Err(e)
        }
    }

    // All of the examples have the same dependencies, so we can just install
    // to the root `node_modules`, since Node resolves packages from any outer
    // directories as well as the one containing the `package.json`.

    static INSTALLED: Mutex<bool> = Mutex::const_new(false);

    // We lock this while installing so that by the time other threads read it
    // as `true`, the actual installation is done.
    let mut installed = INSTALLED.lock().await;

    if !*installed {
        fs::copy(
            manifest_dir.join("_package.json"),
            manifest_dir.join("package.json"),
        )
        .map(|_| ())
        .or_else(allow_already_exists)?;

        run(Command::new("npm").arg("install").current_dir(manifest_dir))?;

        fs::remove_file(manifest_dir.join("package.json"))?;

        *installed = true;
    }

    // release the lock
    drop(installed);

    // Build the example.
    run(Command::new("npm")
        .arg("run")
        .arg("build")
        .current_dir(&path))?;

    test_example(&path.join("dist")).await
}

macro_rules! webpack_tests {
    ($($test:ident = $name:literal,)*) => {
        $(
            #[tokio::test]
            async fn $test() {
                test_webpack_example($name).await.unwrap();
            }
        )*
    };
}

webpack_tests! {
    add = "add",
    canvas = "canvas",
    char = "char",
    closures = "closures",
    console_log = "console_log",
    dom = "dom",
    duck_typed_interfaces = "duck-typed-interfaces",
    fetch = "fetch",
    guide_supported_types_examples = "guide-supported-types-examples",
    hello_world = "hello_world",
    import_js = "import_js",
    julia_set = "julia_set",
    paint = "paint",
    performance = "performance",
    request_animation_frame = "request-animation-frame",
    todomvc = "todomvc",
    wasm_in_wasm_imports = "wasm-in-wasm-imports",
    wasm_in_wasm = "wasm-in-wasm",
    weather_report = "weather_report",
    webaudio = "webaudio",
    webgl = "webgl",
    webrtc_datachannel = "webrtc_datachannel",
    // WebXR isn't supported in Firefox yet
    // webxr = "webxr",
}

#[cfg(unix)]
async fn test_shell_example(name: &str) -> anyhow::Result<()> {
    let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "examples", name]
        .iter()
        .copied()
        .collect();
    run(Command::new(path.join("build.sh")).current_dir(&path))?;
    test_example(&path).await
}

#[cfg(unix)]
macro_rules! shell_tests {
    ($($test:ident = $name:literal,)*) => {
        $(
            #[tokio::test]
            async fn $test() {
                test_shell_example($name).await.unwrap();
            }
        )*
    };
}

// Since these run on shell scripts, they won't work outside Unix-based OSes.
#[cfg(unix)]
shell_tests! {
    // This requires module workers, which Firefox doesn't support yet.
    // synchronous_instantiation = "synchronous-instantiation", target = "web",
    wasm2js = "wasm2js",
    wasm_in_web_worker = "wasm-in-web-worker",
    websockets = "websockets",
    without_a_bundler = "without-a-bundler",
    without_a_bundler_no_modules = "without-a-bundler-no-modules",
}

#[cfg(unix)]
#[tokio::test]
async fn raytrace_parallel() {
    let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "examples", "raytrace-parallel"]
        .iter()
        .copied()
        .collect();

    run(Command::new(path.join("build.sh"))
        .current_dir(&path)
        // This requires nightly.
        .env("RUSTUP_TOOLCHAIN", "nightly"))
    .unwrap();

    test_example(&path).await.unwrap();
}
