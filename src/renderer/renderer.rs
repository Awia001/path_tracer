use indicatif::ProgressBar;
use rand::distributions::Uniform;
use rand::prelude::*;
use std::time::{Duration, Instant};

use crate::hittable::{HitRecord, Hittable, HittableList, Sphere};
use crate::renderer::{Camera, Ray, Vec3};

pub struct Renderer {
    camera: Camera,
    hittables: HittableList,
    height: u64,
    width: u64,
    max_depth: u64,
    max_samples: u64,
}

impl Renderer {
    /// Creates a new Renderer
    /// # Arguments
    /// * `hittables` - A list of objects that implement the Hittable trait
    /// * `canvas` - An SDL2 canvas, here of type Window but I suppose it could be anything
    /// * `image_height` - The height of the output image
    /// * `image_width` - The width of the output image
    /// * `max_samples` - The maximum number of ray samples per pixel
    pub fn new(height: u64, width: u64, max_depth: u64, max_samples: u64) -> Self {
        // For now just add objects to the scene statically
        let hittables = Self::create_world();

        Self {
            camera: Camera::new(),
            hittables,
            height,
            width,
            max_depth,
            max_samples,
        }
    }

    fn create_world() -> HittableList {
        let mut hittables = HittableList::new();
        hittables.add_hittable(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
        hittables.add_hittable(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
        hittables
    }

    /// Perform the actual render looping, generate as many pixels as possible in 1/60 seconds
    pub fn render(&mut self) {
        let uniform_dist = Uniform::from((0.)..=1.);

        let mut image = image::ImageBuffer::new(self.width as u32, self.height as u32);
        let start = Instant::now();

        println!("Iterating pixels");
        let bar = ProgressBar::new(self.height);
        for j in (0..self.height).rev() {
            for i in 0..self.width {
                let mut colour = Vec3 {
                    ..Default::default()
                };
                for _ in 0..self.max_samples {
                    let u_jitter = uniform_dist.sample(&mut thread_rng());
                    let v_jitter = uniform_dist.sample(&mut thread_rng());
                    let u = (i as f64 + u_jitter) / (self.width - 1) as f64;
                    let v = ((self.height - j) as f64 + v_jitter) / (self.height - 1) as f64;
                    colour += Self::ray_colour(
                        &self.camera.get_ray(u, v),
                        &self.hittables,
                        self.max_depth,
                    )
                }
                let rgb_colour = colour.constrain_colour(self.max_samples);
                image.put_pixel(i as u32, j as u32, rgb_colour);
            }
            bar.inc(1);
        }
        println!("Took {:?} to generate image", start.elapsed());
        println!("Saving file");
        image.save("test.png").unwrap();
    }

    /// Function to determine the colour of a ray
    fn ray_colour(ray: &Ray, world: &impl Hittable, depth: u64) -> Vec3 {
        let mut rec = HitRecord {
            ..Default::default()
        };

        // Have we reached recursion depth?
        if depth == 0 {
            return Vec3 {
                ..Default::default()
            };
        }

        // If we hit a hittable in the world
        if world.hit(ray, 0.001, f64::MAX, &mut rec) {
            // Generate a random vector from the point where we hit, the normal and a unit vector in random direction inside the unit sphere on that surface
            // This is a reasonable aproximation to Lambertian reflectance
            let target = rec.point + rec.normal + Vec3::random_unit_vector();

            // Return the colour of that ray (at half power)
            return 0.5
                * Self::ray_colour(&Ray::new(rec.point, target - rec.point), world, depth - 1);
            //return rec.normal;
        }

        // If we didn't hit anything then lerp between white and a kind of blue as we go up the screen
        let unit_direction = ray.dir.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3::new(1.0, 1.0, 1.0).lerp(&Vec3::new(0.5, 0.7, 1.0), t)
    }
}
