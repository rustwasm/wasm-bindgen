const child_process = require('child_process');
const fs = require('fs');

function set_env(name, val) {
  fs.appendFileSync(process.env['GITHUB_ENV'], `${name}=${val}\n`)
}

function fetch(url) {
  child_process.execFileSync('curl', ['--retry', '5', '-LO', url])
}

const version = '0.24.0';

if (process.platform === 'win32') {
  const file = `geckodriver-v${version}-win64.zip`;
  fetch(`https://github.com/mozilla/geckodriver/releases/download/v${version}/${file}`);
  child_process.execFileSync('unzip', [file]);
  set_env("GECKODRIVER", process.cwd() + '/geckodriver.exe');
} else {
  const file = `geckodriver-v${version}-linux64.tar.gz`;
  fetch(`https://github.com/mozilla/geckodriver/releases/download/v${version}/${file}`);
  child_process.execFileSync('tar', ['xf', file]);
  set_env("GECKODRIVER", process.cwd() + '/geckodriver');
}
