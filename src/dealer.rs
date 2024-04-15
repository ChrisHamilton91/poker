use crate::card::{deck_52, Card};
use rand::Rng;

#[derive(Debug)]
pub struct Dealer {
    deck: Vec<Card>,
}

impl Dealer {
    pub fn new() -> Self {
        let mut dealer = Self { deck: deck_52() };
        dealer.shuffle();
        dealer
    }

    pub fn shuffle(&mut self) {
        for i in 0..self.deck.len() - 2 {
            let j = rand::thread_rng().gen_range(i..self.deck.len());
            self.deck.swap(i, j);
        }
    }

    pub fn deal(&mut self, n: usize) -> Vec<Card> {
        let mut dealt_cards: Vec<Card> = Vec::new();
        for _ in 0..n {
            match self.deck.pop() {
                Some(card) => dealt_cards.push(card),
                None => panic!("No more cards!"),
            }
        }
        dealt_cards
    }
}
