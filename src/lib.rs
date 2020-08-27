#![feature(unboxed_closures)]
#![feature(fn_traits)]

//! `p5-sys` crate(library) is bindings to [p5.js](https://p5js.org/) for Rust/Wasm.
//! 
//! *** Nightly rustc is required ***
//! 
//! Currently,almost all global function works. The instance methods and properties don't work.
//! For example, `vector.add()` will not work. You are advised to use rust types for such tasks. 
//! Also functions that take arrays and modify them will not work. 
//! 
//! The documentation is taken from p5.js reference and are not ported to rust yet.
//! 
//! # Example
//! ```no_run
//! use p5::*;
//! 
//! pub struct State {
//!     x: f64,
//! }
//! 
//! #[wasm_bindgen]
//! pub fn setup() -> State {
//!     createCanvas(400., 400., RENDERER::Webgl);
//!     background(123., 234., 124.);
//!     return State {
//!         x: 10.,
//!     }
//! }
//! 
//! #[wasm_bindgen]
//! pub fn draw(state: &mut State) {
//!     state.x += 1;
//!     rect(state.x, 40., 100., 120.);
//! }
//! ```
//! 
//! We don't have mutable global variable in Rust, so we have to use state.
//! setup creates the state. draw and other event handlers recieve
//! a mutable reference to it, means that they can change it.  

pub mod global;
pub mod constants;
pub mod types;

pub use global::*;
pub use types::*;
pub use constants::*;
pub use wasm_bindgen::prelude::*;