use std::ops::Not;

#[derive(Clone, Copy, PartialEq)]
enum Tokens {
    OpenParens,
    CloseParens,
    OpenBracks,
    CloseBracks,
    OpenBraces,
    CloseBraces,
    OpenAngles,
    CloseAngles,
}

impl Not for Tokens {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Tokens::OpenParens => Tokens::CloseParens,
            Tokens::CloseParens => Tokens::OpenParens,
            Tokens::OpenBracks => Tokens::CloseBracks,
            Tokens::CloseBracks => Tokens::OpenBracks,
            Tokens::OpenBraces => Tokens::CloseBraces,
            Tokens::CloseBraces => Tokens::OpenBraces,
            Tokens::OpenAngles => Tokens::CloseAngles,
            Tokens::CloseAngles => Tokens::OpenAngles,
        }
    }
}

impl From<char> for Tokens {
    fn from(c: char) -> Self {
        match c {
            '(' => Tokens::OpenParens,
            '[' => Tokens::OpenBracks,
            '{' => Tokens::OpenBraces,
            '<' => Tokens::OpenAngles,
            ')' => Tokens::CloseParens,
            ']' => Tokens::CloseBracks,
            '}' => Tokens::CloseBraces,
            '>' => Tokens::CloseAngles,
            _ => panic!("Invalid token"),
        }
    }
}

fn parse_syntax(line: &str) -> (Option<Vec<Tokens>>, Option<Tokens>) {
    let mut stack = Vec::new();
    for token in line.chars().map(Tokens::from) {
        match token {
            Tokens::OpenParens | Tokens::OpenBracks | Tokens::OpenBraces | Tokens::OpenAngles => {
                stack.push(token)
            }
            token => {
                if let Some(opener) = stack.pop() {
                    if opener != !token {
                        return (None, Some(token));
                    }
                }
            }
        };
    }
    if stack.is_empty() {
        (None, None)
    } else {
        (Some(stack.into_iter().rev().collect()), None)
    }
}

pub fn run(input: &'static str) -> (usize, usize) {
    let (completions, scores): (Vec<_>, Vec<_>) = input.lines().map(parse_syntax).unzip();

    let d10p1 = scores
        .into_iter()
        .flatten()
        .fold(0, |acc, token| match token {
            Tokens::CloseParens => acc + 3,
            Tokens::CloseBracks => acc + 57,
            Tokens::CloseBraces => acc + 1197,
            Tokens::CloseAngles => acc + 25137,
            _ => panic!("Invalid token"),
        });

    let mut d10p2_scores = completions
        .into_iter()
        .flatten()
        .map(|incompletions| {
            incompletions
                .into_iter()
                .fold(0_usize, |acc, token| match token {
                    Tokens::OpenParens => acc * 5 + 1,
                    Tokens::OpenBracks => acc * 5 + 2,
                    Tokens::OpenBraces => acc * 5 + 3,
                    Tokens::OpenAngles => acc * 5 + 4,
                    _ => panic!("Invalid token"),
                })
        })
        .collect::<Vec<_>>();

    d10p2_scores.sort_unstable();

    (d10p1, d10p2_scores[d10p2_scores.len() / 2])
}

#[test]
fn test() {
    let input = "
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";
    assert_eq!(run(input), (26397, 288957));
}
