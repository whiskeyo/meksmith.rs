# meksmith.rs

<p align="center">
  <img src="./website/assets/images/logo.svg" height="700">
</p>

`meksmith` is a tool composed from _meklang_ **DSL** (_domain-specific language_) and **code generator** designed to reduce the efforts of implementing applications using different languages for communicating over wire in a binary format (such as O-RAN FH CUS, eCPRI, and many others). It lets you define a protocol using the meklang DSL and then generates the code for a language of your choice, including builders of messages.

The grammar of the language is pretty simple and looks quite similar to what C and C++ offer, but includes a few extra features making your protocols easier to define. A few examples that you can run are in `examples` directory and each of them can be run with `cargo run --example <example_name>`.
