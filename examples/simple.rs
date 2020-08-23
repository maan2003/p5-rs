use p5::canvas::create_canvas;
use p5::canvas::Renderer;
use p5::color::{background, Color};
use p5::geom::*;

pub fn setup() {
    create_canvas(400, 400).renderer(Renderer::P2D);
}

pub fn draw() {
    for _ in 0..100 {
        arc(20, 20, 10, 5).start(0.0).stop(1.3).mode(ArcMode::Pie);
    }
}

fn main() {}
