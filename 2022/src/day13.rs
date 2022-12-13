use std::{str::FromStr, cmp::Ordering};
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
enum List {
    Num(usize),
    List(Vec<List>),
}

impl List {
    fn compare(&self, right: &Self) -> Ordering {
        eprintln!("\nleft: {:?}", self);
        eprintln!("right: {:?}", right);
        match (self, right) {
            (List::Num(l), List::Num(r)) => l.cmp(r),
            (List::Num(l), List::List(_r)) => List::List(vec![List::Num(*l)]).compare(right),
            (List::List(_l), List::Num(r)) => self.compare(&List::List(vec![List::Num(*r)])),
            (List::List(l), List::List(r)) => {
                for (left, right) in l.iter().zip(r.iter()) {
                    match left.compare(right) {
                        Ordering::Equal => continue,
                        ordering => return ordering,
                    }
                    //     eprintln!("in order: {:?}, {:?}", left, right);
                    //     return true;
                    // } else {
                    //     eprintln!("not in order: {:?}, {:?}", left, right);
                    //     continue;
                    // }
                }
                l.len().cmp(&r.len())
            }
        }
    }
}

impl FromStr for List {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).map_err(|e| e.to_string())
    }
}

pub fn run(input: &'static str) -> (usize, usize) {
    let list: Vec<_> = input
        .split("\n\n")
        .map(|pair| {
            let (left, right) = pair.split_once('\n').unwrap();
            (List::from_str(&left).unwrap(), List::from_str(&right).unwrap())
        })
        .collect();


    let p1 = list
        .iter()
        .zip(1..)
        .filter_map(|((left, right), index)| {
            if left.compare(right) == Ordering::Less { Some(index) } else { None }
        })
        .sum();

    (p1, 0)
}

#[test]
fn test() {
    let input = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";
    assert_eq!(run(input), (13 ,0));
}

