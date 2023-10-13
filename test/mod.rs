// include sweet - it will export the main function to collect & run our tests
#![feature(imported_main)]
pub use sweet::*;

// include tests that should run natively & in-broswer
mod common;

// include tests that should only run with feature
#[cfg(all(not(target_arch = "wasm32"), feature = "e2e"))]
mod native_e2e;

// include tests that should only run in-browser
#[cfg(target_arch = "wasm32")]
mod web_component;
#[cfg(target_arch = "wasm32")]
mod web_e2e;
