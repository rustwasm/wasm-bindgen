use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = AudioListener , typescript_type = "AudioListener" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `AudioListener` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener)
    ///
    ///*This API requires the following crate features to be activated: `AudioListener`*
    pub type AudioListener;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioListener" , js_name = dopplerFactor ) ]
    ///Getter for the `dopplerFactor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/dopplerFactor)
    ///
    ///*This API requires the following crate features to be activated: `AudioListener`*
    pub fn doppler_factor(this: &AudioListener) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "AudioListener" , js_name = dopplerFactor ) ]
    ///Setter for the `dopplerFactor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/dopplerFactor)
    ///
    ///*This API requires the following crate features to be activated: `AudioListener`*
    pub fn set_doppler_factor(this: &AudioListener, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioListener" , js_name = speedOfSound ) ]
    ///Getter for the `speedOfSound` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/speedOfSound)
    ///
    ///*This API requires the following crate features to be activated: `AudioListener`*
    pub fn speed_of_sound(this: &AudioListener) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "AudioListener" , js_name = speedOfSound ) ]
    ///Setter for the `speedOfSound` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/speedOfSound)
    ///
    ///*This API requires the following crate features to be activated: `AudioListener`*
    pub fn set_speed_of_sound(this: &AudioListener, value: f64);

    # [ wasm_bindgen ( method , structural , js_class = "AudioListener" , js_name = setOrientation ) ]
    ///The `setOrientation()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/setOrientation)
    ///
    ///*This API requires the following crate features to be activated: `AudioListener`*
    pub fn set_orientation(
        this: &AudioListener,
        x: f64,
        y: f64,
        z: f64,
        x_up: f64,
        y_up: f64,
        z_up: f64,
    );

    # [ wasm_bindgen ( method , structural , js_class = "AudioListener" , js_name = setPosition ) ]
    ///The `setPosition()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/setPosition)
    ///
    ///*This API requires the following crate features to be activated: `AudioListener`*
    pub fn set_position(this: &AudioListener, x: f64, y: f64, z: f64);

    # [ wasm_bindgen ( method , structural , js_class = "AudioListener" , js_name = setVelocity ) ]
    ///The `setVelocity()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/setVelocity)
    ///
    ///*This API requires the following crate features to be activated: `AudioListener`*
    pub fn set_velocity(this: &AudioListener, x: f64, y: f64, z: f64);

}
