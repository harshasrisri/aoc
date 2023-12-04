fn solve_line(line: &str) -> Option<usize> {
    let dig_vec = line
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<Vec<_>>();
    let num_str = vec![dig_vec.first(), dig_vec.last()]
        .into_iter()
        .filter_map(|opt| opt.cloned())
        .collect::<String>();
    num_str.parse::<usize>().ok()
}

pub fn run(input: &'static str) -> (usize, usize) {
    let digit_names = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let input_p2 = input.lines().map(|line| {
        let mut line = line.to_string();
        let left = digit_names
            .iter()
            .enumerate()
            .filter_map(|(digit, name)| line.find(name).map(move |pos| (pos, digit)))
            .min();
        let right = digit_names
            .iter()
            .enumerate()
            .filter_map(|(digit, name)| line.rfind(name).map(move |pos| (pos, digit)))
            .max();

        if let Some((pos, digit)) = left {
            line.replace_range(pos..pos + 1, &digit.to_string());
        }
        if let Some((pos, digit)) = right {
            line.replace_range(pos..pos + 1, &digit.to_string());
        }
        line
    });

    (
        input.lines().filter_map(solve_line).sum(),
        input_p2.filter_map(|line| solve_line(&line)).sum(),
    )
}

#[test]
fn test1() {
    let input = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
    println!("Test one");
    assert_eq!(run(input).0, 142);
}

#[test]
fn test2() {
    let input = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
    println!("Test two");
    assert_eq!(run(input).1, 281);
}
