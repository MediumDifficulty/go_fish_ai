use anyhow::{Result, anyhow};
use crate::{probability::Probability, game::Move};

/// The number of different cards
pub const DECK_SIZE: usize = 13; // We only need to know the value of the card, not the suit.
/// The number of card suits
pub const SUITS: usize = 4;
/// The number of cards a player should start with
pub const STARTING_CARDS: usize = 7;

/// Represents a deck where we know what each card is unlike [`Deck`]
pub type KnownDeck = [usize; DECK_SIZE];


/// Used for estimating what other players could have based on the observing player's observations
#[derive(Debug, Clone)]
pub struct GameObserver {
    pub deck: Deck,
    pub other_players: Vec<Player>,
    pub own_deck: Option<KnownDeck>,
    pub id: usize,
}

impl GameObserver {
    /// Creates a new game observer
    /// 
    /// Set `own_deck` to `None` if you are not participating in the game as a player.
    pub fn new(other_players: usize, own_deck: Option<KnownDeck>, id: usize) -> Self {
        let mut deck = Deck::new_full();

        if let Some(own_deck) = own_deck {
            deck.remove(&Deck::from_known(own_deck));
        }

        Self {
            // cards_remaining: DECK_SIZE - (7 * players),
            other_players: (0..other_players).map(|_| Player::with_starting_cards(&mut deck, STARTING_CARDS)).collect(),
            deck,
            own_deck,
            id
        }
    }

    /// Observes a player picking up a card from the deck
    pub fn pickup(&mut self, player: usize) {
        self.other_players[player].cards.add_unknown_from_other(&mut self.deck, 1);
    }

    /// Observes a player asking another player for a card
    ///
    /// # Arguments
    /// 
    /// * `player` - The id of the player which the current player asked
    /// * `card` - The id of the card which the player asked for
    /// * `amount_received` - The amount of cards the player received
    /// * `placed` - Whether the player placed a set
    pub fn query(&mut self, current_player: usize, player: usize, card: usize, amount_received: usize, placed: bool) -> Result<()> {
        self.other_players[player].cards.cards[card] = Probability::Known(0);

        if placed {
            self.remove_all_cards_with_id(card);
            self.deck.size -= SUITS;
            return Ok(());
        }

        self.other_players[current_player].cards.size += amount_received;

        match amount_received {
            2 => self.other_players[current_player].cards.cards[card] = Probability::Known(3),
            1 => self.other_players[current_player].cards.cards[card] = Probability::MoreThan(2),
            _ => return Err(anyhow!("Invalid amount of cards received"))
        }

        Ok(())
    }

    /// Observes a player placing a set
    pub fn place(&mut self, card: usize) {
        self.remove_all_cards_with_id(card);
        self.deck.remove_known_cards(card, SUITS);
    }

    /// Observes a player picking up a card from the deck
    pub fn self_pickup(&mut self, card: usize) {
        if let Some(own_deck) = &mut self.own_deck {
            self.deck.remove_known_cards(card, 1);
            own_deck[card] += 1;

            if own_deck[card] == 4 {
                own_deck[card] = 0;
            }
        }
    }

    /// Observes the observing player asking another player for a card
    pub fn self_query(&mut self, player: usize, card: usize, amount_received: usize) {
        if let Some(own_deck) = &mut self.own_deck {
            own_deck[card] += amount_received;
            self.other_players[player].cards.remove_known_cards(card, amount_received);

            if own_deck[card] == 4 {
                own_deck[card] = 0;
            }
        }
    }

    /// Observes the observing player giving a set to another player
    pub fn self_give_all(&mut self, player: usize, card: usize) -> Result<()> {
        if let Some(own_deck) = &mut self.own_deck {
            match own_deck[card] {
                1 => self.other_players[player].cards.cards[card] = Probability::MoreThan(2),
                2 => self.other_players[player].cards.cards[card] = Probability::Known(3),
                3 => self.other_players[player].cards.cards[card] = Probability::Known(0),
                _ => return Err(anyhow!("Invalid amount of cards to give"))
            }

            own_deck[card] = 0;
        }

        Ok(())
    }

    pub fn move_is_legal(&self, m: &Move) -> bool {
        match m {
            Move::Pickup => self.deck.total() > 0.,
            Move::Query(q) => q.player != self.id && self.own_deck.is_some_and(|own_deck| own_deck[q.card] > 0),
        }
    }

    fn remove_all_cards_with_id(&mut self, id: usize) {
        for player in self.other_players.iter_mut() {
            player.cards.cards[id] = Probability::Known(0);
        }

        self.deck.cards[id] = Probability::Known(0);
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    pub cards: Deck,
}

impl Player {
    pub fn with_starting_cards(deck: &mut Deck, amount: usize) -> Self {
        let mut cards = Deck::new_empty();
        cards.add_unknown_from_other(deck, amount);

        Self {
            cards,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            cards: Deck::new_empty()
        }
    }
}

/// Represents a collection of cards based on their probability
#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: [Probability; DECK_SIZE],
    pub size: usize,
}

impl Deck {
    /// Constructs a new empty `Deck`
    pub fn new_empty() -> Self {
        Self {
            cards: [Probability::Unknown(0.0); DECK_SIZE],
            size: 0,
        }
    }

    /// Constructs a new full `Deck`
    pub fn new_full() -> Self {
        Self {
            cards: [Probability::Unknown(SUITS as f32); DECK_SIZE],
            size: SUITS * DECK_SIZE,
        }
    }

    /// Removes all cards from an other deck from this `Deck`
    pub fn remove(&mut self, other: &Self) {
        for (i, card) in self.cards.iter_mut().enumerate() {
            *card -= other.cards[i];
        }

        self.size -= other.size;
    }

    /// Removes an amount of a certain card from this `Deck`
    pub fn remove_known_cards(&mut self, card: usize, amount: usize) {
        self.cards[card] -= Probability::Known(amount);
        self.size -= amount;
    }

    fn nonzero_len(&self) -> usize {
        self.cards.iter().filter(|&x| x.value() > 0.0).count()
    }

    /// Returns the total value of all card probabilities
    pub fn total(&self) -> f32 {
        self.cards.iter().map(|x| x.value()).sum()
    }

    /// Adds an unknown amount of a certain card to this `Deck` from an other `Deck`
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

        self.size += amount;
    }

    /// Constructs a new Deck from a known `Deck`
    pub fn from_known(known: KnownDeck) -> Self {
        Self {
            cards: known.map(Probability::Known),
            size: known.iter().sum(),
        }
    }
}