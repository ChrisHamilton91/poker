use self::Hand::*;
use crate::card::Rank;
use std::cmp::Ordering::{self, *};

pub mod five_card_hand;
pub mod seven_card_hand;

pub type HighCardRanks = [Rank; 5];
pub type OnePairRanks = [Rank; 4];
pub type TwoPairRanks = [Rank; 3];
pub type ThreeOfAKindRanks = [Rank; 3];
pub type FlushRanks = [Rank; 5];
pub type FullHouseRanks = [Rank; 2];
pub type FourOfAKindRanks = [Rank; 2];

#[derive(Eq, Debug)]
/// Hands with relevant ranks.
///
/// Ranks are compared from left to right.
///
/// This means the most important ranks need to come first.
///
/// For example, in one pair the rank of the pair comes first, then kickers from highest to lowest.
///
/// In two pair the higher pair rank comes first, then the lower pair, then the kicker.
///
/// In a full house the 3 of a kind rank comes first, then the pair.
pub enum Hand {
    HighCard(HighCardRanks),
    OnePair(OnePairRanks),
    TwoPair(TwoPairRanks),
    ThreeOfAKind(ThreeOfAKindRanks),
    Straight(Rank),
    Flush(FlushRanks),
    FullHouse(FullHouseRanks),
    FourOfAKind(FourOfAKindRanks),
    StraightFlush(Rank),
    RoyalFlush,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(&other) == Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (HighCard(a), HighCard(b)) => cmp_ranks(a, b),
            (HighCard(_), _) => Less,
            (_, HighCard(_)) => Greater,
            (OnePair(a), OnePair(b)) => cmp_ranks(a, b),
            (OnePair(_), _) => Less,
            (_, OnePair(_)) => Greater,
            (TwoPair(a), TwoPair(b)) => cmp_ranks(a, b),
            (TwoPair(_), _) => Less,
            (_, TwoPair(_)) => Greater,
            (ThreeOfAKind(a), ThreeOfAKind(b)) => cmp_ranks(a, b),
            (ThreeOfAKind(_), _) => Less,
            (_, ThreeOfAKind(_)) => Greater,
            (Straight(a), Straight(b)) => a.cmp(b),
            (Straight(_), _) => Less,
            (_, Straight(_)) => Greater,
            (Flush(a), Flush(b)) => cmp_ranks(a, b),
            (FullHouse(a), FullHouse(b)) => cmp_ranks(a, b),
            (Flush(_), _) => Less,
            (_, Flush(_)) => Greater,
            (FullHouse(_), _) => Less,
            (_, FullHouse(_)) => Greater,
            (FourOfAKind(a), FourOfAKind(b)) => cmp_ranks(a, b),
            (FourOfAKind(_), _) => Less,
            (_, FourOfAKind(_)) => Greater,
            (StraightFlush(a), StraightFlush(b)) => a.cmp(b),
            (StraightFlush(_), _) => Less,
            (_, StraightFlush(_)) => Greater,
            (RoyalFlush, RoyalFlush) => Equal,
        }
    }
}

fn cmp_ranks<const N: usize>(a: &[Rank; N], b: &[Rank; N]) -> Ordering {
    for i in 0..a.len() {
        if a[i] < b[i] {
            return Less;
        }
        if a[i] > b[i] {
            return Greater;
        }
    }
    Equal
}
