use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    MessageEvent, RtcDataChannelEvent, RtcPeerConnection, RtcPeerConnectionIceEvent, RtcSdpType,
    RtcSessionDescriptionInit,
};

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
macro_rules! console_warn {
    ($($t:tt)*) => (warn(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn warn(s: &str);
}

#[wasm_bindgen(start)]
async fn start() -> Result<(), JsValue> {
    /*
     * Set up PeerConnections
     * pc1 <=> pc2
     *
     */
    let pc1 = RtcPeerConnection::new()?;
    console_log!("pc1 created: state {:?}", pc1.signaling_state());
    let pc2 = RtcPeerConnection::new()?;
    console_log!("pc2 created: state {:?}", pc2.signaling_state());

    /*
     * Create DataChannel on pc1 to negotiate
     * Message will be shown here after connection established
     *
     */
    let dc1 = pc1.create_data_channel("my-data-channel");
    console_log!("dc1 created: label {:?}", dc1.label());

    let dc1_clone = dc1.clone();
    let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |ev: MessageEvent| {
        if let Some(message) = ev.data().as_string() {
            console_warn!("{:?}", message);
            dc1_clone.send_with_str("Pong from pc1.dc!").unwrap();
        }
    });
    dc1.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();

    /*
     * If negotiation has done, this closure will be called
     *
     */
    let ondatachannel_callback = Closure::<dyn FnMut(_)>::new(move |ev: RtcDataChannelEvent| {
        let dc2 = ev.channel();
        console_log!("pc2.ondatachannel!: {:?}", dc2.label());

        let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |ev: MessageEvent| {
            if let Some(message) = ev.data().as_string() {
                console_warn!("{:?}", message);
            }
        });
        dc2.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        let dc2_clone = dc2.clone();
        let onopen_callback = Closure::<dyn FnMut()>::new(move || {
            dc2_clone.send_with_str("Ping from pc2.dc!").unwrap();
        });
        dc2.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
    });
    pc2.set_ondatachannel(Some(ondatachannel_callback.as_ref().unchecked_ref()));
    ondatachannel_callback.forget();

    /*
     * Handle ICE candidate each other
     *
     */
    let pc2_clone = pc2.clone();
    let onicecandidate_callback1 =
        Closure::<dyn FnMut(_)>::new(move |ev: RtcPeerConnectionIceEvent| {
            if let Some(candidate) = ev.candidate() {
                console_log!("pc1.onicecandidate: {:#?}", candidate.candidate());
                let _ = pc2_clone.add_ice_candidate_with_opt_rtc_ice_candidate(Some(&candidate));
            }
        });
    pc1.set_onicecandidate(Some(onicecandidate_callback1.as_ref().unchecked_ref()));
    onicecandidate_callback1.forget();

    let pc1_clone = pc1.clone();
    let onicecandidate_callback2 =
        Closure::<dyn FnMut(_)>::new(move |ev: RtcPeerConnectionIceEvent| {
            if let Some(candidate) = ev.candidate() {
                console_log!("pc2.onicecandidate: {:#?}", candidate.candidate());
                let _ = pc1_clone.add_ice_candidate_with_opt_rtc_ice_candidate(Some(&candidate));
            }
        });
    pc2.set_onicecandidate(Some(onicecandidate_callback2.as_ref().unchecked_ref()));
    onicecandidate_callback2.forget();

    /*
     * Send OFFER from pc1 to pc2
     *
     */
    let offer = JsFuture::from(pc1.create_offer()).await?;
    let offer_sdp = Reflect::get(&offer, &JsValue::from_str("sdp"))?
        .as_string()
        .unwrap();
    console_log!("pc1: offer {:?}", offer_sdp);

    let offer_obj = RtcSessionDescriptionInit::new(RtcSdpType::Offer);
    offer_obj.set_sdp(&offer_sdp);
    let sld_promise = pc1.set_local_description(&offer_obj);
    JsFuture::from(sld_promise).await?;
    console_log!("pc1: state {:?}", pc1.signaling_state());

    /*
     * Receive OFFER from pc1
     * Create and send ANSWER from pc2 to pc1
     *
     */
    let offer_obj = RtcSessionDescriptionInit::new(RtcSdpType::Offer);
    offer_obj.set_sdp(&offer_sdp);
    let srd_promise = pc2.set_remote_description(&offer_obj);
    JsFuture::from(srd_promise).await?;
    console_log!("pc2: state {:?}", pc2.signaling_state());

    let answer = JsFuture::from(pc2.create_answer()).await?;
    let answer_sdp = Reflect::get(&answer, &JsValue::from_str("sdp"))?
        .as_string()
        .unwrap();
    console_log!("pc2: answer {:?}", answer_sdp);

    let answer_obj = RtcSessionDescriptionInit::new(RtcSdpType::Answer);
    answer_obj.set_sdp(&answer_sdp);
    let sld_promise = pc2.set_local_description(&answer_obj);
    JsFuture::from(sld_promise).await?;
    console_log!("pc2: state {:?}", pc2.signaling_state());

    /*
     * Receive ANSWER from pc2
     *
     */
    let answer_obj = RtcSessionDescriptionInit::new(RtcSdpType::Answer);
    answer_obj.set_sdp(&answer_sdp);
    let srd_promise = pc1.set_remote_description(&answer_obj);
    JsFuture::from(srd_promise).await?;
    console_log!("pc1: state {:?}", pc1.signaling_state());

    Ok(())
}
