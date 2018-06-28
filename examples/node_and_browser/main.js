const wasm = require('./node_and_browser.js');
const fs = require('fs');
async function main() {
    if (process.argv.length != 4) {
        return printHelp();
    }
    let md = await readMarkdown(process.argv[2]);
    let html = parseMarkdown(md);
    await writeHTML(html, process.argv[3]);
}

function printHelp() {
    console.log(`\
USAGE:
node ./main.js <inpath> <outpath>

ARGUMENTS:
inpath    this is the path to your
          input markdown file
outpath   this is the path where you
          would like the HTML file to be written`);
}

function readMarkdown(path) {
    return new Promise((resolve, reject) => {
        fs.readFile(path, (err, content) => {
            if (err) return reject(err);
            if (typeof content !== 'string') {
                content = content.toString();
            }
            resolve(content);
        })
    });
}

function parseMarkdown(md) {
    return wasm.parse_markdown(md);
}

function writeHTML(html, path) {
    return new Promise((resolve, reject) => {
        fs.writeFile(path, html, err => {
            if (err) return reject(err);
            resolve();
        });
    });
}

main();