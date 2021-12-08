use std::collections::HashMap;

pub fn run(input: &'static str) -> (usize, usize) {
    let mut crab_groups: Vec<(usize, usize)> = input
        .trim()
        .split(',')
        .fold(HashMap::new(), |mut acc_map, h_pos| {
            let h_pos = h_pos.parse::<usize>().unwrap();
            let h_pos_entry = acc_map.entry(h_pos).or_default();
            *h_pos_entry += 1;
            acc_map
        })
        .into_iter()
        .collect();

    crab_groups.sort_by(|a, b| a.0.cmp(&b.0));

    // brute force method
    // let d7p1: usize = (0..crab_groups.len())
    //     .map(|i| {
    //         crab_groups[..i]
    //             .iter()
    //             .chain(crab_groups[i + 1..].iter())
    //             .map(|(p, w)| p.abs_diff(crab_groups[i].0) * w)
    //             .sum()
    //     })
    //     .min()
    //     .unwrap();

    // bisection method
    let bisect = |v: &Vec<(usize, usize)>, p: usize| -> (usize, usize) {
        (
            v.iter().take_while(|(i, _)| i < &p).map(|(_, w)| w).sum(),
            v.iter().skip_while(|(i, _)| i < &p).map(|(_, w)| w).sum(),
        )
    };

    let (mut start, mut end) = (0, crab_groups.last().unwrap().0);
    let (mut left, mut right) = (0, 0);

    let median = loop {
        let pivot = (start + end) / 2;
        let (l, r) = bisect(&crab_groups, pivot);

        if (left, right) == (l, r) {
            break pivot;
        } else if l < r {
            start = pivot;
        } else {
            end = pivot;
        }
        left = l;
        right = r;
    };

    let d7p1: usize = crab_groups
        .iter()
        .map(|(p, w)| p.abs_diff(median) * w)
        .sum();

    (d7p1, 0)
}

#[test]
pub fn test() {
    let input = "16,1,2,0,4,2,7,1,2,14";
    assert_eq!(run(input), (37, 0))
}
