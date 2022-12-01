pub fn run(input: &'static str) -> (usize, usize) {
    let p1 = input
        .split("\n\n")
        .map(|elf_cals| {
            elf_cals
                .split("\n")
                .map(|cals| cals.parse::<usize>().unwrap_or_default())
                .sum()
        }).max().unwrap();
    (p1, 0)
}

#[test]
fn test() {
    let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    assert_eq!(run(input), (24000,0));
}
