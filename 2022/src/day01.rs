pub fn run(input: &'static str) -> (usize, usize) {
    let (min, mid, max) = input
        .split("\n\n")
        .map(|elf_cals| {
            elf_cals
                .split('\n')
                .map(|cals| cals.parse::<usize>().unwrap_or_default())
                .sum()
        })
        .fold((0,0,0), |(min, mid, max), cals| {
            if cals >= max {
                (mid, max, cals)
            } else if cals >= mid {
                (mid, cals, max)
            } else if cals >= min {
                (cals, mid, max)
            } else {
                (min, mid, max)
            }
        });

    (max, max + mid + min)
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
    assert_eq!(run(input), (24000, 45000));
}
