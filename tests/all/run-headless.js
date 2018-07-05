const process = require("process");
const { promisify } = require("util");
const { Builder, By, Key, logging, promise, until } = require("selenium-webdriver");
const firefox = require("selenium-webdriver/firefox");

promise.USE_PROMISE_MANAGER = false;

const prefs = new logging.Preferences();
prefs.setLevel(logging.Type.BROWSER, logging.Level.DEBUG);

const opts = new firefox.Options();
opts.headless();
if (process.env.WASM_BINDGEN_FIREFOX_BIN_PATH) {
  console.log("Using custom firefox-bin: $WASM_BINDGEN_FIREFOX_BIN_PATH =",
              process.env.WASM_BINDGEN_FIREFOX_BIN_PATH);
  opts.setBinary(process.env.WASM_BINDGEN_FIREFOX_BIN_PATH);
}

console.log("Using Firefox options:", opts);

const driver = new Builder()
  .forBrowser("firefox")
  .setFirefoxOptions(opts)
  .build();

const SECONDS = 1000;
const MINUTES = 60 * SECONDS;

const start = Date.now();
const timeSinceStart = () => {
  const elapsed = Date.now() - start;
  const minutes = Math.floor(elapsed / MINUTES);
  const seconds = elapsed % MINUTES / SECONDS;
  return `${minutes}m${seconds.toFixed(3)}s`;
};

async function logged(msg, promise) {
  console.log(`${timeSinceStart()}: START: ${msg}`);
  try {
    const value = await promise;
    console.log(`${timeSinceStart()}: END: ${msg}`);
    return value;
  } catch (e) {
    console.log(`${timeSinceStart()}: ERROR: ${msg}: ${e}\n\n${e.stack}`);
    throw e;
  }
}

async function main() {
  let body;
  try {
    await logged(
      "load http://localhost:8080/index.html",
      driver.get("http://localhost:8080/index.html")
    );

    body = driver.findElement(By.tagName("body"));

    await logged(
      "Waiting for <body> to include text 'TEST_START'",
      driver.wait(
        until.elementTextContains(body, "TEST_START"),
        1 * MINUTES
      )
    );

    await logged(
      "Waiting for <body> to include text 'TEST_DONE'",
      driver.wait(
        until.elementTextContains(body, "TEST_DONE"),
        1 * MINUTES
      )
    );

    const status = await logged(
      "get #status text",
      body.findElement(By.id("status")).getText()
    );

    console.log(`Test status is: "${status}"`);
    if (status != "good") {
      throw new Error(`test failed with status = ${status}`);
    }
  } finally {
    const logs = await logged(
      "getting browser logs",
      body.findElement(By.id("logs")).getText()
    );

    if (logs.length > 0) {
      console.log("logs:");
      logs.split("\n").forEach(line => {
        console.log(`    ${line}`);
      });
    }

    const errors = await logged(
      "getting browser errors",
      body.findElement(By.id("error")).getText()
    );

    if (errors.length > 0) {
      console.log("errors:");
      errors.split("\n").forEach(line => {
        console.log(`    ${line}`);
      });
    }

    const bodyText = await logged(
      "getting browser body",
      body.getText()
    );

    if (bodyText.length > 0) {
      console.log("body:");
      bodyText.split("\n").forEach(line => {
        console.log(`    ${line}`);
      });
    }
  }
}

main()
  .finally(() => driver.quit())
  .catch(e => {
    console.error(`Got an error: ${e}\n\nStack: ${e.stack}`);
    process.exit(1);
  });
