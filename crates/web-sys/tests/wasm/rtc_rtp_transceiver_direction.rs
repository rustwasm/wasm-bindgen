use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

use futures::{
    future::{ok, IntoFuture},
    Future,
};

use web_sys::{
    RtcPeerConnection, RtcRtpTransceiver, RtcRtpTransceiverDirection, RtcRtpTransceiverInit,
    RtcSessionDescriptionInit,
};

#[wasm_bindgen(
    inline_js = "export function is_unified_avail() { return Object.keys(RTCRtpTransceiver.prototype).indexOf('currentDirection')>-1; }"
)]
extern "C" {
    /// Available in FF since forever, in Chrome since 72, in Safari since 12.1
    fn is_unified_avail() -> bool;
}

#[wasm_bindgen_test(async)]
fn rtc_rtp_transceiver_direction() -> impl Future<Item = (), Error = JsValue> {
    if !is_unified_avail() {
        ok::<(), JsValue>(());
    }

    let mut tr_init: RtcRtpTransceiverInit = RtcRtpTransceiverInit::new();

    let pc1: RtcPeerConnection = RtcPeerConnection::new().unwrap();

    let tr1: RtcRtpTransceiver = pc1.add_transceiver_with_str_and_init(
        "audio",
        tr_init.direction(RtcRtpTransceiverDirection::Sendonly),
    );
    assert_eq!(tr1.direction(), RtcRtpTransceiverDirection::Sendonly);
    assert_eq!(tr1.current_direction(), None);

    let pc2: RtcPeerConnection = RtcPeerConnection::new().unwrap();

    exchange_sdps(pc1, pc2).and_then(move |(_, p2)| {
        assert_eq!(tr1.direction(), RtcRtpTransceiverDirection::Sendonly);
        assert_eq!(
            tr1.current_direction(),
            Some(RtcRtpTransceiverDirection::Sendonly)
        );

        let tr2: RtcRtpTransceiver = js_sys::try_iter(&p2.get_transceivers())
            .unwrap()
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
            .unchecked_into();

        assert_eq!(tr2.direction(), RtcRtpTransceiverDirection::Recvonly);
        assert_eq!(
            tr2.current_direction(),
            Some(RtcRtpTransceiverDirection::Recvonly)
        );

        Ok(())
    })
}

fn exchange_sdps(
    p1: RtcPeerConnection,
    p2: RtcPeerConnection,
) -> impl Future<Item = (RtcPeerConnection, RtcPeerConnection), Error = JsValue> {
    JsFuture::from(p1.create_offer())
        .and_then(move |offer| {
            let offer = offer.unchecked_into::<RtcSessionDescriptionInit>();
            JsFuture::from(p1.set_local_description(&offer)).join4(
                JsFuture::from(p2.set_remote_description(&offer)),
                Ok(p1),
                Ok(p2),
            )
        })
        .and_then(|(_, _, p1, p2)| JsFuture::from(p2.create_answer()).join3(Ok(p1), Ok(p2)))
        .and_then(|(answer, p1, p2)| {
            let answer = answer.unchecked_into::<RtcSessionDescriptionInit>();
            JsFuture::from(p2.set_local_description(&answer)).join4(
                JsFuture::from(p1.set_remote_description(&answer)),
                Ok(p1),
                Ok(p2),
            )
        })
        .and_then(|(_, _, p1, p2)| Ok((p1, p2)))
}
