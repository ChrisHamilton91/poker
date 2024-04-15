use crate::card::{Rank::*, Suit::*};
use crate::hand::five_card_hand::best_hand;

#[test]
fn scratch() {
    println!(
        "{:?}",
        best_hand([
            (Jack, Clubs),
            (Queen, Clubs),
            (Eight, Clubs),
            (Ten, Clubs),
            (Nine, Clubs)
        ])
    )
}
