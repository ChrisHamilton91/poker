use super::{
    FlushRanks, FourOfAKindRanks, FullHouseRanks,
    Hand::{self, *},
    OnePairRanks, ThreeOfAKindRanks, TwoPairRanks,
};
use crate::card::{
    Card,
    Rank::{self, *},
};
use std::cmp::Reverse;

pub fn best_hand(cards: [Card; 5]) -> Hand {
    let mut cards = cards.clone();
    cards.sort_by_key(|&(r, _)| Reverse(r));
    if is_royal_flush(cards) {
        return RoyalFlush;
    };
    if let Some(ranks) = is_straight_flush(cards) {
        return StraightFlush(ranks);
    };
    if let Some(ranks) = is_four_of_a_kind(cards) {
        return FourOfAKind(ranks);
    };
    if let Some(ranks) = is_full_house(cards) {
        return FullHouse(ranks);
    };
    if let Some(ranks) = is_flush(cards) {
        return Flush(ranks);
    };
    if let Some(ranks) = is_straight(cards) {
        return Straight(ranks);
    };
    if let Some(ranks) = is_three_of_a_kind(cards) {
        return ThreeOfAKind(ranks);
    }
    if let Some(ranks) = is_two_pair(cards) {
        return TwoPair(ranks);
    }
    if let Some(ranks) = is_one_pair(cards) {
        return OnePair(ranks);
    }
    HighCard(cards.map(|(r, _)| r))
}

fn is_royal_flush(cards: [Card; 5]) -> bool {
    is_royal(cards) && is_flush(cards).is_some()
}

fn is_royal(cards: [Card; 5]) -> bool {
    match cards {
        [(Ace, _), (King, _), (Queen, _), (Jack, _), (Ten, _)] => true,
        _ => false,
    }
}

fn is_straight_flush(cards: [Card; 5]) -> Option<Rank> {
    if is_straight(cards).is_some() && is_flush(cards).is_some() {
        let (r, _) = cards[0];
        return Some(r);
    };
    None
}

fn is_full_house(cards: [Card; 5]) -> Option<FullHouseRanks> {
    let [(a, _), (b, _), (c, _), (d, _), (e, _)] = cards;
    if a == b && a == c && d == e {
        return Some([a, d]);
    }
    if a == b && c == d && c == e {
        return Some([c, a]);
    }
    None
}

fn is_flush(cards: [Card; 5]) -> Option<FlushRanks> {
    let [(ra, sa), (rb, sb), (rc, sc), (rd, sd), (re, se)] = cards;
    if sa == sb && sa == sc && sa == sd && sa == se {
        return Some([ra, rb, rc, rd, re]);
    }
    None
}

fn is_four_of_a_kind(cards: [Card; 5]) -> Option<FourOfAKindRanks> {
    let [(a, _), (b, _), (c, _), (d, _), (e, _)] = cards;
    if a == b && a == c && a == d {
        return Some([a, e]);
    }
    if b == c && b == d && b == e {
        return Some([b, a]);
    }
    None
}

fn is_straight(cards: [Card; 5]) -> Option<Rank> {
    match cards {
        [(Ace, _), (Five, _), (Four, _), (Three, _), (Two, _)] => Some(Five),

        [(a, _), (b, _), (c, _), (d, _), (e, _)] => {
            let (ua, ub, uc, ud, ue) = (a as u8, b as u8, c as u8, d as u8, e as u8);
            match ua == (ub + 1) && ub == (uc + 1) && uc == (ud + 1) && ud == (ue + 1) {
                true => Some(a),
                false => None,
            }
        }
    }
}

fn is_three_of_a_kind(cards: [Card; 5]) -> Option<ThreeOfAKindRanks> {
    let [(a, _), (b, _), (c, _), (d, _), (e, _)] = cards;
    if a == b && a == c {
        return Some([a, d, e]);
    }
    if c == d && c == e {
        return Some([c, a, b]);
    }
    None
}

fn is_two_pair(cards: [Card; 5]) -> Option<TwoPairRanks> {
    let [(a, _), (b, _), (c, _), (d, _), (e, _)] = cards;
    if a == b && c == d {
        return Some([a, c, e]);
    }
    if a == b && d == e {
        return Some([a, d, c]);
    }
    if b == c && d == e {
        return Some([b, d, a]);
    }
    None
}

fn is_one_pair(cards: [Card; 5]) -> Option<OnePairRanks> {
    let [(a, _), (b, _), (c, _), (d, _), (e, _)] = cards;
    if a == b {
        return Some([a, c, d, e]);
    }
    if b == c {
        return Some([b, a, d, e]);
    }
    if c == d {
        return Some([c, a, b, e]);
    }
    if d == e {
        return Some([d, a, b, c]);
    }
    None
}
