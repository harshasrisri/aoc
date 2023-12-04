use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0, u32},
    multi::separated_list0,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

#[derive(Default, Debug)]
struct Subset {
    red: usize,
    green: usize,
    blue: usize,
}

impl Subset {
    fn valid(&self, other: &Self) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }

    fn max(&self, other: &Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn power(&self) -> usize {
        self.red * self.blue * self.green
    }
}

impl From<Vec<(u32, &str)>> for Subset {
    fn from(value: Vec<(u32, &str)>) -> Self {
        value
            .into_iter()
            .fold(Default::default(), |mut s, (count, color)| {
                match color.trim() {
                    "red" => {
                        s.red += count as usize;
                    }
                    "blue" => {
                        s.blue += count as usize;
                    }
                    "green" => {
                        s.green += count as usize;
                    }
                    _ => panic!("unknown color"),
                }
                s
            })
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    subsets: Vec<Subset>,
}

fn parse_game(line: &str) -> IResult<&str, Game> {
    let (remaining, id) =
        terminated(preceded(tag("Game"), preceded(multispace0, u32)), char(':'))(line)?;
    let (remaining, subsets) = separated_list0(
        tag(";"),
        separated_list0(
            tag(","),
            separated_pair(
                preceded(multispace0, u32),
                tag(" "),
                alt((tag("blue"), tag("red"), tag("green"))),
            ),
        ),
    )(remaining)?;

    // println!("Parsed: Subsets: {subsets:?}; remaining: {remaining}");
    Ok((
        remaining,
        Game {
            id: id as usize,
            subsets: subsets.into_iter().map(From::from).collect(),
        },
    ))
}

pub fn run(input: &'static str) -> (usize, usize) {
    let max_sub = Subset {
        red: 12,
        green: 13,
        blue: 14,
    };

    let (p1, p2) = input
        .lines()
        .filter_map(|line| parse_game(line).ok())
        .map(|(_, game)| {
            let valid = game.subsets.iter().all(|subset| subset.valid(&max_sub));
            let power = game
                .subsets
                .into_iter()
                .fold(Default::default(), |min_set: Subset, subset| {
                    min_set.max(&subset)
                })
                .power();
            (if valid { Some(game.id) } else { None }, power)
        })
        .fold((0_usize, 0_usize), |(p1, p2), (opt_id, power)| {
            (p1 + if let Some(id) = opt_id { id } else { 0 }, p2 + power)
        });

    (p1, p2)
}

#[test]
fn test() {
    let input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
    assert_eq!(run(input), (8, 2286));
}
