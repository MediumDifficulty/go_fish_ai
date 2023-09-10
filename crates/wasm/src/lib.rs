use fish_core::{observer::{GameObserver, DECK_SIZE, KnownDeck}, game::{Bot, INPUTS_PER_UNKNOWN_CARD}, neural_network::NeuralNetwork, trainer::BotTrainer, util};
use rand::Rng;
use wasm_bindgen::prelude::*;
use rand::thread_rng;

extern "C" {

}

static mut BOT: Option<Bot> = None;

#[wasm_bindgen]
pub fn rand() -> String {
    thread_rng().gen_range(0..10).to_string()
}

#[wasm_bindgen]
pub fn init(game_size: usize, deck: Option<Vec<usize>>, position_in_game: usize) {
    console_error_panic_hook::set_once();

    let mut new_deck = [0; DECK_SIZE];

    if let Some(deck) = &deck {
        new_deck.copy_from_slice(deck);
    }
    
    let mut rng = thread_rng();

    unsafe {
        BOT = Some(Bot::new(GameObserver::new(game_size - 1, deck.map(|_| new_deck), position_in_game),
            &NeuralNetwork::new_rand(
            game_size * DECK_SIZE * INPUTS_PER_UNKNOWN_CARD + DECK_SIZE + 1,
            &[
                ((game_size - 1) * DECK_SIZE, util::ac_tanh),
                ((game_size - 1) * DECK_SIZE, util::ac_softmax),
            ],
            &mut rng
        )));
    }


    // let mut rng = thread_rng();
    // let mut trainer = BotTrainer::new(
    //     100,
    //     0.1,
    //     40,
    //     4,
    //     50,
    //     &mut rng
    // );

    // for i in 0..100 {
    //     trainer.step(&mut rng).unwrap();
    //     web_sys::console::log_1(&JsValue::from_str(format!("Game {i}: Top Fitness: {}", trainer.players.iter().max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap()).unwrap().fitness).as_str()));
    // }

    // BOT = Some(Bot::new(NeuralNetwork::new_rand(inputs, architecture, rng)));
}

#[wasm_bindgen]
pub fn get_observer() -> JsValue {
    match unsafe { BOT.as_ref() } {
        Some(bot) => serde_wasm_bindgen::to_value(&bot.observer).unwrap(),
        None => JsValue::NULL,
    }
}