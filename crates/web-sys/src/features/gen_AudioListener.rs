use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = AudioListener , typescript_name = AudioListener ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioListener` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener)\n\n*This API requires the following crate features to be activated: `AudioListener`*"]
    pub type AudioListener;
    # [ wasm_bindgen ( structural , method , getter , js_name = dopplerFactor ) ]
    #[doc = "Getter for the `dopplerFactor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/dopplerFactor)\n\n*This API requires the following crate features to be activated: `AudioListener`*"]
    pub fn doppler_factor(this: &AudioListener) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = dopplerFactor ) ]
    #[doc = "Setter for the `dopplerFactor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/dopplerFactor)\n\n*This API requires the following crate features to be activated: `AudioListener`*"]
    pub fn set_doppler_factor(this: &AudioListener, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = speedOfSound ) ]
    #[doc = "Getter for the `speedOfSound` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/speedOfSound)\n\n*This API requires the following crate features to be activated: `AudioListener`*"]
    pub fn speed_of_sound(this: &AudioListener) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = speedOfSound ) ]
    #[doc = "Setter for the `speedOfSound` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/speedOfSound)\n\n*This API requires the following crate features to be activated: `AudioListener`*"]
    pub fn set_speed_of_sound(this: &AudioListener, value: f64);
    # [ wasm_bindgen ( method , structural , js_name = setOrientation ) ]
    #[doc = "The `setOrientation()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/setOrientation)\n\n*This API requires the following crate features to be activated: `AudioListener`*"]
    pub fn set_orientation(
        this: &AudioListener,
        x: f64,
        y: f64,
        z: f64,
        x_up: f64,
        y_up: f64,
        z_up: f64,
    );
    # [ wasm_bindgen ( method , structural , js_name = setPosition ) ]
    #[doc = "The `setPosition()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/setPosition)\n\n*This API requires the following crate features to be activated: `AudioListener`*"]
    pub fn set_position(this: &AudioListener, x: f64, y: f64, z: f64);
    # [ wasm_bindgen ( method , structural , js_name = setVelocity ) ]
    #[doc = "The `setVelocity()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/setVelocity)\n\n*This API requires the following crate features to be activated: `AudioListener`*"]
    pub fn set_velocity(this: &AudioListener, x: f64, y: f64, z: f64);
}
