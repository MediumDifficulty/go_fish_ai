use go_fish_ai::neural_network::NeuralNetwork;
use rand::thread_rng;

fn main() {
    // let mut observer = GameObserver::new(4, Some([0, 0, 0, 3, 0, 1, 2, 1, 0, 0, 0, 0, 0]));
    // println!("-----\n{:?}", observer);
    // observer.pickup();
    // println!("-----\n{:?}", observer);
    // observer.next();
    // observer.pickup();
    // println!("-----\n{:?}", observer);
    // observer.pickup();
    // println!("-----\n{:?}", observer);
    // observer.query(2, 2, 1, false);

    let mut nn = NeuralNetwork::new_rand(2, &[(2, f32::tanh), (1, f32::tanh)], &mut thread_rng());
    let nn2 = NeuralNetwork::new_rand(2, &[(2, f32::tanh), (1, f32::tanh)], &mut thread_rng());

    let nn3 = nn.cross(&nn2, &mut thread_rng());
    println!("nn: {:?}", nn);
    println!("nn eval: {:?}", nn.eval(&[0.5, 0.5]));
    println!("nn cross: {:?}", nn3);
}
