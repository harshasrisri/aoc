use std::collections::VecDeque;

pub fn run(input: &'static str) -> (usize, usize) {
    let mut timer_pop = VecDeque::from(vec![0 as usize; 9]);
    input.trim().split(',').for_each(|timer| timer_pop[timer.parse::<usize>().unwrap()] += 1);

    for _ in 0..80 { 
        let day0 = timer_pop.pop_front().unwrap();
        timer_pop[6] += day0;
        timer_pop.push_back(day0);
    }

    let d6p1 = timer_pop.iter().sum();

    for _ in 81..=256 { 
        let day0 = timer_pop.pop_front().unwrap();
        timer_pop[6] += day0;
        timer_pop.push_back(day0);
    }

    let d6p2 = timer_pop.into_iter().sum();

    (d6p1, d6p2)
}

#[test]
fn test() {
    let input = "3,4,3,1,2";
    assert_eq!(run(input), (5934, 26984457539));
}
