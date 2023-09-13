# sweet-demo
Demo for the sweet test framework

## Quickstart

```sh
### NATIVE ###

cargo run --example sweet
# watch
cargo watch -- cargo run --example sweet -- -w
# e2e
cargo run --example sweet --features=e2e

### WEB ###

forky sweet --example sweet
# watch
forky sweet --example sweet -w
# interactive
forky sweet --example sweet -w --interactive
```