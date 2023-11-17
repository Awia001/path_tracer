mod hittable;
mod renderer;

use crate::renderer::Renderer;

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 800;
    const IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 10;
    const MAX_DEPTH: u64 = 5;

    println!("{}:{} Creating renderer", file!(), line!());
    let mut renderer = Renderer::new(IMAGE_HEIGHT, IMAGE_WIDTH, MAX_DEPTH, SAMPLES_PER_PIXEL);
    println!("Renderer created");
    renderer.render();
}
