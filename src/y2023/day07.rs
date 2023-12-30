use std::collections::BTreeMap;

use nom::InputIter;

const CARDS: &[u8; 13] = b"23456789TJQKA";
const CARDS_P2: &[u8; 13] = b"J23456789TQKA";

pub fn p1(input: &str) -> usize {
    _solve(input, CARDS, false)
}

pub fn p2(input: &str) -> usize {
    _solve(input, CARDS_P2, true)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn _solve(input: &str, cards: &[u8; 13], jolly: bool) -> usize {
    let mut hands: Vec<_> = input
        .trim()
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(' ').unwrap();
            (
                (
                    parse_hand(hand.as_bytes(), jolly),
                    score_hand_with(hand.as_bytes(), cards),
                ),
                bid.parse::<usize>().unwrap(),
            )
        })
        .collect();

    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum()
}

fn parse_hand(s: &[u8], jolly: bool) -> Hand {
    let mut cards = s.iter().fold(BTreeMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    let j_count = if jolly { cards.remove(&b'J') } else { None };

    let mut groups = cards.into_iter().fold(BTreeMap::new(), |mut acc, (_, n)| {
        *acc.entry(n).or_insert(0) += 1;
        acc
    });

    if let Some(j) = j_count {
        let partial_best = *groups.keys().max().unwrap_or(&0);
        if let Some(1) = groups.get(&partial_best) {
            groups.remove(&partial_best);
        } else {
            groups.entry(partial_best).and_modify(|e| *e -= 1);
        }
        *groups.entry(j + partial_best).or_default() = 1;
    }

    if groups.contains_key(&5) {
        Hand::FiveOfAKind
    } else if groups.contains_key(&4) {
        Hand::FourOfAKind
    } else if groups.contains_key(&3) {
        if groups.contains_key(&2) {
            Hand::FullHouse
        } else {
            Hand::ThreeOfAKind
        }
    } else {
        match groups.get(&2) {
            Some(2) => Hand::TwoPair,
            Some(1) => Hand::OnePair,
            _ => Hand::HighCard,
        }
    }
}

fn score_hand_with(s: &[u8], cards: &[u8; 13]) -> Vec<usize> {
    let secondary_score = s
        .iter()
        .map(|x| cards.position(|c| c == *x).unwrap())
        .collect();

    secondary_score
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 6440);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 5905);
    }
}

aoc::solve!(2023, 7, p1, p2);
