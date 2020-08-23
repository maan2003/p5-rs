use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum Renderer {
    WebGl = "webgl",
    P2D = "p2d",
}

function!{
    pub fn create_canvas
    js createCanvas 
    struct Canvas
    { width: u32, height: u32 }
    { renderer: Renderer = Renderer::P2D }
}
