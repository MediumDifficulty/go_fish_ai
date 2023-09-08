use go_fish_ai::{neural_network::NeuralNetwork, trainer::BotTrainer};
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    let mut trainer = BotTrainer::new(
        100,
        0.1,
        4,
        4,
        50,
        &mut rng
    );

    for i in 0..10 {
        trainer.step(&mut rng).unwrap();
        println!("{}", i);
    }
}
