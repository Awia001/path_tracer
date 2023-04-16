use rand::thread_rng;
use sdl2::pixels::Color;
use std::simd::{Simd, SimdFloat, StdFloat};
use std::vec::Vec;

use rand::seq::IteratorRandom;

/// A struct to hold colour samples at each point in screen space
pub struct SampleMap {
    /// The colours themselves
    colours: Vec<Simd<f64, 4>>,
    /// How many samples we've taken at each point
    samples: Vec<u32>,
    /// The width of screen space
    pub width: usize,
    /// The height of screen space
    pub height: usize,
    /// The maximum number of samples we should take for each point
    pub max_samples: u32,
}

impl Default for SampleMap {
    fn default() -> Self {
        Self {
            colours: vec![Simd::splat(0.0); 1],
            samples: vec![0; 1],
            width: 1,
            height: 1,
            max_samples: 1,
        }
    }
}

impl SampleMap {
    pub fn new(max_samples: u32, width: usize, height: usize) -> Self {
        Self {
            max_samples,
            width,
            height,
            colours: vec![Simd::splat(0.); width * height],
            samples: vec![0; width * height],
        }
    }

    // Turn 2D array back into the index of a 1D array
    fn calc_index(&self, x: usize, y: usize) -> usize {
        x + self.width * y
    }

    // Turn a 1D index into 2D co-ords
    fn calc_coords(&self, index: usize) -> (usize, usize) {
        let x = index % self.width;
        let y = index / self.width;
        (x, y)
    }

    pub fn add_sample_to_map(&mut self, x: usize, y: usize, value: Simd<f64, 4>) {
        // Get the index into our array
        let index = self.calc_index(x, y);
        let samples = self.samples.get_mut(index).expect("Index out of bounds");

        // If we've already sampled this ray max times just skip
        if *samples == self.max_samples {
            return;
        }
        let colour = self.colours.get_mut(index).expect("Index out of bounds");

        // Add our new sample to the list
        *colour += value;

        // Increment our sample count
        *samples += 1;
    }

    /// Determine if we have saturated a particular sample point
    pub fn is_saturated(&self, x: usize, y: usize) -> bool {
        let index = self.calc_index(x, y);
        let samples = self.samples.get(index).expect("Index out of bounds");

        *samples >= self.max_samples
    }

    /// Get a random unsaturated point in screen space to sample the colour at
    pub fn get_random_unsaturated(&self) -> (usize, usize) {
        // Get all points which have not been sampled
        let all_unsampled = self
            .samples
            .iter()
            .enumerate()
            .filter(|sample| *sample.1 == 0);

        // determine it's 2D co-ords then choose one at random
        let choice = all_unsampled.choose(&mut thread_rng()).unwrap_or((0, &0)); // A reference to a static number feels odd

        self.calc_coords(choice.0)
    }

    pub fn get_values(&self, x: usize, y: usize) -> (Simd<f64, 4>, u32) {
        let index = self.calc_index(x, y);
        (
            *self.colours.get(index).expect("Index out of range"),
            *self.samples.get(index).expect("Index out of range"),
        )
    }

    /// Get the full scale colour from the sample map then get the value constrained by the sample size
    pub fn constrain_colour(&self, x: usize, y: usize) -> Color {
        let (mut values, samples) = self.get_values(x, y);

        // Scale our colour values by how many samples we have taken
        let scale = Simd::splat(1. / samples as f64);
        values *= scale;

        // Simple gamma correction
        values = values.sqrt();

        // Clamp our values between 0. and 0.999 then multiply by 255 to get a value that will fit in a u8
        values = values.simd_clamp(Simd::splat(0.), Simd::splat(0.999));
        values *= Simd::splat(255.);

        // Cast to u8 and create an SDL2 Color struct from that
        let cast_vec = Simd::cast::<u8>(values);

        Color::RGB(cast_vec[0], cast_vec[1], cast_vec[2])
    }

    pub fn invalidate_samples(&mut self) {
        self.samples = vec![0; self.width * self.height];
    }

    /// Gets the colour data held in self.colours as an array of u8 integers, esentially RGB24
    pub fn get_pixel_data(&self) -> Vec<u8> {
        let mut colour_data = vec![0; self.width * self.height * 3];
        for (pos, _) in self.colours.iter().enumerate() {
            let (x, y) = self.calc_coords(pos);
            colour_data[pos * 3..pos * 3 + 3].clone_from_slice(&self.constrain_colour(x, y));
        }
        println!("Copied constrained colour data from map to slice");
        colour_data
    }
}

// impl<Idx> Index<Idx> for SampleMap {
//     type Output = u8;
//     fn index(&self, index: Idx) -> &Self::Output {}
// }
