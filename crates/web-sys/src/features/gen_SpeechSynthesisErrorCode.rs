use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `SpeechSynthesisErrorCode` enum.\n\n*This API requires the following crate features to be activated: `SpeechSynthesisErrorCode`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SpeechSynthesisErrorCode {
    Canceled = "canceled",
    Interrupted = "interrupted",
    AudioBusy = "audio-busy",
    AudioHardware = "audio-hardware",
    Network = "network",
    SynthesisUnavailable = "synthesis-unavailable",
    SynthesisFailed = "synthesis-failed",
    LanguageUnavailable = "language-unavailable",
    VoiceUnavailable = "voice-unavailable",
    TextTooLong = "text-too-long",
    InvalidArgument = "invalid-argument",
}
