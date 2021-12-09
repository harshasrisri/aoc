use std::str::FromStr;

struct Patterns { 
    pub input: Vec<String>,
    pub output: Vec<String>,
}

impl FromStr for Patterns {
    type Err = std::str::Utf8Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (input, output) = s.split_once('|').unwrap();
        Ok(Patterns { 
            input: input.split(' ').map(String::from).collect(),
            output: output.split(' ').map(String::from).collect(),
        })
    }
}

fn process_data(input: &'static str) -> Vec<Patterns> { 
    input
        .trim()
        .lines()
        .map(Patterns::from_str)
        .collect::<Result<Vec<_>, std::str::Utf8Error>>()
        .unwrap()
}

pub fn run(input: &'static str) -> (usize, usize) { 
    let patterns = process_data(input);

    let d8p1 = patterns
        .iter()
        .map(|pat| pat.output
             .iter()
             .filter(|output| [2, 3, 4, 7].contains(&output.len()))
             .count())
        .sum();

    (d8p1, 0)
}

#[test]
pub fn test() { 
    let input = "
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";
    assert_eq!(run(input), (26,0));
}
