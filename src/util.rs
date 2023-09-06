use std::ops::RangeInclusive;

pub fn lerp(range: RangeInclusive<f32>, progress: f32) -> f32 { 
    let r = range.end() - range.start();
    range.start() + progress * r
}