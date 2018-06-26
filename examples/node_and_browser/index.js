let wasm;
import('./node_and_browser.js').then(w => {
    console.log('wasm_cmark_parse resolved');
    wasm = w;
    ready();
});

function parseMD(md) {
    console.log('parseMD');
    return wasm.parse_markdown(md);
}

let debounce;
function ready() {
    console.log('ready');
    renderMD();
    let input = getInput();
    if (!input) throw new Error('Unable to find MD input');
    input.addEventListener('keyup', () => {
        if (!debounce) {
            debounce = setTimeout(renderMD, 2000);
        } else {
            clearTimeout(debounce);
            debounce = setTimeout(renderMD, 2000);
        }
    });
}
function renderMD() {
    console.log('renderMD');
    debounce = null;
    let input = getInput();
    if (!input) throw new Error('Unable to find MD input');
    let md = input.value;
    let html = parseMD(md);
    let target = getHTML();
    if (!target) throw new Error('Unable to find div to render HTML');
    target.innerHTML = html;
}

function getInput() {
    console.log('getInput');
    return document.getElementById('md-input-box');
}

function getHTML() {
    console.log('getHTML');
    return document.getElementById('rendered');
}