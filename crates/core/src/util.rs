use std::ops::RangeInclusive;

use rand::Rng;

use crate::neural_network::Neuron;

pub fn lerp(range: RangeInclusive<f32>, progress: f32) -> f32 { 
    let r = range.end() - range.start();
    range.start() + progress * r
}

pub fn ac_softmax(inputs: &mut [Neuron]) {
    let max_val = inputs.iter().map(|n| n.value).fold(f32::NEG_INFINITY, f32::max);
    let exp_sum = inputs.iter().map(|x| (x.value - max_val).exp()).sum::<f32>();

    for neuron in inputs.iter_mut() {
        neuron.value = (neuron.value - max_val).exp() / exp_sum;
    }
}

pub fn ac_tanh(inputs: &mut [Neuron]) {
    for neuron in inputs.iter_mut() {
        neuron.value = neuron.value.tanh();
    }
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