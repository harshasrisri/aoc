use itertools::Itertools;
use std::str::FromStr;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    HighCard(Vec<u32>),
    OnePair(Vec<u32>),
    TwoPair(Vec<u32>),
    ThreeOfAKind(Vec<u32>),
    FullHouse(Vec<u32>),
    FourOfAKind(Vec<u32>),
    FiveOfAKind(Vec<u32>),
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hand = s
            .chars()
            .filter_map(|c| match c {
                d if d.is_ascii_digit() => d.to_digit(10),
                'T' => Some(10),
                'J' => Some(11),
                'Q' => Some(12),
                'K' => Some(13),
                'A' => Some(14),
                _ => None,
            })
            .collect::<Vec<_>>();

        let counts = hand
            .iter()
            .counts()
            .into_values()
            .sorted_by(|a, b| b.cmp(a))
            .collect_vec();
        match *counts.as_slice() {
            [5] => Ok(Hand::FiveOfAKind(hand)),
            [4, 1] => Ok(Hand::FourOfAKind(hand)),
            [3, 2] => Ok(Hand::FullHouse(hand)),
            [3, 1, 1] => Ok(Hand::ThreeOfAKind(hand)),
            [2, 2, 1] => Ok(Hand::TwoPair(hand)),
            [2, 1, 1, 1] => Ok(Hand::OnePair(hand)),
            [1, 1, 1, 1, 1] => Ok(Hand::HighCard(hand)),
            _ => Err(format!("Invalid hand: {hand:?}")),
        }
    }
}

impl Hand {
    fn jokerize(self) -> Self {
        let hand = match &self {
            Hand::HighCard(h) => h,
            Hand::OnePair(h) => h,
            Hand::TwoPair(h) => h,
            Hand::ThreeOfAKind(h) => h,
            Hand::FullHouse(h) => h,
            Hand::FourOfAKind(h) => h,
            Hand::FiveOfAKind(h) => h,
        };

        let js = hand.iter().filter(|score| **score == 11).count();
        let j_hand = hand
            .iter()
            .map(|&score| if score == 11 { 1 } else { score })
            .collect::<Vec<_>>();
        match (js, self) {
            (5, Hand::FiveOfAKind(_)) => Hand::FiveOfAKind(j_hand),
            (4, Hand::FourOfAKind(_)) => Hand::FiveOfAKind(j_hand),
            (3, Hand::FullHouse(_)) => Hand::FiveOfAKind(j_hand),
            (3, Hand::ThreeOfAKind(_)) => Hand::FourOfAKind(j_hand),
            (2, Hand::FullHouse(_)) => Hand::FiveOfAKind(j_hand),
            (2, Hand::TwoPair(_)) => Hand::FourOfAKind(j_hand),
            (2, Hand::OnePair(_)) => Hand::ThreeOfAKind(j_hand),
            (1, Hand::FourOfAKind(_)) => Hand::FiveOfAKind(j_hand),
            (1, Hand::ThreeOfAKind(_)) => Hand::FourOfAKind(j_hand),
            (1, Hand::TwoPair(_)) => Hand::FullHouse(j_hand),
            (1, Hand::OnePair(_)) => Hand::ThreeOfAKind(j_hand),
            (1, Hand::HighCard(_)) => Hand::OnePair(j_hand),
            (_, hand) => hand,
        }
    }
}

pub fn run(input: &'static str) -> (usize, usize) {
    let games = input
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(hand, score)| {
            (
                hand.parse::<Hand>().unwrap(),
                score.parse::<usize>().unwrap(),
            )
        })
        .sorted()
        .collect_vec();

    let p1 = games
        .iter()
        .enumerate()
        .map(|(rank, (_hand, bid))| bid * (rank + 1))
        .sum::<usize>();

    let games = games
        .into_iter()
        .map(|(hand, bid)| (hand.jokerize(), bid))
        .sorted()
        .collect_vec();

    let p2 = games
        .iter()
        .enumerate()
        .map(|(rank, (_hand, bid))| bid * (rank + 1))
        .sum::<usize>();

    (p1, p2)
}

#[test]
fn test() {
    let input = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
    assert_eq!(run(input), (6440, 5905));
}
