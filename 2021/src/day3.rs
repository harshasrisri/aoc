pub fn run(input: &'static str) -> (usize, usize) {
    let input = input
        .lines()
        .collect::<Vec<_>>();

    let signals = |input: &Vec<&str>| {
        input
            .iter()
            .map(|bits| {
                bits.chars()
                    .map(|bit| match bit {
                        '0' => -1,
                        '1' => 1,
                        _ => panic!("invalid bit {}", bit),
                    })
                .collect::<Vec<isize>>()
            })
        .fold(Vec::new(), |acc: Vec<isize>, signals| {
            let acc = if acc.is_empty() {
                vec![0; signals.len()]
            } else {
                acc
            };
            acc.iter().zip(signals.iter()).map(|(a, b)| a + b).collect()
        })
    };

    let (gamma, epsilon) = signals(&input)
        .iter()
        .fold((0, 0), |(gamma, epsilon), signal| match signal.cmp(&0) {
            std::cmp::Ordering::Greater => ((gamma << 1) | 1, epsilon << 1),
            std::cmp::Ordering::Less => (gamma << 1, (epsilon << 1) | 1),
            std::cmp::Ordering::Equal => panic!("Not expecting 0 here"),
        });

    let d3p1 = gamma * epsilon;

    let mut interim = input.clone();
    let mut pos = 0;
    let o2 = loop {
        let signal = signals(&interim);
        let signal = signal
            .iter()
            .map(|signal| if signal >= &0 { '1' } else { '0' })
            .nth(pos);
        interim = interim
            .into_iter()
            .filter(|diag| diag.chars().nth(pos) == signal)
            .collect::<Vec<_>>();
        if interim.len() == 1 {
            break interim.pop().unwrap();
        } else {
            pos += 1;
        };
    }
    .chars()
        .fold(0, |acc, bit| match bit {
            '1' => (acc << 1) | 1,
            '0' => (acc << 1),
            _ => acc,
        });

    let mut interim = input;
    let mut pos = 0;
    let co2 = loop {
        let signal = signals(&interim);
        let signal = signal
            .iter()
            .map(|signal| if signal >= &0 { '0' } else { '1' })
            .collect::<String>();
        let signal = signal.chars().nth(pos);
        interim = interim
            .into_iter()
            .filter(|diag| diag.chars().nth(pos) == signal)
            .collect::<Vec<_>>();
        if interim.len() <= 1 {
            break interim.pop().unwrap();
        } else {
            pos += 1;
        };
    }
    .chars()
        .fold(0, |acc, bit| match bit {
            '1' => (acc << 1) | 1,
            '0' => (acc << 1),
            _ => acc,
        });

    let d3p2 = o2 * co2;

    (d3p1, d3p2)
}

#[test]
fn test() { 
    let input = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
    assert_eq!(run(input), (198, 230));
}
