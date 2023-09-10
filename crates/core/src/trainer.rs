use rand::{Rng, seq::SliceRandom};
use anyhow::Result;
use rayon::{slice::ParallelSliceMut, prelude::ParallelIterator};

use crate::{game::{BotGame, INPUTS_PER_UNKNOWN_CARD}, neural_network::NeuralNetwork, observer::DECK_SIZE, util};

pub struct BotTrainer {
    pub players: Vec<Agent>,
    pub agents: usize,
    pub reproduction_fraction: f32,
    pub evaluation_games: usize,
    pub game_size: usize,
    pub max_turns: usize,
}

#[derive(Clone)]
pub struct Agent {
    pub network: NeuralNetwork,
    pub fitness: f32
}

            // network: NeuralNetwork::new_rand(
            //     (other_players + 1) * DECK_SIZE * INPUTS_PER_UNKNOWN_CARD + DECK_SIZE + 1,
            //     &[(other_players * DECK_SIZE, f32::tanh)],
            //     rng
            // ),

impl BotTrainer {
    pub fn new(agents: usize, reproduction_fraction: f32, evaluation_games: usize, game_size: usize, max_turns: usize, rng: &mut impl Rng) -> Self {
        Self {
            players: (0..agents).map(|_| Agent {
                fitness: 0.0,
                network: NeuralNetwork::new_rand(
                    game_size * DECK_SIZE * INPUTS_PER_UNKNOWN_CARD + DECK_SIZE + 1,
                    &[
                        ((game_size - 1) * DECK_SIZE, util::ac_tanh),
                        ((game_size - 1) * DECK_SIZE, util::ac_softmax),
                    ],
                    rng
                )
            }).collect(),
            agents,
            reproduction_fraction,
            evaluation_games,
            game_size,
            max_turns,
        }
    }

    pub fn step(&mut self, rng: &mut impl Rng) -> Result<()> {
        // Selection
        self.players.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

        let reproductive_agents = (self.agents as f32 * self.reproduction_fraction) as usize;

        // Crossover + Mutation
        for i in reproductive_agents..self.agents {
            let parent_1 = rng.gen_range(0..reproductive_agents);
            let mut parent_2 = rng.gen_range(0..reproductive_agents);

            while parent_2 == parent_1 {
                parent_2 = rng.gen_range(0..reproductive_agents);
            }

            let parent_1 = &self.players[parent_1];
            let parent_2 = &self.players[parent_2];

            self.players[i].network = parent_1.network.cross(&parent_2.network, rng);
            self.players[i].network.mutate(rng);
        }

        // Reset fitness
        for player in self.players.iter_mut() {
            player.fitness = 0.;
        }

        // Evaluation
        for _ in 0..self.evaluation_games {
            self.players.shuffle(rng);
            self.players.chunks_mut(self.game_size).for_each(|chunk| {
                let mut rng = rand::thread_rng();
                let mut game = BotGame::new_rand(chunk.iter().map(|a| a.network.clone()).collect::<Vec<_>>().as_slice(), &mut rng);

                for _ in 0..self.max_turns {
                    game.step(&mut rng).unwrap();
                    if game.over {
                        break;
                    }
                }

                for (j, player) in chunk.iter_mut().enumerate() {
                    player.fitness += game.players[j].number_placed as f32;
                }
            })
        }

        Ok(())
    }
}

