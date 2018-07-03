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

const driver = new Builder()
  .forBrowser("firefox")
  .setFirefoxOptions(opts)
  .build();

async function logged(msg, promise) {
  console.log("START:", msg);
  try {
    const value = await promise;
    console.log("END:", msg);
    return value;
  } catch (e) {
    console.log(`ERROR: ${msg}: ${e}\n\n${e.stack}`);
    throw e;
  }
}

const SECONDS = 1000;
const MINUTES = 60 * SECONDS;

async function main() {
  const body = driver.findElement(By.tagName("body"));
  try {
    await logged(
      "load http://localhost:8080/index.html",
      driver.get("http://localhost:8080/index.html")
    );

    await logged(
      "Waiting for <body> to include text 'TESTDONE'",
      driver.wait(
        until.elementTextContains(body, "TESTDONE"),
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
  }
}

main()
  .finally(() => driver.quit())
  .catch(e => {
    console.error(`Got an error: ${e}\n\nStack: ${e.stack}`);
    process.exit(1);
  });
