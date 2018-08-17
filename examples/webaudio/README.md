# Web Audio example

This directory is an example of how to use the Web Audio APIs from Rust.  It creates a very simple
FM (frequency modulation) synth, and let's you control the primary frequency, the modulation amount,
and the modulation frequency.

To run, first install some utilities via npm:

 > npm install
 
 Then build the project with either `build.bat` or `build.sh`.
 
 Finally, run a development web server with `npm run serve` and then open
 [http://localhost:8080/](http://localhost:8080/) in a browser! 