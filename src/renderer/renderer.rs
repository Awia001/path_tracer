use indicatif::ProgressBar;
use rand::distributions::Uniform;
use rand::prelude::*;
use std::time::Instant;

use crate::hittable::{HitRecord, Hittable, HittableList, Sphere};
use crate::renderer::{Camera, Ray};

type Vec3 = nalgebra::Vector3<f64>;
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
                let mut colour = Vec3::default();
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
                let rgb_colour = Self::constrain_colour(&colour, self.max_samples);
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
            return Vec3::default();
        }

        // If we hit a hittable in the world
        if world.hit(ray, 0.001, f64::MAX, &mut rec) {
            // Generate a random vector from the point where we hit, the normal and a unit vector in random direction inside the unit sphere on that surface
            // This is a reasonable aproximation to Lambertian reflectance
            let target = rec.point + rec.normal + Self::random_unit_vector();

            // Return the colour of that ray (at half power)
            return 0.5
                * Self::ray_colour(&Ray::new(rec.point, target - rec.point), world, depth - 1);
            //return rec.normal;
        }

        // If we didn't hit anything then lerp between white and a kind of blue as we go up the screen
        let unit_direction = ray.dir.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3::new(1.0, 1.0, 1.0).lerp(&Vec3::new(0.5, 0.7, 1.0), t)
    }

    pub fn random_in_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            rand::thread_rng().gen_range(min..max),
            rand::thread_rng().gen_range(min..max),
            rand::thread_rng().gen_range(min..max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Self::random_in_range(-1., 1.);
            if p.magnitude_squared() >= 1. {
                continue;
            }
            return p;
        }
    }

    fn random_unit_vector() -> Vec3 {
        Self::random_in_unit_sphere().normalize()
    }

    fn constrain_colour(colour: &Vec3, samples: u64) -> image::Rgb<u8> {
        // Scale our colour values by how many samples we have taken
        let scale = Vec3::new(1., 1., 1.) / samples as f64;
        let mut colour = colour.component_mul(&scale);

        // Simple gamma correction
        colour = Self::sqrt(&colour);

        // Clamp our values between 0. and 0.999 then multiply by 255 to get a value that will fit in a u8
        colour.cap_magnitude(0.999);
        colour = colour * 255.;

        let cast_vec: [u8; 3] = [colour.x as u8, colour.y as u8, colour.z as u8];

        image::Rgb([cast_vec[0], cast_vec[1], cast_vec[2]])
    }

    fn sqrt(vec: &Vec3) -> Vec3 {
        Vec3::new(vec.x.sqrt(), vec.y.sqrt(), vec.z.sqrt())
    }

    pub fn clamp(vec: &mut Vec3, min: f64, max: f64) {
        vec.x = vec.x.clamp(min, max);
        vec.y = vec.y.clamp(min, max);
        vec.z = vec.z.clamp(min, max);
    }
}
