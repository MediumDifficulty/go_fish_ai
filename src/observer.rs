use anyhow::{Result, anyhow};
use crate::probability::Probability;

const DECK_SIZE: usize = 13; // We only need to know the value of the card, not the suit.
const SUITS: usize = 4;
const STARTING_CARDS: usize = 7;

type KnownDeck = [usize; DECK_SIZE];

#[derive(Debug)]
pub struct GameObserver {
    deck: Deck,
    current_player: usize,
    players: Vec<Player>,
    own_deck: Option<KnownDeck>
}

impl GameObserver {
    /// Creates a new game observer
    /// 
    /// Set `own_deck` to `None` if you are not participating in the game as a player.
    pub fn new(players: usize, own_deck: Option<KnownDeck>) -> Self {
        let mut deck = Deck::new_full();

        if let Some(own_deck) = own_deck {
            deck.remove(&Deck::from_known(own_deck));
        }

        Self {
            // cards_remaining: DECK_SIZE - (7 * players),
            players: (0..players).map(|_| Player::with_starting_cards(&mut deck, STARTING_CARDS)).collect(),
            current_player: 0,
            deck,
            own_deck
        }
    }

    /// Observes a player ending their turn
    pub fn next(&mut self) {
        self.current_player = (self.current_player + 1) % self.get_players();
    }

    /// Observes a player picking up a card from the deck
    pub fn pickup(&mut self) {
        self.players[self.current_player].cards.add_unknown_from_other(&mut self.deck, 1);
    }

    /// Observes a player asking another player for a card
    ///
    /// # Arguments
    /// 
    /// * `player` - The id of the player which the current player asked
    /// * `card` - The id of the card which the player asked for
    /// * `amount_received` - The amount of cards the player received
    /// * `placed` - Whether the player placed a set
    pub fn query(&mut self, player: usize, card: usize, amount_received: usize, placed: bool) -> Result<()> {
        self.players[player].cards.cards[card] = Probability::Known(0);

        if placed {
            self.remove_all_cards_with_id(card);
            return Ok(());
        }

        match amount_received {
            2 => self.players[self.current_player].cards.cards[card] = Probability::Known(3),
            1 => self.players[self.current_player].cards.cards[card] = Probability::MoreThan(2),
            _ => return Err(anyhow!("Invalid amount of cards received"))
        }

        Ok(())
    }

    pub fn self_pickup(&mut self, card: usize) {
        if let Some(own_deck) = &mut self.own_deck {
            self.deck.remove_known_cards(card, 1);
            own_deck[card] += 1;
        }
    }

    pub fn self_query(&mut self, player: usize, card: usize, amount_received: usize) {
        if let Some(own_deck) = &mut self.own_deck {
            own_deck[card] += amount_received;
            self.players[player].cards.remove_known_cards(card, amount_received)
        }
    }

    pub fn get_deck(&self) -> Deck {
        self.deck.clone()
    }

    pub fn get_current_player(&self) -> usize {
        self.current_player
    }

    pub fn get_players(&self) -> usize {
        self.players.len()
    }

    pub fn get_own_deck(&self) -> Option<KnownDeck> {
        self.own_deck
    }

    fn remove_all_cards_with_id(&mut self, id: usize) {
        for player in self.players.iter_mut() {
            player.cards.cards[id] = Probability::Known(0);
        }

        self.deck.cards[id] = Probability::Known(0);
    }
}

#[derive(Debug, Clone)]
struct Player {
    cards: Deck,
}

impl Player {
    pub fn new() -> Self {
        Self {
            cards: Deck::new_empty(),
        }
    }

    pub fn with_starting_cards(deck: &mut Deck, amount: usize) -> Self {
        let mut cards = Deck::new_empty();
        cards.add_unknown_from_other(deck, amount);

        Self {
            cards,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Deck {
    cards: [Probability; DECK_SIZE]
}

impl Deck {
    pub fn new_empty() -> Self {
        Self {
            cards: [Probability::Unknown(0.0); DECK_SIZE]
        }
    }

    pub fn new_full() -> Self {
        Self {
            cards: [Probability::Unknown(SUITS as f32); DECK_SIZE]
        }
    }

    pub fn remove(&mut self, other: &Self) {
        for (i, card) in self.cards.iter_mut().enumerate() {
            *card -= other.cards[i];
        }
    }

    pub fn remove_known_cards(&mut self, card: usize, amount: usize) {
        self.cards[card] -= Probability::Known(amount)
    }

    fn nonzero_len(&self) -> usize {
        self.cards.iter().filter(|&x| x.value() > 0.0).count()
    }

    fn total(&self) -> f32 {
        self.cards.iter().map(|x| x.value()).sum()
    }

    pub fn add_unknown_from_other(&mut self, other: &mut Self, amount: usize) {
        let mut tmp = [0.0; DECK_SIZE];

        for (i, card) in self.cards.iter_mut().enumerate() {
            let value = (other.cards[i].value() / other.total()) * amount as f32;
            *card += Probability::Unknown(value);
            tmp[i] = value;
        }

        for (i, value) in tmp.iter().enumerate() {
            other.cards[i] -= Probability::Unknown(*value);
        }
    }

    pub fn from_known(known: KnownDeck) -> Self {
        Self {
            cards: known.map(Probability::Known)
        }
    }
}