use anyhow::anyhow;
use anyhow::Result;
use rand::Rng;
use crate::{observer::{GameObserver, KnownDeck, DECK_SIZE, STARTING_CARDS}, neural_network::NeuralNetwork, probability::Probability, util::lerp};

const INPUTS_PER_UNKNOWN_CARD: usize = 2;

pub struct BotGame {
    players: Vec<Bot>,
    deck: KnownDeck,
    current_player: usize,
    over: bool,
}

pub struct Bot {
    observer: GameObserver,
    network: NeuralNetwork,
    id: usize,
}

pub enum Move {
    Pickup,
    Query(Query)
}

pub struct Query {
    pub player: usize,
    pub card: usize,
}

impl BotGame {
    pub fn new_rand(players: usize, rng: &mut impl Rng) -> Self {
        let (bots, deck) = loop {
            let mut deck = [4; DECK_SIZE];

            let bots = (0..players).map(|i| {
                    let mut player_deck = [0; DECK_SIZE];
                    for _ in 0..STARTING_CARDS {
                        let mut rand = rng.gen_range(0..DECK_SIZE);
                        while deck[rand] == 0 {
                            rand = rng.gen_range(0..DECK_SIZE);
                        }
        
                        player_deck[rand] += 1;
                        deck[rand] -= 1;
                    }

                    Bot::new_rand(GameObserver::new(players - 1, Some(player_deck)), players - 1, i, rng)
                }).collect::<Vec<_>>();

            // This is very lazy and inefficient, I should make it so games can start with some players already have placed cards on the table
            // but I don't see how it could affect training and this is easier
            if !bots.iter().any(|bot| bot.observer.own_deck.unwrap().iter().any(|&card| card == 4)) {
                break (bots, deck);
            }
        };

        Self {
            players: bots,
            deck,
            current_player: 0,
            over: false,
        }
    }

    pub fn step(&mut self, rng: &mut impl Rng) -> Result<()> {
        let bot_move = match self.players[self.current_player].eval() {
            None => {
                self.over = true;
                return Ok(());
            },
            Some(m) => m
        };

        match bot_move {
            Move::Pickup => {
                let card = rand_card_from_deck(&self.deck, rng)?;

                self.players[self.current_player].observer.self_pickup(card);

                for (i, player) in self.players.iter_mut().enumerate().filter(|(i, _)| *i != self.current_player) {
                    player.observer.pickup(i - (self.current_player >= player.id) as usize);
                }
            },
            Move::Query(query) => {
                let transfer_amount = self.players[query.player].observer.own_deck.unwrap()[query.card];
                let placed = self.players[self.current_player].observer.own_deck.unwrap()[query.card] + transfer_amount >= 4;
                
                // Inform the current player
                self.players[self.current_player].observer.self_query(query.player, query.card, transfer_amount);

                let p = &mut self.players[query.player];
                // Inform the player who the current player asked
                p.observer.self_give_all(self.current_player - (self.current_player >= p.id) as usize, query.card)?;

                // Inform the other players not involved in the transaction
                for (i, player) in self.players.iter_mut().enumerate().filter(|(i, _)| *i != self.current_player && *i != query.player) {
                    player.observer.query(self.current_player - (i >= player.id) as usize, query.player - (i >= player.id) as usize, query.card, transfer_amount, placed)?;
                }
            },
        }

        self.current_player += 1;
        self.current_player %= self.players.len();

        Ok(())
    }
}

fn rand_card_from_deck(deck: &[usize], rng: &mut impl Rng) -> Result<usize> {
    let total = deck.iter().sum::<usize>();
    let rand = rng.gen_range(1..=total);
    let mut sum = 0;

    for (i, &count) in deck.iter().enumerate() {
        sum += count;
        if rand <= sum {
            return Ok(i);
        }
    }

    Err(anyhow!("Cannot get random card from empty deck"))
}

impl Bot {
    pub fn new_rand(observer: GameObserver, other_players: usize, id: usize, rng: &mut impl Rng) -> Self {
        Self {
            network: NeuralNetwork::new_rand(
                (other_players + 1) * DECK_SIZE * INPUTS_PER_UNKNOWN_CARD + DECK_SIZE + 1,
                &[(other_players * DECK_SIZE, f32::tanh)],
                rng
            ),
            observer,
            id,
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

        Move::Pickup
    }
}