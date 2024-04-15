use std::cmp::min;

use crate::card::Card;

#[derive(Debug, Clone)]
pub struct Player {
    pub cards: Vec<Card>,
    pub chips: u64,
    pub bet: u64,
}

impl Player {
    pub fn new(chips: u64) -> Self {
        Self {
            cards: Vec::new(),
            chips,
            bet: 0,
        }
    }

    pub fn bet(&mut self, mut bet: u64) {
        bet = min(bet, self.chips);
        self.chips -= bet;
        self.bet += bet;
    }

    pub fn take_cards(&mut self, cards: Vec<Card>) {
        self.cards = cards;
    }

    pub fn muck(&mut self) {
        self.cards.clear()
    }
}
