import init from './pkg/webrtc_datachannel.js';

window.addEventListener('load', async () => {
    await init('./pkg/webrtc_datachannel_bg.wasm');
});
