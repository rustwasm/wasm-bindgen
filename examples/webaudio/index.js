const rust = import('./webaudio');


// Most browsers don't let WebAudio autoplay without some interaction from the user.  So once the module is loaded,
// it's passed to this function which will set up the UI elements for the user to interact with
function setup(rust_module) {
    play = function() {
            console.log("About to create some music!");
            fm = new rust_module.FmOsc();

            fm.set_note(50);
            fm.set_fm_frequency(0);
            fm.set_fm_amount(0);
            fm.set_gain(0.8);

    };

    // create some UI elements
    const primary_slider = document.getElementById("primary_input");
    primary_slider.oninput = (e) => {
        fm.set_note(e.target.value);
    };

    const fm_freq = document.getElementById("fm_freq");
    fm_freq.oninput = (e) => {
        fm.set_fm_frequency(e.target.value);
    };

    const fm_amount = document.getElementById("fm_amount");
    fm_amount.oninput = (e) => {
        fm.set_fm_amount(e.target.value);
    };

    console.log("Ready!  Press the play button!");
}


rust.then(m => {
    setup(m);
});
