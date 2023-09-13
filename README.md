# sweet-demo
Demo for the sweet test framework.

[Documentation here](https://mrchantey.github.io/forky/docs/sweet/index.html)

## Cheatsheet

```sh
# NATIVE

cargo run --example sweet
## watch
cargo watch -- cargo run --example sweet -- -w
## e2e (requires chromedriver)
cargo run --example sweet --features=e2e

# WEB (requires forky_cli, wasm-bindgen-cli, chromedriver)

forky sweet --example sweet
## watch
forky sweet --example sweet -w
## interactive
forky sweet --example sweet -w --interactive
```