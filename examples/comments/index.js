const mod = import('./comments');
let wasm;
mod.then(m => {
    wasm = m;
    let button = document.getElementById('add-comment-button');
    if (!button) return console.error('Unable to find add button');
    button.addEventListener('click', newComment);
    displayComments();
});

/**
 * Click event handler for add button
 * @param {MouseEvent} ev 
 */
function newComment(ev) {
    clearError();
    let name = document.getElementById('name');
    if (!name) return console.error('Failed to find name input');
    if (name.value == '') return displayError('Name cannot be blank');
    let comment = document.getElementById('comment');
    if (!comment) return console.error('Failed to find comment input');
    if (comment.value == '') return displayError('comment cannot be blank');
    addComment(name.value, comment.value);
    name.form.reset();
    displayComments();
}

/**
 * Add a comment to the list
 * @param {string} name The name of the person submitting
 * @param {string} content The actual text of the comment
 */
function addComment(name, content) {
    let existing = comments();
    let count = existing.length;
    existing.push(new wasm.Comment(name, content, count));
    storeComments();
}

/**
 * Convert the rust comments to JSON comments and store
 * in local storage
 */
function storeComments() {
    let json = comments().map(c => {
        console.log('mapping comments for storage', c);
        return {
            name: c.name(),
            comment: c.comment(),
            count: c.count,
        }
    });
    localStorage.setItem('comments', JSON.stringify(json));
}
/**
 * Get the comments from local storage and convert them to 
 * rust comments
 */
function getComments() {
    let json = localStorage.getItem('comments');
    if (!json) return [];
    let raw = JSON.parse(json);
    return raw.map(c => new wasm.Comment(c.name, c.comment, c.count));
}

/**A in memory cache of the localStorage comments for this site */
let cachedComments = null;
/** 
 * Get a list of comments for this page
 * @param {boolean} refresh force a refresh from localStorage
 */
function comments(refresh = false) {
    if (refresh || !cachedComments) {
        cachedComments = getComments();
    }
    return cachedComments;
}

/**
 * Clear the comments section and re-render with the
 * current comments list
 */
function displayComments() {
    let node = document.getElementById('comments');
    if (!node) return console.error('Failed to get comments container');
    while (node.hasChildNodes()) {
        node.removeChild(node.lastChild);
    }
    for (let comment of comments()) {
        node.appendChild(renderComment(comment));
    }
}

/**
 * Generate the HTML needed to display a single comment
 * @param {Comment} comment the comment to display
 * @returns {HTMLDivElement} The div containing the comment html
 */
function renderComment(comment) {
    let cls = `comment ${comment.color() == wasm.Color.Blue ? 'blue' : 'pink'}`;
    let div = document.createElement('div');
    div.setAttribute('class', cls);
    let top = document.createElement('div');
    top.setAttribute('class', 'comment-top');
    let ct = document.createElement('span');
    ct.setAttribute('class', 'count');
    ct.appendChild(document.createTextNode(`${comment.count}:`));
    let name = document.createElement('span');
    name.setAttribute('class', 'user-name');
    name.appendChild(document.createTextNode(`${comment.name()}`));
    top.appendChild(ct);
    top.appendChild(name);
    let bottom = document.createElement('div');
    bottom.setAttribute('class', 'comment-bottom');
    let title = document.createElement('span');
    title.setAttribute('class', 'comment-title');
    title.appendChild(document.createTextNode('comment'));
    bottom.appendChild(title);
    let text = document.createElement('span');
    text.setAttribute('class', 'comment-text');
    text.appendChild(document.createTextNode(comment.comment()))
    bottom.appendChild(text);
    div.appendChild(top);
    div.appendChild(bottom)
    return div;
}

function displayError(message) {
    let e = document.getElementById('error');
    if (!e) return console.error('Failed to find error container');
    if (e.innerHTML != '') return setTimeout(displayError, 1000, message);
    e.innerHTML = message;
    setTimeout(clearError, 3000);
}

function clearError() {
    let e = document.getElementById('error');
    if (!e) return console.error('Failed to find error container');
    e.innerHTML = '';
}