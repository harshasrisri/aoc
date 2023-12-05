use std::collections::HashMap;

fn get_num(input: &Vec<Vec<char>>, row: usize, mut col: usize) -> Option<(usize, usize)> {
    if row >= input.len() || col >= input[0].len() { return None; }
    if !input[row][col].is_ascii_digit() { return None; }
    while col > 0 && input[row][col].is_ascii_digit() { col -= 1; }
    while !input[row][col].is_ascii_digit() { col += 1; }
    let (mut count, mut num) = (0,  0);
    for digit in input[row].iter().skip(col).take_while(|c| c.is_ascii_digit()).filter_map(|c| c.to_digit(10)) {
        num *= 10;
        num += digit as usize;
        count += 1;
    }
    if count == 0 { None } else { Some((count, num)) }
}

fn surrounded_by_symbol(skip: usize, input: &Vec<Vec<char>>, row: usize, col: usize) -> Option<(char, usize, usize)> {
    let row_min = row.saturating_sub(1);
    let row_max = (input.len() - 1).min(row + 1);
    let col_min = col.saturating_sub(1);
    let col_max = (input[row].len() - 1).min(col + skip);

    for r in row_min..=row_max {
        for c in col_min..=col_max {
            if r == row && c >= col && c < col + skip { continue; }
            if input[r][c].is_ascii_punctuation() && input[r][c] != '.' {
                return Some((input[r][c], r, c))
            }
        }
    }
    return None;
}

pub fn run(input: &'static str) -> (usize, usize) {
    let input = input.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut p1 = 0_usize;
    let mut gear_map = HashMap::new();

    for row in 0..input.len() {
        let mut col = 0;
        while col < input[row].len() {
            if let Some((skip, num)) = get_num(&input, row, col) {
                if let Some((symbol, sym_row, sym_col)) = surrounded_by_symbol(skip, &input, row, col) {
                    if symbol == '*' {
                        gear_map.entry((symbol, sym_row, sym_col)).and_modify(|v: &mut Vec<usize>| v.push(num)).or_insert(vec![num]);
                    }
                    p1 += num;
                }
                col += skip;
            } else {
                col += 1;
            }
        }
    }

    let p2 = gear_map.into_values().filter(|v| v.len() == 2).map(|v| v[0] * v[1]).sum::<usize>();

    (p1,p2)
}

#[test]
fn test() {
    let input = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
    assert_eq!(run(input), (4361, 467835));
}
