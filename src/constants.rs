#![allow(bad_style)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub enum ANGLE_MODE {
    Radians = "radians",
    Degrees = "degrees",
}
#[wasm_bindgen]
pub enum ARC_MODE {
    Chord = "chord",
    Pie = "pie",
    Open = "open",
}
#[wasm_bindgen]
pub enum BEGIN_KIND {
    Points = 0x0000,
    Lines = 0x0001,
    Triangles = 0x0004,
    TriangleFan = 0x0006,
    TriangleStrip = 0x0005,
    // Quads = "quads",
    // QuadStrip = "quad_strip",
    // Tess = "tess",
}
#[wasm_bindgen]
pub enum BLEND_MODE {
    Blend = "source-over",
    Darkest = "darkest",
    Lightest = "lighten",
    Difference = "difference",
    Multiply = "multiply",
    Exclusion = "exclusion",
    Screen = "screen",
    Replace = "copy",
    Overlay = "overlay",
    HardLight = "hard-light",
    SoftLight = "soft-light",
    Dodge = "color-dodge",
    Burn = "color-burn",
    Add = "lighter",
    Normal = "normal",
}
#[wasm_bindgen]
pub enum COLOR_MODE {
    Rgb = "rgb",
    Hsb = "hsb",
    Hsl = "hsl",
}
#[wasm_bindgen]
pub enum CURSOR_TYPE {
    Arrow = "arrow",
    Cross = "cross",
    Hand = "hand",
    Move = "move",
    Text = "text",
}
#[wasm_bindgen]
pub enum DEBUG_MODE {
    Grid = "grid",
    Axes = "axes",
}
#[wasm_bindgen]
pub enum DESCRIBE_DISPLAY {
    Label = "label",
    Fallback = "fallback",
}
#[wasm_bindgen]
pub enum ELLIPSE_MODE {
    Center = "center",
    Radius = "radius",
    Corner = "corner",
    Corners = "corners",
}
#[wasm_bindgen]
pub enum END_MODE {
    Close = "close",
}
#[wasm_bindgen]
pub enum FILTER_TYPE {
    Threshold = "threshold",
    Gray = "gray",
    Opaque = "opaque",
    Invert = "invert",
    Posterize = "posterize",
    Blur = "blur",
    Erode = "erode",
    Dilate = "dilate",
}
#[wasm_bindgen]
pub enum GRAPHICS_RENDERER {
    P2d = "p2d",
    Webgl = "webgl",
}
#[wasm_bindgen]
pub enum HORIZ_ALIGN {
    Left = "left",
    Center = "center",
    Right = "right",
}
#[wasm_bindgen]
pub enum IMAGE_MODE {
    Corner = "corner",
    Corners = "corners",
    Center = "center",
}
#[wasm_bindgen]
pub enum RECT_MODE {
    Corner = "corner",
    Corners = "corners",
    Center = "center",
    Radius = "radius",
}
#[wasm_bindgen]
pub enum RENDERER {
    P2d = "p2d",
    Webgl = "webgl",
}
#[wasm_bindgen]
pub enum SIZE_H {
    Auto = "auto",
}
#[wasm_bindgen]
pub enum SIZE_W {
    Auto = "auto",
}
#[wasm_bindgen]
pub enum STROKE_CAP {
    Round = "round",
    Square = "butt",
    Project = "square",
}
#[wasm_bindgen]
pub enum STROKE_JOIN {
    Miter = "miter",
    Bevel = "bevel",
    Round = "round",
}
#[wasm_bindgen]
pub enum TEXTURE_MODE {
    Image = "image",
    Normal = "normal",
}
#[wasm_bindgen]
pub enum THE_STYLE {
    Normal = "normal",
    Italic = "italic",
    Bold = "bold",
    Bolditalic = "bolditalic",
}
#[wasm_bindgen]
pub enum TYPE {
    Video = "video",
    Audio = "audio",
}
#[wasm_bindgen]
pub enum VERT_ALIGN {
    Top = "top",
    Bottom = "bottom",
    Center = "center",
    Baseline = "alphabetic",
}
#[wasm_bindgen]
pub enum WRAP_X {
    Clamp = "clamp",
    Repeat = "repeat",
    Mirror = "mirror",
}
#[wasm_bindgen]
pub enum WRAP_Y {
    Clamp = "clamp",
    Repeat = "repeat",
    Mirror = "mirror",
}
