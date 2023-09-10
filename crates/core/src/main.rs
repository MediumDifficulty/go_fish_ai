use fish_core::{neural_network::NeuralNetwork, trainer::BotTrainer};
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    let mut trainer = BotTrainer::new(
        100,
        0.1,
        40,
        4,
        50,
        &mut rng
    );

    for i in 0..10000 {
        trainer.step(&mut rng).unwrap();
        println!("Game {i}: Top Fitness: {}", trainer.players.iter().max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap()).unwrap().fitness);
    }
}
