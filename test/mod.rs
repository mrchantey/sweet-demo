// include sweet - it will export the main function to collect & run our tests
#![feature(imported_main)]
pub use sweet::*;

// include tests that should run natively & in-broswer
mod common;

// include tests that should only run with feature
#[cfg(feature = "e2e")]
mod e2e;

// include tests that should only run in-browser
#[cfg(target_arch = "wasm32")]
mod web;
