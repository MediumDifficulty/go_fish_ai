use anyhow::{Result, Context};
use rand::Rng;
use rand_distr::StandardNormal;

const MUTATION_RATE: f32 = 0.1;

type Activation = fn(&mut [Neuron]);

#[derive(Debug, Clone)]
pub struct NeuralNetwork {
    layers: Vec<Layer>,
}

#[derive(Debug, Clone)]
struct Layer {
    neurons: Vec<Neuron>,
    activation: Activation,
}

#[derive(Debug, Clone)]
pub struct Neuron {
    weights: Vec<f32>,
    bias: f32,
    pub value: f32,
}

impl NeuralNetwork {
    pub fn eval(&mut self, inputs: &[f32]) -> Result<Vec<f32>> {
        for i in 0..self.layers.len() {
            let inputs = if i > 0 {
                self.layers[i - 1].neurons.iter().map(|neuron| neuron.value).collect::<Vec<f32>>()
            } else {
                inputs.to_vec()
            };

            self.layers[i].eval(inputs.as_slice());
        }

        Ok(self.layers.last().context("Invalid network size")?.neurons.iter().map(|neuron| neuron.value).collect())
    }

    pub fn new_rand(inputs: usize, architecture: &[(usize, Activation)], rng: &mut impl Rng) -> Self {
        Self {
            layers: architecture.iter()
                .enumerate()
                .map(|(i, (size, activation))| Layer::new_rand(*size, if i > 0 {
                    architecture[i - 1].0
                } else { inputs }, rng, *activation))
                .collect(),
        }
    }

    pub fn cross(&self, other: &Self, rng: &mut impl Rng) -> Self {
        // Performs a uniform crossover of two networks
        Self {
            layers: self.layers.iter()
                .enumerate()
                .map(|(i, layer)|
                    Layer { activation: layer.activation, neurons: layer.neurons.iter()
                        .enumerate()
                        .map(|(j, neuron)| Neuron {
                            weights: neuron.weights.iter()
                            .enumerate()
                            .map(|(k, weight)| if rng.gen_bool(0.5) {
                                        *weight
                                    } else {
                                        other.layers[i].neurons[j].weights[k]
                                    }).collect(),
                                bias: if rng.gen_bool(0.5) {
                                    neuron.bias
                                } else {
                                    other.layers[i].neurons[j].bias
                                },
                                value: 0.0
                            }).collect()
                    }).collect()
        }
    }

    pub fn mutate(&mut self, rng: &mut impl Rng) {
        for layer in self.layers.iter_mut() {
            for neuron in layer.neurons.iter_mut() {
                for weight in neuron.weights.iter_mut() {
                    if rng.gen_bool(MUTATION_RATE as f64) {
                        *weight += rng.sample::<f32, _>(StandardNormal);
                    }
                }
            }
        }
    }
}

impl Layer {
    pub fn eval(&mut self, inputs: &[f32]) {
        for neuron in self.neurons.iter_mut() {
            neuron.eval(inputs, self.activation);
        }

        (self.activation)(&mut self.neurons);
    }

    pub fn new_rand(size: usize, prev_size: usize, rng: &mut impl Rng, activation: Activation) -> Self {
        Self {
            neurons: (0..size).map(|_| Neuron::new_rand(prev_size, rng)).collect(),
            activation
        }
    }
}

impl Neuron {
    pub fn eval(&mut self, inputs: &[f32], activation: Activation) {
        self.value = inputs
            .iter()
            .enumerate()
            .map(|(i, &value)| value * self.weights[i] + self.bias)
            .sum()
    }

    pub fn new_rand(prev_layer_size: usize, rand: &mut impl Rng) -> Self {
        Self {
            value: 0.,
            bias: rand.gen::<f32>() * 2. - 1.,
            weights: (0..prev_layer_size).map(|_| rand.gen::<f32>() * 2. - 1.).collect(),
        }
    }
}
