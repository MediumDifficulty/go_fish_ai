use std::ops::RangeInclusive;

use rand::Rng;

pub fn lerp(range: RangeInclusive<f32>, progress: f32) -> f32 { 
    let r = range.end() - range.start();
    range.start() + progress * r
}

// pub trait Shuffle {
//     fn shuffle(&mut self, iterations: usize, rng: &mut impl Rng);
// }

// impl <T> Shuffle for Vec<T> {
//     fn shuffle(&mut self, iterations: usize, rng: &mut impl Rng) {
//         for i in 0..iterations {
//             rng
//         }
//     }
// }