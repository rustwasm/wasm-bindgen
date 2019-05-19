import init from './pkg/websockets.js';

window.addEventListener('load', async () => {
    await init('./pkg/websockets_bg.wasm');
});
