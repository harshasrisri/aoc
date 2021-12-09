use std::str::FromStr;

#[allow(dead_code)]
struct SegPattern {
    input: Vec<String>,
    output: Vec<String>,
    segments: Vec<String>,
    digits: Vec<String>,
}

impl FromStr for SegPattern {
    type Err = std::str::Utf8Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (input, output) = s.split_once('|').unwrap();

        let output = output.trim().split(' ').map(String::from).collect();
        let mut input = input
            .trim()
            .split(' ')
            .map(String::from)
            .collect::<Vec<String>>();
        input.iter_mut().for_each(|s| {
            let mut chars = s.chars().collect::<Vec<char>>();
            chars.sort_unstable();
            *s = chars.iter().collect();
        });
        input.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));

        Ok(SegPattern {
            input,
            output,
            segments: Vec::new(),
            digits: Vec::new(),
        })
    }
}

fn process_data(input: &'static str) -> Vec<SegPattern> {
    input
        .trim()
        .lines()
        .map(SegPattern::from_str)
        .collect::<Result<Vec<_>, std::str::Utf8Error>>()
        .unwrap()
}

trait SegString {
    fn sub(self, rhs: &Self) -> Self;
    fn is_anagram_of(&self, rhs: &Self) -> bool;
}

impl SegString for String {
    fn sub(mut self, rhs: &Self) -> Self {
        rhs.chars().for_each(|c| self.remove_matches(c));
        self
    }

    fn is_anagram_of(&self, rhs: &Self) -> bool {
        let mut lhs = self.chars().collect::<Vec<_>>();
        lhs.sort_unstable();
        let mut rhs = rhs.chars().collect::<Vec<_>>();
        rhs.sort_unstable();
        lhs == rhs
    }
}

impl SegPattern {
    fn make_digits(&mut self) {
        let (a, b, c, d, e, f, g) = (
            &self.segments[0],
            &self.segments[1],
            &self.segments[2],
            &self.segments[3],
            &self.segments[4],
            &self.segments[5],
            &self.segments[6],
        );
        let zero00 = String::new() + a + b + c + e + f + g;
        let one111 = String::new() + c + f;
        let two222 = String::new() + a + c + d + e + g;
        let three3 = String::new() + a + c + d + f + g;
        let four44 = String::new() + b + c + d + f;
        let five55 = String::new() + a + b + d + f + g;
        let six666 = String::new() + a + b + d + e + f + g;
        let seven7 = String::new() + a + c + f;
        let eight8 = String::new() + a + b + c + d + e + f + g;
        let nine99 = String::new() + a + b + c + d + f + g;
        self.digits = vec![
            zero00, one111, two222, three3, four44, five55, six666, seven7, eight8, nine99,
        ];
    }

    fn deduce_output(&mut self) -> usize {
        self.deduce_segments();
        self.output
            .iter()
            .map(|op| -> usize {
                for (i, digit) in self.digits.iter().enumerate() {
                    if digit.is_anagram_of(op) {
                        return i;
                    }
                }
                panic!("panic")
            })
            .fold(0, |acc, n| acc * 10 + n)
    }

    fn deduce_segments(&mut self) {
        let cf = self.input.remove(0);
        let acf = self.input.remove(0);
        let bdcf = self.input.remove(0);
        let abcdefg = self.input.pop().unwrap();

        assert_eq!(self.input.len(), 6);

        let bd = bdcf.sub(&cf);
        let a = acf.sub(&cf);

        let six_seg = self.input[3..].iter();

        let g = six_seg
            .clone()
            .map(|num| num.clone().sub(&a).sub(&bd).sub(&cf))
            .find(|num| num.len() == 1)
            .unwrap();

        let e = abcdefg.sub(&a).sub(&bd).sub(&cf).sub(&g);

        let f = six_seg
            .clone()
            .map(|num| num.clone().sub(&a).sub(&bd).sub(&e).sub(&g))
            .find(|num| num.len() == 1)
            .unwrap();

        let c = cf.clone().sub(&f);

        let b = six_seg
            .map(|num| num.clone().sub(&a).sub(&cf).sub(&e).sub(&g))
            .find(|num| num.len() == 1)
            .unwrap();

        let d = bd.sub(&b);

        self.input.clear();
        self.segments = vec![a, b, c, d, e, f, g];
        self.make_digits();
    }
}

pub fn run(input: &'static str) -> (usize, usize) {
    let mut patterns = process_data(input);

    let d8p1 = patterns
        .iter()
        .map(|pat| {
            pat.output
                .iter()
                .filter(|output| [2, 3, 4, 7].contains(&output.len()))
                .count()
        })
        .sum();

    let d8p2 = patterns.iter_mut().map(|pat| pat.deduce_output()).sum();

    (d8p1, d8p2)
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
    assert_eq!(run(input), (26, 61229));
}
