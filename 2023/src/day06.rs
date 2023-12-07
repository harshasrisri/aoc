use nom::{sequence::{delimited, terminated, tuple}, character::complete::{space0, digit0, line_ending}, IResult, multi::many0, combinator::{eof, map_res}, bytes::complete::tag, branch::alt};

pub fn run(input: &'static str) -> (usize, usize) {
    let parse_number = |input| -> IResult<&str, usize> { delimited(space0, map_res(digit0, str::parse::<usize>), space0)(input) };
    let (input, (_, times)) = terminated(tuple((tag("Time:"), many0(parse_number))), alt((line_ending, eof)))(input).unwrap();
    let (_, (_, distances)) = terminated(tuple((tag("Distance:"), many0(parse_number))), alt((line_ending, eof)))(input).unwrap();

    let p1 = times.into_iter().zip(distances.into_iter())
        .map(|(time, distance)| (0..=time).filter(|t| t * (time - t) > distance).count())
        .product::<usize>();

    (p1, 0)
}

#[test]
fn test() {
    let input = "\
Time:      7  15   30
Distance:  9  40  200
";
    assert_eq!(run(input), (288, 71503));
}

