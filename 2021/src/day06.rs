use std::collections::VecDeque;

pub fn run(input: &'static str) -> (usize, usize) {
    let mut fish_age_groups: VecDeque<usize> = input
        .trim()
        .split(',')
        .fold(vec![0_usize; 9], |mut acc_vec, timer| {
            let timer = timer.parse::<usize>().unwrap();
            acc_vec[timer] += 1;
            acc_vec
        })
        .into();

    let elapse_day = |age_groups: &mut VecDeque<usize>| {
        let day0 = age_groups.pop_front().unwrap();
        age_groups[6] += day0;
        age_groups.push_back(day0);
    };

    (0..80).for_each(|_| elapse_day(&mut fish_age_groups));
    let d6p1 = fish_age_groups.iter().sum();

    (80..256).for_each(|_| elapse_day(&mut fish_age_groups));
    let d6p2 = fish_age_groups.into_iter().sum();

    (d6p1, d6p2)
}

#[test]
fn test() {
    let input = "3,4,3,1,2";
    assert_eq!(run(input), (5934, 26984457539));
}
