use wasm_bindgen::prelude::*;
use web_sys::{WebSocket};
use js_sys::{Function};


#[wasm_bindgen]
pub struct WebSocketFactory {
    ws: WebSocket,
}

#[wasm_bindgen]
impl WebSocketFactory {
    #[wasm_bindgen(constructor)]
    pub fn new(ws_server_url: &str) -> WebSocketFactory {
        WebSocketFactory {
            ws: WebSocket::new(ws_server_url).expect("Failed to connect!"),
        }
    }

    pub fn init(&self, callback: &Function) {
        self.ws.set_onmessage(Some(callback));
    }

    pub fn send(&self) {
        self.ws.send_with_str("ping").expect("Failed to send!");
    }

    pub fn close(&self) {
        self.ws.close().expect("Failed to close!");
    }
}
