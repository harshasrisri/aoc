use std::ops::Not;

#[allow(dead_code)]
enum SyntaxStatus {
    Valid,
    Incomplete(Tokens),
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
            Tokens::OpenParens =>  Tokens::CloseParens,
            Tokens::CloseParens => Tokens::OpenParens,
            Tokens::OpenBracks =>  Tokens::CloseBracks,
            Tokens::CloseBracks => Tokens::OpenBracks,
            Tokens::OpenBraces =>  Tokens::CloseBraces,
            Tokens::CloseBraces => Tokens::OpenBraces,
            Tokens::OpenAngles =>  Tokens::CloseAngles,
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
            _ => panic!("Invalid token")
        }
    }
}

fn parse_syntax(line: &str) -> SyntaxStatus {
    let mut stack = Vec::new();
    for token in line.chars().map(|c| Tokens::from(c)) {
        match token {
            Tokens::OpenParens | Tokens::OpenBracks | Tokens::OpenBraces | Tokens::OpenAngles => stack.push(token),
            Tokens::CloseParens | Tokens::CloseBracks | Tokens::CloseBraces | Tokens::CloseAngles => {
                if let Some(opener) = stack.pop() {
                    if opener != !token {
                        return SyntaxStatus::Corrupted(token);
                    }
                } else {
                    return SyntaxStatus::Incomplete(token);
                }
            }
        };
    }
    SyntaxStatus::Valid
}

pub fn run(input: &'static str) -> (usize, usize) {
    let d10p1 = input
        .lines()
        .map(parse_syntax)
        .filter_map(|status| if let SyntaxStatus::Corrupted(b) = status {
            match b {
                Tokens::CloseParens => Some(3),
                Tokens::CloseBracks => Some(57),
                Tokens::CloseBraces => Some(1197),
                Tokens::CloseAngles => Some(25137),
                _ => None,
            }
        } else {
            None
        })
        .sum();

    (d10p1, 0)
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
    assert_eq!(run(input), (26397, 0));
}
