use rand::distributions::Uniform;
use rand::prelude::*;
use sdl2::video::WindowContext;
use sdl2::EventPump;
use std::time::Instant;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, RenderTarget, Texture, TextureAccess, TextureCreator};
use sdl2::ttf::{Font, Sdl2TtfContext};

use crate::hittable::{HitRecord, Hittable, HittableList, Sphere};
use crate::renderer::{Camera, Ray, SampleMap, Vec3};

use std::simd::{Simd, SimdFloat};

pub struct Renderer {
    camera: Camera,
    hittables: HittableList,
    sample_map: SampleMap,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    height: f64,
    width: f64,
    texture_creator: TextureCreator<WindowContext>,
    sdl_context: sdl2::Sdl,
}

impl Renderer {
    /// Creates a new Renderer
    /// # Arguments
    /// * `hittables` - A list of objects that implement the Hittable trait
    /// * `canvas` - An SDL2 canvas, here of type Window but I suppose it could be anything
    /// * `image_height` - The height of the output image
    /// * `image_width` - The width of the output image
    /// * `max_samples` - The maximum number of ray samples per pixel
    pub fn new<'a>(height: u32, width: u32, max_samples: u32) -> Self {
        // For now just add objects to the scene statically
        let hittables = Self::create_world();
        let (canvas, sdl_context) = Self::create_sdl_canvas(width, height);

        let texture_creator = canvas.texture_creator();

        Self {
            camera: Camera::new(),
            hittables,
            sample_map: SampleMap::new(max_samples, width as usize, height as usize),
            canvas,
            height: height as f64,
            width: width as f64,
            texture_creator,
            sdl_context,
        }
    }

    fn create_world() -> HittableList {
        let mut hittables = HittableList::new();
        hittables.add_hittable(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
        hittables.add_hittable(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
        hittables
    }

    /// Renders as many pixels as possible in 1/60 seconds (Maybe make the time constant variable)
    pub fn render_one(
        &mut self,
        width_rng: Uniform<f64>,
        height_rng: Uniform<f64>,
        rng: Uniform<f64>,
    ) {
        let now = Instant::now();

        while now.elapsed().as_secs_f64() < 1. / 60. {
            // Get a random pixel
            let i = width_rng.sample(&mut thread_rng());
            let j = height_rng.sample(&mut thread_rng());

            // Add jitter
            let jitter_u = rng.sample(&mut thread_rng());
            let jitter_v = rng.sample(&mut thread_rng());
            let u = (i + jitter_u) / (self.width - 1.);
            let v = ((self.height - j) + jitter_v) / (self.height - 1.);

            // Determine if the ray intersects any objects
            let ray = Self::ray_colour(&self.camera.get_ray(u, v), &self.hittables);
            self.sample_map
                .set_value(i as usize, j as usize, Simd::<f64, 4>::from(ray));

            // Draw the colour we expect
            self.canvas.set_draw_color(Self::constrain_colour(
                i as usize,
                j as usize,
                &self.sample_map,
            ));
            self.canvas
                .draw_point(Point::new(i as i32, j as i32))
                .expect("Failed to draw point");
        }
    }

    /// Perform the actual render looping, generate as many pixels as possible in 1/60 seconds
    pub fn render(&mut self) {
        // Create our random sources
        let width_rng = Uniform::from((0.)..self.width);
        let height_rng = Uniform::from((0.)..self.height);
        let uniform_dist = Uniform::from((0.)..=1.);

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

        let now = Instant::now();
        // Render a frame to a texture
        self.render_one(width_rng, height_rng, uniform_dist);

        let fps_str = format!("{:.2} FPS", 1.0 / now.elapsed().as_secs_f64());
        self.render_text(fps_str, 0, 0);

        self.canvas.present();
    }

    pub fn get_event_pump(&self) -> EventPump {
        self.sdl_context.event_pump().unwrap()
    }

    pub fn create_sdl_canvas(
        width: u32,
        height: u32,
    ) -> (sdl2::render::Canvas<sdl2::video::Window>, sdl2::Sdl) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("raytracer", width, height)
            .opengl()
            .position_centered()
            .build()
            .unwrap();

        (window.into_canvas().build().unwrap(), sdl_context)
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

    fn constrain_colour(x: usize, y: usize, sample_map: &SampleMap) -> Color {
        let mut values = sample_map.get_values(x, y);
        let scale = 1. / values.1 as f64;
        values.0 *= Simd::splat(scale);
        values.0 = values.0.simd_clamp(Simd::splat(0.), Simd::splat(0.999));
        values.0 *= Simd::splat(255.);
        let cast_vec = Simd::cast::<u8>(values.0);
        Color::RGB(cast_vec[0], cast_vec[1], cast_vec[2])
    }

    fn render_text(&mut self, text: String, x: i32, y: i32) {
        let ttf_context = &sdl2::ttf::init().unwrap();
        let font = ttf_context
            .load_font("C:\\Windows\\Fonts\\verdana.ttf", 12)
            .expect("Failed to load font");

        let rect_size = font.size_of(&text).unwrap();

        let rendered_text = font.render(&text).solid(Color::RGBA(0, 0, 0, 255)).unwrap();
        let text_texture = rendered_text.as_texture(&self.texture_creator).unwrap();

        self.canvas
            .copy(
                &text_texture,
                None,
                Rect::new(x, y, rect_size.0, rect_size.1),
            )
            .unwrap();
    }
}
