use std::io::Write;
mod vec3;
use crate::vec3::Vec3;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;

fn create_sdl_canvas(
    width: u32,
    height: u32,
) -> (sdl2::render::Canvas<sdl2::video::Window>, sdl2::Sdl) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("raytracer", width, height)
        .position_centered()
        .build()
        .unwrap();

    (window.into_canvas().build().unwrap(), sdl_context)
}

fn main() {
    let image_height = 256;
    let image_width = 256;

    let created = create_sdl_canvas(image_width, image_height);
    let mut canvas = created.0;
    let sdl_context = created.1;

    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for j in (0..image_height).rev() {
            eprintln!("Lines remaining: {j}");
            for i in 0..image_width {
                let r = i as f64 / (image_width - 1) as f64;
                let g = j as f64 / (image_height - 1) as f64;
                let b = 0.25;
                let colour = Vec3(
                    (255.999 * r) as f64,
                    (255.999 * g) as f64,
                    (255.999 * b) as f64,
                );
                canvas.set_draw_color(Color::RGB(colour.0 as u8, colour.1 as u8, colour.2 as u8));
                canvas.draw_point(Point::new(i as i32, j as i32)).ok();
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
