#[cfg(test)]
mod tests {
    use crate::services::card_builder;
    use crate::shuffles::hand_shuffle::shuffle;

    #[test]
    fn create_cards() {
        let cards = card_builder();
        assert_eq!(cards.len(), 52);
    }

    #[test]
    fn hand_shuffle() {
        let original = card_builder();
        let mut for_shuffle = original.to_vec();
        shuffle(&mut for_shuffle);
        assert_ne!(original, for_shuffle);
    }
}
