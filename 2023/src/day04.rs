use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{char, multispace0, digit0},
    multi::separated_list0,
    sequence::{preceded, separated_pair, terminated},
    IResult, combinator::map_res,
};

#[derive(Debug, Default)]
struct Card {
    id: usize,
    wins: HashSet<usize>,
    hits: HashSet<usize>,
}

impl Card {
    fn score(&self) -> usize {
        let matches = self.wins.intersection(&self.hits).count();
        if matches >= 1 {
            2_usize.pow(matches as u32 - 1)
        } else {
            0
        }
    }

    fn parse(input: &str) -> IResult<&str, Card> {
        let (input, id) = terminated(
            preceded(
                tag("Card"), 
                preceded(
                    multispace0, 
                    map_res(digit0, str::parse::<usize>)
                )
            ),
            char(':')
        )(input)?;

        let (input, (wins, hits)) = separated_pair(
            separated_list0(
                tag(" "),
                preceded(
                    multispace0, 
                    map_res(digit0, str::parse::<usize>)
                )
            ),
            tag(" | "), 
            separated_list0(
                tag(" "),
                preceded(
                    multispace0, 
                    map_res(digit0, str::parse::<usize>)
                )
            )
        )(input)?;

        Ok((input, Card { id, wins: wins.into_iter().collect(),  hits: hits.into_iter().collect() }))
    }
}

pub fn run(input: &'static str) -> (usize, usize) {
    let p1 = input
        .lines()
        .filter_map(|line| Card::parse(line).ok())
        .map(|(_, card)| card.score())
        .sum::<usize>();

    (p1, 0)
}

#[test]
fn test() {
    let input = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
    assert_eq!(run(input), (13, 0));
}
