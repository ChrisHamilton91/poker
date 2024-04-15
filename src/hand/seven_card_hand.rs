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

pub fn best_hand(cards: [Card; 7]) -> Hand {
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
    let ranks: [Rank; 5] = cards.map(|(r, _)| r)[0..5].try_into().unwrap();
    HighCard(ranks)
}

fn is_royal_flush(cards: [Card; 7]) -> bool {
    for i in 0..=2 {
        let (ra, sa) = cards[i];
        if ra != Ace {
            continue;
        }
        let (mut king, mut queen, mut jack, mut ten) = (false, false, false, false);
        for j in (i + 1)..7 {
            match cards[j] {
                (King, sb) if sa == sb => king = true,
                (Queen, sb) if sa == sb => queen = true,
                (Jack, sb) if sa == sb => jack = true,
                (Ten, sb) if sa == sb => ten = true,
                _ => (),
            }
            if king & queen & jack & ten {
                return true;
            }
        }
    }
    false
}

fn is_straight_flush(cards: [Card; 7]) -> Option<Rank> {
    for i in 0..=2 {
        let (ra, sa) = cards[i];
        let ua = ra as u8;
        let mut count = 1;
        for j in (i + 1)..=6 {
            let (rb, sb) = cards[j];
            let ub = rb as u8;
            if sa == sb && ua == (ub + count) {
                count += 1;
            }
            if count == 5 {
                return Some(ra);
            }
        }
    }
    is_wheel_flush(cards)
}

fn is_wheel_flush(cards: [Card; 7]) -> Option<Rank> {
    for i in 0..=2 {
        let (ra, sa) = cards[i];
        if ra != Ace {
            continue;
        }
        let (mut two, mut three, mut four, mut five) = (false, false, false, false);
        for j in (i + 1)..7 {
            match cards[j] {
                (Two, sb) if sa == sb => two = true,
                (Three, sb) if sa == sb => three = true,
                (Four, sb) if sa == sb => four = true,
                (Five, sb) if sa == sb => five = true,
                _ => (),
            }
            if two & three & four & five {
                return Some(Five);
            }
        }
    }
    None
}

fn is_full_house(cards: [Card; 7]) -> Option<FullHouseRanks> {
    for i in 0..=4 {
        let ((a, _), (b, _), (c, _)) = (cards[i], cards[i + 1], cards[i + 2]);
        if (a, a) != (b, c) {
            continue;
        }
        let s1 = &cards[0..i];
        let s2 = &cards[(i + 3)..];
        let s3 = [s1, s2].concat();
        for j in 0..(s3.len() - 1) {
            let ((d, _), (e, _)) = (s3[j], s3[j + 1]);
            if d != e {
                continue;
            }
            return Some([a, d]);
        }
    }
    None
}

fn is_flush(cards: [Card; 7]) -> Option<FlushRanks> {
    for i in 0..=2 {
        let (ra, sa) = cards[i];
        let mut ranks = vec![ra];
        for j in (i + 1)..=6 {
            let (rb, sb) = cards[j];
            if sa == sb {
                ranks.push(rb)
            }
            if ranks.len() == 5 {
                return Some(ranks.try_into().unwrap());
            }
        }
    }
    None
}

fn is_four_of_a_kind(cards: [Card; 7]) -> Option<FourOfAKindRanks> {
    for i in 0..=3 {
        let ((a, _), (b, _), (c, _), (d, _)) = (cards[i], cards[i + 1], cards[i + 2], cards[i + 3]);
        if (a, a, a) != (b, c, d) {
            continue;
        }
        let (k, _) = match i {
            0 => cards[4],
            _ => cards[0],
        };
        return Some([a, k]);
    }
    None
}

fn is_straight(cards: [Card; 7]) -> Option<Rank> {
    for i in 0..=2 {
        let (ra, _) = cards[i];
        let ua = ra as u8;
        let mut count = 1;
        for j in (i + 1)..=6 {
            let (rb, _) = cards[j];
            let ub = rb as u8;
            if ua == (ub + count) {
                count += 1;
            }
            if count == 5 {
                return Some(ra);
            }
        }
    }
    is_wheel(cards)
}

fn is_wheel(cards: [Card; 7]) -> Option<Rank> {
    let (mut ace, mut two, mut three, mut four, mut five) = (false, false, false, false, false);
    for (r, _) in cards {
        match r {
            Ace => ace = true,
            Two => two = true,
            Three => three = true,
            Four => four = true,
            Five => five = true,
            _ => (),
        }
        if ace & two & three & four & five {
            return Some(Five);
        }
    }
    None
}

fn is_three_of_a_kind(cards: [Card; 7]) -> Option<ThreeOfAKindRanks> {
    for i in 0..=4 {
        let ((a, _), (b, _), (c, _)) = (cards[i], cards[i + 1], cards[i + 2]);
        if (a, a) != (b, c) {
            continue;
        }
        let [(k1, _), (k2, _)] = match i {
            0 => [cards[3], cards[4]],
            1 => [cards[0], cards[4]],
            _ => [cards[0], cards[1]],
        };
        return Some([a, k1, k2]);
    }
    None
}

fn is_two_pair(cards: [Card; 7]) -> Option<TwoPairRanks> {
    for i in 0..=3 {
        let ((a, _), (b, _)) = (cards[i], cards[i + 1]);
        if a != b {
            continue;
        }
        for j in (i + 2)..=5 {
            let ((c, _), (d, _)) = (cards[j], cards[j + 1]);
            if c != d {
                continue;
            }
            let (k, _) = match (i, j) {
                (0, 2) => cards[4],
                (0, 3..) => cards[2],
                _ => cards[0],
            };
            return Some([a, c, k]);
        }
    }
    None
}

fn is_one_pair(cards: [Card; 7]) -> Option<OnePairRanks> {
    for i in 0..=5 {
        let ((a, _), (b, _)) = (cards[i], cards[i + 1]);
        if a != b {
            continue;
        }
        let [(k1, _), (k2, _), (k3, _)] = match i {
            0 => [cards[2], cards[3], cards[4]],
            1 => [cards[0], cards[3], cards[4]],
            2 => [cards[0], cards[1], cards[4]],
            _ => [cards[0], cards[1], cards[2]],
        };
        return Some([a, k1, k2, k3]);
    }
    None
}
