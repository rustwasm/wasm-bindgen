use std::collections::VecDeque;
use std::fmt::{self, Display, Formatter, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use std::{env, str};

use anyhow::{bail, Context};
use futures_util::{future, SinkExt, StreamExt};
use mozprofile::profile::Profile;
use mozrunner::firefox_default_path;
use mozrunner::runner::{FirefoxProcess, FirefoxRunner, Runner, RunnerProcess};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::net::TcpStream;
use tokio::sync::oneshot;
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
                    if start.elapsed() > Duration::from_secs(20) {
                        return Err(e).context("failed to connect to Firefox (after 20s)");
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

/// Run a single example with the passed name, using the passed closure to
/// build it if prebuilt examples weren't provided.
pub async fn test_example(
    name: &str,
    build: impl FnOnce() -> anyhow::Result<PathBuf>,
) -> anyhow::Result<()> {
    let path = if let Some(value) = env::var_os("EXBUILD") {
        Path::new(&value).join(name)
    } else {
        build()?
    };

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

pub fn run(command: &mut Command) -> anyhow::Result<()> {
    // Format the command to use in errors.
    let mut cmdline = command.get_program().to_string_lossy().to_string();
    for arg in command.get_args().map(|arg| arg.to_string_lossy()) {
        cmdline += " ";
        cmdline += &arg;
    }

    let status = command.status()?;
    if !status.success() {
        bail!("`{cmdline}` failed with {status}");
    }
    Ok(())
}

/// Returns the path of root `wasm-bindgen` folder.
pub fn manifest_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
}

/// Returns the path of the example with the passed name.
pub fn example_dir(name: &str) -> PathBuf {
    [manifest_dir(), "examples".as_ref(), name.as_ref()]
        .iter()
        .collect()
}
