use std::collections::HashMap;

fn get_median<F>(crab_groups: &[(usize, usize)], bisect: F) -> usize
where
    F: Fn(&[(usize, usize)], usize) -> (usize, usize) + Copy,
{
    let (mut start, mut end) = (0, crab_groups.last().unwrap().0);
    let (mut left, mut right) = (0, 0);

    eprintln!("{:?}", crab_groups);

    let median = loop {
        let pivot = (start + end) / 2;
        let (l, r) = bisect(crab_groups, pivot);

        eprintln!("{}, {}, {}, {}, {}", l, r, start, pivot, end);
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

    median
}

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

    let bisect = |v: &[(usize, usize)], p: usize| -> (usize, usize) {
        (
            v.iter().take_while(|(i, _)| i < &p).map(|(_, w)| w).sum(),
            v.iter().skip_while(|(i, _)| i < &p).map(|(_, w)| w).sum(),
        )
    };

    let median = get_median(&crab_groups, bisect);

    let d7p1: usize = crab_groups
        .iter()
        .map(|(p, w)| p.abs_diff(median) * w)
        .sum();

    let bisect = |v: &[(usize, usize)], p: usize| -> (usize, usize) {
        (
            v.iter()
                .take_while(|(i, _)| i < &p)
                .map(|(i, w)| {
                    let i = i.abs_diff(p);
                    i * (i + 1) * w / 2
                })
                .sum(),
            v.iter()
                .skip_while(|(i, _)| i < &p)
                .map(|(i, w)| {
                    let i = i.abs_diff(p);
                    i * (i + 1) * w / 2
                })
                .sum(),
        )
    };

    let median = get_median(&crab_groups, bisect);

    let mdn_pos_in_vec = {
        let median_plus = crab_groups.iter().take_while(|(i, _)| i < &median).count();
        (median_plus - 1, median_plus)
    };

    eprintln!(
        "{} - {} - {}",
        crab_groups[mdn_pos_in_vec.0].0, median, crab_groups[mdn_pos_in_vec.1].0
    );

    let d7p2 = //(mdn_pos_in_vec.0..=mdn_pos_in_vec.1)
        (crab_groups[mdn_pos_in_vec.0].0..=crab_groups[mdn_pos_in_vec.1].0)
        .into_iter()
        .inspect(|median_pos| eprint!("{} - {} ", median_pos, crab_groups[*median_pos].0))
        .map(|median_pos| {
            let median = crab_groups[median_pos].0;
            crab_groups
                .iter()
                .map(|(p, w)| {
                    let p = p.abs_diff(median);
                    p * (p + 1) * w / 2
                })
            .sum::<usize>()
        })
        .inspect(|median_sum| eprintln!("median_sum: {}", median_sum))
        .collect::<Vec<usize>>();

    eprintln!("{:?}", d7p2);

    (d7p1, d7p2.into_iter().sum())
}

#[test]
pub fn test() {
    let input = "16,1,2,0,4,2,7,1,2,14";
    assert_eq!(run(input), (37, 168))
}
