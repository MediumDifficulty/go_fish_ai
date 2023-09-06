#[cfg(test)]
mod tests {
    use go_fish_ai::observer::Deck;

    #[test]
    fn new_full() {
        let deck = Deck::new_full();

        assert_eq!(deck.size as f32, deck.total());
    }

    #[test]
    fn new_empty() {
        let deck = Deck::new_empty();

        
    }
}