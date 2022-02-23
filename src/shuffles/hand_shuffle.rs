use crate::models::card::Card;
use rand::Rng;

pub fn shuffle(cards: &mut Vec<Card>) {
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let part_size = rng.gen_range(0..(cards.len() / 2) as usize);

        let index = rng.gen_range(0..(cards.len() - part_size) as usize);

        let sub_range = cards
            .iter()
            .skip(index)
            .take(part_size)
            .map(|x| x.to_owned())
            .collect::<Vec<Card>>();

        cards.drain(index..part_size + index);

        cards.extend(sub_range);
    }
}
