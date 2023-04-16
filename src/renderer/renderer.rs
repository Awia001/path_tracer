use rand::distributions::Uniform;
use rand::prelude::*;
use sdl2::video::WindowContext;
use sdl2::EventPump;
use std::time::Instant;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;

use crate::hittable::{HitRecord, Hittable, HittableList, Sphere};
use crate::renderer::{Camera, Ray, SampleMap, Vec3};

use std::simd::Simd;

pub struct Renderer {
    camera: Camera,
    hittables: HittableList,
    sample_map: SampleMap,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    height: f64,
    width: f64,
    sdl_context: sdl2::Sdl,
    max_depth: u32,
}

impl Renderer {
    /// Creates a new Renderer
    /// # Arguments
    /// * `hittables` - A list of objects that implement the Hittable trait
    /// * `canvas` - An SDL2 canvas, here of type Window but I suppose it could be anything
    /// * `image_height` - The height of the output image
    /// * `image_width` - The width of the output image
    /// * `max_samples` - The maximum number of ray samples per pixel
    pub fn new(height: u32, width: u32, max_samples: u32) -> Self {
        // For now just add objects to the scene statically
        let hittables = Self::create_world();
        let (mut canvas, sdl_context) = Self::create_sdl_canvas(width, height);
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.present();

        let mut renderer = Self {
            camera: Camera::new(),
            hittables,
            sample_map: SampleMap::new(max_samples, width as usize, height as usize),
            canvas,
            height: height as f64,
            width: width as f64,
            sdl_context,
            max_depth: 50,
        };
        renderer.render_first_frame();
        renderer.canvas.present();
        renderer
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
            // Get a random point
            let i = width_rng.sample(&mut thread_rng());
            let j = height_rng.sample(&mut thread_rng());

            if self
                .sample_map
                .is_saturated(i as usize, (self.height - j) as usize)
            {
                // If we have saturated this point start again and determine a new point
                continue;
            }

            // Add jitter
            let jitter_u = rng.sample(&mut thread_rng());
            let jitter_v = rng.sample(&mut thread_rng());
            let u = (i + jitter_u) / (self.width - 1.);
            let v = ((self.height - j) + jitter_v) / (self.height - 1.);

            // Determine if the ray intersects any objects
            let ray = Self::ray_colour(&self.camera.get_ray(u, v), &self.hittables, self.max_depth);
            self.sample_map
                .add_sample_to_map(i as usize, j as usize, Simd::<f64, 4>::from(ray));
        }
    }

    /// Render one frame as we create the Renderer object so that we have a complete image before presenting
    fn render_first_frame(&mut self) {
        self.render_text(
            "Rendering first frame",
            self.width as u32 / 2,
            self.height as u32 / 2,
            Color::RGB(0, 0, 0),
        );
        self.canvas.present();
        for j in (0..self.height as u32).rev() {
            for i in 0..self.width as u32 {
                let u = (i as f64) / (self.width - 1.);
                let v = (self.height - j as f64) / (self.height - 1.);
                let r = self.camera.get_ray(u, v);
                let colour = Self::ray_colour(&r, &self.hittables, 25);
                self.sample_map
                    .add_sample_to_map(i as usize, j as usize, colour.0);
            }
        }
    }

    /// Perform the actual render looping, generate as many pixels as possible in 1/60 seconds
    pub fn render(&mut self) {
        // Create our random sources
        let width_rng = Uniform::from((0.)..self.width);
        let height_rng = Uniform::from((0.)..self.height);
        let uniform_dist = Uniform::from((0.)..=1.);

        let now = Instant::now();
        // Render a frame to a texture
        self.render_one(width_rng, height_rng, uniform_dist);

        let fps_str = format!("{:.2} FPS", 1.0 / now.elapsed().as_secs_f64());
        self.render_text(fps_str.as_str(), 0, 0, Color::RGBA(0, 0, 0, 255));

        for j in (0..self.height as u32).rev() {
            for i in 0..self.width as u32 {
                let colour = self.sample_map.constrain_colour(i as usize, j as usize);
                self.canvas.set_draw_color(colour);
                self.canvas
                    .draw_point(Point::new(i as i32, j as i32))
                    .expect("Failed to draw point");
            }
        }
        self.canvas.present();
    }

    /// Generate the SDL2 event pump for whoever wants it
    pub fn get_event_pump(&self) -> EventPump {
        self.sdl_context.event_pump().unwrap()
    }

    /// Create the SDL2 context and window canvas
    pub fn create_sdl_canvas(
        width: u32,
        height: u32,
    ) -> (sdl2::render::Canvas<sdl2::video::Window>, sdl2::Sdl) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Path Tracer", width, height)
            .opengl()
            .position_centered()
            .build()
            .unwrap();

        (window.into_canvas().build().unwrap(), sdl_context)
    }

    /// Function to determine the colour of a ray
    fn ray_colour(ray: &Ray, world: &impl Hittable, depth: u32) -> Vec3 {
        let mut rec = HitRecord {
            ..Default::default()
        };

        // Have we reached recursion depth?
        if depth == 0 {
            return Vec3::new(0., 0., 0.);
        }

        // If we hit a hittable in the world
        if world.hit(ray, 0.001, f64::INFINITY, &mut rec) {
            // Generate a random vector from the point where we hit, the normal and a unit vector in random direction inside the unit sphere on that surface
            // This is a reasonable aproximation to Lambertian reflectance
            let target = rec.point + rec.normal + Vec3::random_unit_vector();

            // Return the colour of that ray (at half power)
            return 0.5
                * Self::ray_colour(&Ray::new(rec.point, target - rec.point), world, depth - 1);
        }

        // If we didn't hit anything then lerp between white and a kind of blue as we go up the screen
        let unit_direction = ray.dir.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }

    fn render_text(&mut self, text: &str, x: u32, y: u32, colour: Color) {
        let ttf_context = &sdl2::ttf::init().unwrap();
        let font = ttf_context
            .load_font("C:\\Windows\\Fonts\\verdana.ttf", 12)
            .expect("Failed to load font");

        let rect_size = font.size_of(text).unwrap();

        let texture_creator: TextureCreator<WindowContext> = self.canvas.texture_creator();

        let rendered_text = font.render(text).solid(colour).unwrap();
        let text_texture = rendered_text.as_texture(&texture_creator).unwrap();

        self.canvas
            .copy(
                &text_texture,
                None,
                Rect::new(x as i32, y as i32, rect_size.0, rect_size.1),
            )
            .unwrap();
    }
}
