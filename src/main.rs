#![feature(portable_simd)]

mod camera;
mod hit_record;
mod hittable;
mod hittable_list;
mod ray;
mod sample_map;
mod sphere;
mod vec3;

use crate::camera::Camera;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sample_map::SampleMap;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

use rand::distributions::{uniform, Uniform};
use rand::prelude::Distribution;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, RenderTarget, TextureCreator};
use sdl2::ttf::Font;

use std::simd::{Simd, SimdFloat};
use std::time::Instant;

use rand::Rng;

fn create_sdl_canvas(
    width: u32,
    height: u32,
) -> (
    sdl2::render::Canvas<sdl2::video::Window>,
    sdl2::Sdl,
    sdl2::ttf::Sdl2TtfContext,
) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("raytracer", width, height)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    (
        window.into_canvas().build().unwrap(),
        sdl_context,
        sdl2::ttf::init().unwrap(),
    )
}

fn ray_colour(ray: &Ray, world: &impl Hittable) -> Vec3 {
    let mut rec = HitRecord {
        ..Default::default()
    };

    if world.hit(ray, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Vec3::new(1.0, 1.0, 1.0));
    }

    let unit_direction = ray.dir.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn constrain_colour_instant(colour: &mut Vec3, samples: u32) -> Color {
    let mut r = colour.x();
    let mut g = colour.y();
    let mut b = colour.z();

    let scale = 1. / samples as f64;

    r = r * scale;
    g = g * scale;
    b = b * scale;

    let r = r.clamp(0., 0.999);
    let g = g.clamp(0., 0.999);
    let b = b.clamp(0., 0.999);

    Color::RGB((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

fn constrain_colour<const T: usize>(x: usize, y: usize, sample_map: &SampleMap<T>) -> Color {
    let mut values = sample_map.get_values(x, y);
    let scale = 1. / values.1 as f64;
    values.0 *= Simd::splat(scale);
    values.0 = values.0.simd_clamp(Simd::splat(0.), Simd::splat(0.999));
    values.0 *= Simd::splat(255.);
    let cast_vec = Simd::cast::<u8>(values.0);
    Color::RGB(cast_vec[0], cast_vec[1], cast_vec[2])
}

fn render_text<T, U: RenderTarget>(
    text: String,
    font: &Font,
    texture_creator: &TextureCreator<T>,
    canvas: &mut Canvas<U>,
) {
    let rect_size = font.size_of(&text).unwrap();

    let rendered_text = font.render(&text).solid(Color::RGBA(0, 0, 0, 255)).unwrap();
    let text_texture = rendered_text.as_texture(texture_creator).unwrap();
    canvas
        .copy(
            &text_texture,
            None,
            Rect::new(0, 0, rect_size.0, rect_size.1),
        )
        .unwrap();
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const FULL_SIZE: usize = (IMAGE_HEIGHT * IMAGE_WIDTH) as usize;
    let samples_per_pixel = 1;

    let created = create_sdl_canvas(IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut canvas = created.0;
    let sdl_context = created.1;
    let ttf_context = created.2;
    let texture_creator = canvas.texture_creator();

    let font = ttf_context
        .load_font("C:\\Windows\\Fonts\\verdana.ttf", 12)
        .expect("Failed to load font");

    let mut rng = rand::thread_rng();
    let uniform = Uniform::<f64>::from((0.)..=1.);

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut camera = Camera::new();

    let mut hittables = HittableList::new();
    hittables.add_hittable(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    hittables.add_hittable(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let sample_map = &mut SampleMap::<{ FULL_SIZE }>::new(
        samples_per_pixel,
        IMAGE_WIDTH.try_into().expect("Couldn't cast"),
    );
    'running: loop {
        let now = Instant::now();

        // for j in (0..IMAGE_HEIGHT).rev() {
        //     println!("Lines left: {j}");
        //     for i in 0..=IMAGE_WIDTH {
        //         let mut colour = Vec3::new(0., 0., 0.);
        //         for _ in 0..samples_per_pixel {
        //             let jitter_u = uniform.sample(&mut rng);
        //             let jitter_v = uniform.sample(&mut rng);
        //             let u = (i as f64 + jitter_u) / (IMAGE_WIDTH - 1) as f64;
        //             let v = ((IMAGE_HEIGHT - j) as f64 + jitter_v) / (IMAGE_HEIGHT - 1) as f64;
        //             let r = camera.get_ray(u, v);
        //             colour += ray_colour(&r, &hittables);
        //             // println!("Colour: {colour:?}")
        //         }
        //         let render_colour = constrain_colour_instant(&mut colour, samples_per_pixel);
        //         println!("Colour to render: {render_colour:?}");
        //         canvas.set_draw_color(render_colour);
        //         canvas.draw_point(Point::new(i as i32, j as i32)).ok();
        //     }
        // }

        // While less than 1/60 seconds has elapsed
        while now.elapsed().as_secs_f64() < 1. / 60. {
            // Get a random pixel
            let i = rng.gen_range(0..IMAGE_WIDTH) as f64;
            let j = rng.gen_range(0..IMAGE_HEIGHT) as f64;

            // Add jitter
            let jitter_u = uniform.sample(&mut rng);
            let jitter_v = uniform.sample(&mut rng);
            let u = (i as f64 + jitter_u) / (IMAGE_WIDTH - 1) as f64;
            let v = ((IMAGE_HEIGHT as f64 - j) as f64 + jitter_v) / (IMAGE_HEIGHT - 1) as f64;

            // Determine if the ray intersects any objects
            let ray = ray_colour(&camera.get_ray(u, v), &hittables);
            sample_map.set_value(i as usize, j as usize, Simd::<f64, 4>::from(ray));

            // Draw the colour we expect
            canvas.set_draw_color(constrain_colour(i as usize, j as usize, &sample_map));
            canvas
                .draw_point(Point::new(i as i32, j as i32))
                .expect("Failed to draw point");
        }

        // canvas.set_draw_color(Color::RGB(255, 255, 255));
        // let x: i32 = rng.gen_range(0..image_width).try_into().unwrap();
        // let y: i32 = rng.gen_range(0..image_width).try_into().unwrap();
        // canvas
        //     .draw_point(Point::new(x, y))
        //     .expect("Failed to draw point");

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
                    camera.translate_x(-0.1);
                    sample_map.invalidate_samples();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    camera.translate_x(0.1);
                    sample_map.invalidate_samples();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    camera.translate_z(-0.1);
                    sample_map.invalidate_samples();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    camera.translate_z(0.1);
                    sample_map.invalidate_samples();
                }
                _ => {}
            }
        }
        let fps_str = format!("{:.2} FPS", 1.0 / now.elapsed().as_secs_f64());
        render_text(fps_str, &font, &texture_creator, &mut canvas);

        canvas.present();

        // ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
