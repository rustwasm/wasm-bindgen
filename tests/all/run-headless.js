const process = require('process');
const { promisify } = require('util');
const { Builder, By, Key, logging, promise, until } = require('selenium-webdriver');
const firefox = require('selenium-webdriver/firefox');

promise.USE_PROMISE_MANAGER = false;

const prefs = new logging.Preferences();
prefs.setLevel(logging.Type.BROWSER, logging.Level.DEBUG);

const opts = new firefox.Options();
opts.headless();
if (process.env.WASM_BINDGEN_FIREFOX_BIN_PATH) {
  opts.setBinary(process.env.WASM_BINDGEN_FIREFOX_BIN_PATH);
}

const driver = new Builder()
  .forBrowser('firefox')
  .setFirefoxOptions(opts)
  .build();

async function main() {
  const body = driver.findElement(By.tagName('body'));
  try {
    await driver.get('http://localhost:8080/index.html');
    await driver.wait(
      until.elementTextContains(body, 'TESTDONE'),
      6 * 1000
    );

    const status = await body.findElement(By.id('status')).getText();
    if (status != 'good')
      throw new Error('test failed');
  } finally {
    const logs = await body.findElement(By.id('logs')).getText();
    if (logs.length > 0) {
      console.log('logs:');
      logs.split("\n").forEach(line => {
        console.log(`    ${line}`);
      });
    }

    const errors = await body.findElement(By.id('error')).getText();
    if (errors.length > 0) {
      console.log('errors:');
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
