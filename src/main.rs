#![feature(portable_simd)]

mod hittable;
mod renderer;

use crate::renderer::Renderer;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const FULL_SIZE: usize = (IMAGE_HEIGHT * IMAGE_WIDTH) as usize;
    const SAMPLES_PER_PIXEL: u32 = 1;

    let mut renderer = Renderer::new(IMAGE_HEIGHT, IMAGE_WIDTH, SAMPLES_PER_PIXEL);

    let mut event_pump = renderer.get_event_pump();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    // camera.translate_x(-0.1);
                    // sample_map.invalidate_samples();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    // camera.translate_x(0.1);
                    // sample_map.invalidate_samples();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    // camera.translate_z(-0.1);
                    // sample_map.invalidate_samples();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    // camera.translate_z(0.1);
                    // sample_map.invalidate_samples();
                }
                _ => {}
            }
        }
        renderer.render();
    }
}
