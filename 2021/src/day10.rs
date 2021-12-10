use std::ops::Not;

#[allow(dead_code)]
enum SyntaxStatus {
    Valid,
    Incomplete(Vec<Tokens>),
    Corrupted(Tokens),
}

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

fn parse_syntax(line: &str) -> SyntaxStatus {
    let mut stack = Vec::new();
    for token in line.chars().map(Tokens::from) {
        match token {
            Tokens::OpenParens | Tokens::OpenBracks | Tokens::OpenBraces | Tokens::OpenAngles => {
                stack.push(token)
            }
            Tokens::CloseParens
            | Tokens::CloseBracks
            | Tokens::CloseBraces
            | Tokens::CloseAngles => {
                if let Some(opener) = stack.pop() {
                    if opener != !token {
                        return SyntaxStatus::Corrupted(token);
                    }
                }
            }
        };
    }
    if stack.is_empty() {
        SyntaxStatus::Valid
    } else {
        SyntaxStatus::Incomplete(stack)
    }
}

pub fn run(input: &'static str) -> (usize, usize) {
    let (completions, scores): (Vec<_>, Vec<usize>) = input
        .lines()
        .map(parse_syntax)
        .map(|status| match status {
            SyntaxStatus::Corrupted(b) => {
                let score = match b {
                    Tokens::CloseParens => 3,
                    Tokens::CloseBracks => 57,
                    Tokens::CloseBraces => 1197,
                    Tokens::CloseAngles => 25137,
                    _ => 0,
                };
                (None, score)
            }
            SyntaxStatus::Incomplete(v) => (Some(v), 0),
            SyntaxStatus::Valid => (None, 0),
        })
        .unzip();

    let d10p1 = scores.iter().sum();

    let mut d10p2_scores = completions
        .iter()
        .filter_map(|v| v.as_ref())
        .map(|v| {
            v.iter().rev().fold(0_usize, |acc, token| match *token {
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
