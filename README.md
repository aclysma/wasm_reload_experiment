# wasm_reload_experiment

Playing around with using wasm as a mechanism for code reload.

wasm_host requires a module implementing `fn render(&mut self, input: &str) -> String` and draws the resulting text

wasm_module implements the function

wasm_module could be implemented in any language that supports web assembly types (https://hacks.mozilla.org/2019/08/webassembly-interface-types/)

## Usage

Terminal window 1:
 * cd into wasm_module
 * Run `WASM_INTERFACE_TYPES=1 wasm-pack build`

Terminal window 2:
 * cd into wasm_host
 * Run `cargo run`, leave this open

Change code in `wasm_module/src/lib.rs`

Terminal window 1:
 * Run `WASM_INTERFACE_TYPES=1 wasm-pack build`

The text should update

## Requirements

Must install wasmpack: https://rustwasm.github.io/wasm-pack/

## Status

Just an experiment!

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).
