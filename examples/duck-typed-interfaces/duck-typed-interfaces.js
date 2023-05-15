import { make_em_quack_to_this } from "./rust_duck_typed_interfaces";

// All of these objects implement the `Quacks` interface!

const alex = {
  quack: () => "you're not wrong..."
};

const ashley = {
  quack: () => "<corgi.gif>"
};

const nick = {
  quack: () => "rappers I monkey-flip em with the funky rhythm I be kickin"
};

// Get all our ducks in a row and call into wasm!

make_em_quack_to_this(alex);
make_em_quack_to_this(ashley);
make_em_quack_to_this(nick);
