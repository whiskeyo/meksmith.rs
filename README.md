# meksmith.rs

<p align="center">
  <img src="./website/assets/images/logo.svg" height="700">
</p>

`meksmith` is a tool composed from _meklang_ **DSL** (_domain-specific language_) and **code generator** designed to reduce the efforts of implementing applications using different languages for communicating over wire in a binary format (such as O-RAN FH CUS, eCPRI, and many others). It lets you define a protocol using the meklang DSL and then generates the code for a language of your choice, including builders of messages. All of that can be also done on the [`meksmith.rs` website](https://meksmith.rs).

The grammar of the language is pretty simple and looks quite similar to what C and C++ offer, but includes a few extra features making your protocols easier to define. A few examples that you can run are in `examples` directory and each of them can be run with `cargo run --example <example_name>`.

## Working on the project

### Prerequisities

To avoid commiting not working code and polluting CI with surely-not-passing builds, make sure to call:

```bash
cp devops/hooks/pre-commit .git/hooks
```

This will check the code automatically with `cargo check`, `cargo clippy`, `cargo fmt` and `cargo test` (all of these with some extra options) before commiting the code. Read more about hooks [here](./devops/hooks/README.md).

### `meksmith`

`meksmith` is a library consisting of parsers and _smiths_ (code generators). `meklang` parser is implemented using [`chumsky`](https://github.com/zesterer/chumsky) and follows a strictly defined DSL, which can be found in [`meksmith/src/parser.rs`](./meksmith/src/parser.rs). The parsed code is represented by a few simple nodes, based on which smiths can be created.

Each smith has two goals:

- create user-friendly representation of the defined protocol using types provided by the language, no external types should be used (e.g. from external libraries/crates/modules), and
- create encoders and decoders for these types using all attributes, so users can easily call `encode_my_struct` and get some bytes that follow their protocol, or `decode_my_struct` which takes some bytes and returns the "simple" types.

Since `meksmith` is a library, it cannot be executed directly. `meksmith`'s code can be used in `examples`, allowing to execute the code with arbitrary protocols. To run an example, call `cargo run --example <example_name>`. If you're working on the `meksmith` library, make sure that you add unit/integration tests that can be executed with `cargo test`.

### `website`

[`meksmith.rs`](https://meksmith.rs) is implemented using [Leptos](https://github.com/leptos-rs/leptos) in the CSR mode (i.e. website is compiled to WASM and everything works in the browser, on the client side). It means that that there is no backend, no database, whatever happens, happens in user's browser.

To start working on the website, you should get familiar with Leptos first and probably the best resource for learning is the [Leptos Book](https://book.leptos.dev/). The process of setting up Rust for web development is described here, but in short you should:

```bash
cargo install trunk
rustup target add wasm32-unknown-unknown
```

Then, from the `website` directory, call `trunk serve` and you'll have the website running on localhost.
