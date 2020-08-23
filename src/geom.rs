use wasm_bindgen::prelude::*;

function! {
    pub fn ellipse
    js ellipse
    struct Ellipse
    { x: i32, y: i32, w: i32, h: i32}
    { detail: i32 = 0 }
}

function! {
    pub fn circle
    js circle
    struct Circle
    { x: i32, y: i32, d: i32}
    { }
}

function! {
    pub fn rect
    js rect
    struct Rect
    { x: i32, y: i32, w: i32, h: i32 }
    {
        upper_left_rad: i32 = 0,
        upper_right_rad: i32 = 0,
        lower_right_rad: i32 = 0,
        lower_left_rad: i32 = 0
    }
}

impl Rect {
    pub fn corner_radius(&mut self, r: i32) -> &mut Self {
        self.upper_left_rad = r;
        self.upper_right_rad = r;
        self.lower_right_rad = r;
        self.lower_left_rad = r;
        self
    }
}

pub fn square(x: i32, y: i32, size: i32) -> Rect {
    rect(x, y, size, size)
}

#[wasm_bindgen]
extern "C" {
    pub fn triangle(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32);
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub enum ArcMode {
    Chord = "chord",
    Open = "open",
    Pie = "pie",
}
function! {
    pub fn arc
    js arc
    struct Arc
    {
        x: i32, y: i32,
        w: i32, h: i32
    }
    {
        start: f32 = 0.0, stop: f32 = (2.0 * std::f32::consts::PI),
        mode: ArcMode = ArcMode::Pie,
        detail: i32 = 0
    }
}

// todo : use some kind of Into<Point> trait here
#[wasm_bindgen]
extern "C" {
    pub fn point(x: i32, y: i32);
    #[wasm_bindgen(js_name = point)]
    pub fn point_3d(x: i32, y: i32, z: i32);
    
    pub fn line(
        x1: i32, y1: i32,
        x2: i32, y2: i32,
    );

    #[wasm_bindgen(js_name = line)]
    pub fn line_3d(
        x1: i32, y1: i32, z1: i32,
        x2: i32, y2: i32, z2: i32,
    );

    pub fn quad(
        x1: i32, y1: i32,
        x2: i32, y2: i32,
        x3: i32, y3: i32,
        x4: i32, y4: i32,
    );

    #[wasm_bindgen(js_name = quad)]
    pub fn quad_3d(
        x1: i32, y1: i32, z1: i32,
        x2: i32, y2: i32, z2: i32,
        x3: i32, y3: i32, z3: i32,
        x4: i32, y4: i32, z4: i32,
    );
}