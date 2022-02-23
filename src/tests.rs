#[cfg(test)]
mod tests {
    use crate::services::card_builder;

    #[test]
    fn create_cards() {
        let cards = card_builder();
        assert_eq!(cards.len(), 52);
    }
}