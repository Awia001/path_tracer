use std::simd::Simd;
use std::vec::Vec;

pub struct SampleMap {
    colours: Vec<Simd<f64, 4>>,
    samples: Vec<u32>,
    pub width: usize,
    pub height: usize,
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

    fn calc_index(&self, x: usize, y: usize) -> usize {
        x + self.width * y
    }

    pub fn set_value(&mut self, x: usize, y: usize, value: Simd<f64, 4>) {
        // Get the index into our array
        let index = self.calc_index(x, y);
        let samples = self.samples.get_mut(index).expect("Index out of bounds");

        // If we've already sampled this ray max times just skip
        if *samples == self.max_samples {
            return;
        }
        let colour = self.colours.get_mut(index).expect("Index out of bounds");

        // Have we sampled this ray before?
        if *samples == 0 {
            *colour = value;
        } else {
            *colour += value;
        }

        // Increment our sample
        *samples += 1;
    }

    pub fn get_values(&self, x: usize, y: usize) -> (Simd<f64, 4>, u32) {
        let index = self.calc_index(x, y);
        (
            *self.colours.get(index).expect("Index out of range"),
            *self.samples.get(index).expect("Index out of range"),
        )
    }

    pub fn invalidate_samples(&mut self) {
        self.samples = vec![0; self.width * self.height];
    }
}
