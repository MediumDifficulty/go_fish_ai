use rand::Rng;
use crate::{observer::{GameObserver, KnownDeck, DECK_SIZE, STARTING_CARDS}, neural_network::NeuralNetwork, probability::{self, Probability}, util::lerp};

const INPUTS_PER_UNKNOWN_CARD: usize = 2;

pub struct NetworkTrainer {
    players: Vec<Bot>,
    deck: KnownDeck,
}

pub struct Bot {
    observer: GameObserver,
    network: NeuralNetwork,
}

pub enum Move {
    Pickup,
    Query(Query)
}

pub struct Query {
    pub player: usize,
    pub card: usize,
}

impl NetworkTrainer {
    pub fn new_rand(players: usize, rng: &mut impl Rng) -> Self {
        let mut deck = [4; DECK_SIZE];

        let mut players = (0..players)
            .map(|_| Bot::new_rand(GameObserver::new(players - 1, Some([0; DECK_SIZE])), players - 1, rng))
            .collect::<Vec<_>>();

        for i in 0..players.len() {
            
        }


        todo!()
        // Self {
        //     players: (0..players).map(|_| {
        //             // Pickup starting cards randomly
        //             let mut player_deck = [0; DECK_SIZE];
        //             for i in 0..STARTING_CARDS { // TODO: If player starts with complete set
        //                 let mut rand = rng.gen_range(0..DECK_SIZE);
        //                 while deck[rand] == 0 {
        //                     rand = rng.gen_range(0..DECK_SIZE);
        //                 }

        //                 player_deck[rand] += 1;
        //                 deck[rand] -= 1;
        //             }

        //             Bot::new_rand(GameObserver::new(players - 1, Some(player_deck)), players - 1, rng)
        //         }).collect::<Vec<_>>(),
        //     deck,
        // }
    }

    pub fn step(&mut self) {

    }
}

impl Bot {
    pub fn new_rand(observer: GameObserver, other_players: usize, rng: &mut impl Rng) -> Self {
        Self {
            network: NeuralNetwork::new_rand(
                (other_players + 1) * DECK_SIZE * INPUTS_PER_UNKNOWN_CARD + DECK_SIZE + 1,
                &[(other_players * DECK_SIZE, f32::tanh)],
                rng
            ),
            observer,
        }
    }

    pub fn eval(&mut self) -> Option<Move> {
        let outputs = self.network.eval(&self.observer_to_inputs()).unwrap().iter().enumerate().map(|(i, _)| i).collect::<Vec<_>>();

        for &output in outputs.iter() {
            let node_move = Move::from_id(self.observer.other_players.len(), output);
            if self.observer.move_is_legal(&node_move) {
                return Some(node_move);
            }
        }

        None
    }

    fn observer_to_inputs(&self) -> Vec<f32> {
        let mut inputs = Vec::with_capacity((self.observer.other_players.len() + 1) * DECK_SIZE * INPUTS_PER_UNKNOWN_CARD);

        // Add player cards
        for player in self.observer.other_players.iter() {
            inputs.append(&mut player.cards.cards.iter().flat_map(Self::weights_from_probability).collect());
        }

        // Add deck cards
        inputs.append(&mut self.observer.deck.cards.iter().flat_map(Self::weights_from_probability).collect());

        // Add own cards
        if let Some(own_deck) = self.observer.own_deck {
            inputs.append(&mut own_deck.iter().map(|card| lerp(-1.0..=1., *card as f32 / 3.0)).collect());
        } else {
            panic!("Own deck is None");
        }

        // Bias
        inputs.push(1.);

        inputs
    }

    fn weights_from_probability(probability: &Probability) -> Vec<f32> {
        match probability {
            Probability::Unknown(x) => vec![-1., lerp(-1.0..=1., x / 3.0)],
            Probability::Known(x) => vec![0., lerp(-1.0..=1., *x as f32 / 3.0)],
            Probability::MoreThan(x) => vec![1., lerp(-1.0..=1., *x as f32 / 3.0)],
        }
    }
}

impl Move {
    pub fn from_id(players: usize, id: usize) -> Self {
        if id <= players * DECK_SIZE {
            return Move::Query(Query { player: id / DECK_SIZE, card: id % DECK_SIZE });
        }

        return Move::Pickup;
    }
}