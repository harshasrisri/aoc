use std::{collections::HashMap, fmt::Display};

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
enum Cave {
    Start,
    Big(String),
    Small(String),
    Twice(String),
    End,
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cave::Start => "start".to_owned(),
                Cave::Big(s) => format!(",{}", s),
                Cave::Small(s) => format!(",{}", s),
                Cave::Twice(s) => format!(",{}", s),
                Cave::End => ",end\n".to_string(),
            }
        )
    }
}

impl Cave {
    fn is_big(&self) -> bool {
        matches!(self, Cave::Big(_))
    }

    fn bump(self) -> Self {
        if let Cave::Small(cave) = self {
            Cave::Twice(cave)
        } else {
            self
        }
    }

    fn boop(self) -> Self {
        if let Cave::Twice(cave) = self {
            Cave::Small(cave)
        } else {
            self
        }
    }
}

type CaveMap = HashMap<Cave, Vec<Cave>>;

impl From<&str> for Cave {
    fn from(s: &str) -> Self {
        match s {
            "start" => Self::Start,
            "end" => Self::End,
            s if s.chars().all(|c| c.is_ascii_uppercase()) => Self::Big(s.to_string()),
            s if s.chars().all(|c| c.is_ascii_lowercase()) => Self::Small(s.to_string()),
            s => panic!("invalid cave label: {}", s),
        }
    }
}

fn map_caves(input: &'static str) -> CaveMap {
    let mut caves: HashMap<Cave, Vec<Cave>> = HashMap::new();
    let conn_iter = input
        .trim()
        .lines()
        .filter_map(|line| line.split_once('-'))
        .map(|(start, end)| (Cave::from(start), Cave::from(end)));

    for (start, end) in conn_iter {
        assert!(
            !(start.is_big() && end.is_big()),
            "2 big caves connected. Not handled"
        );

        let (start, end) = match (start, end) {
            (s, Cave::Start) => (Cave::Start, s),
            (Cave::End, e) => (e, Cave::End),
            (s, e) => (s, e),
        };

        caves.entry(start.clone()).or_default().push(end.clone());

        if !(start == Cave::Start || end == Cave::End) {
            caves.entry(end).or_default().push(start);
        }
    }

    caves
}

fn traverse(mut cave_map: CaveMap, curr: &Cave) -> usize {
    let adjacents = match curr {
        Cave::Start | Cave::Small(_) => cave_map.remove(curr),
        Cave::End => return 1,
        Cave::Big(_) => cave_map.get(curr).cloned(),
        Cave::Twice(_) => {
            let adj = cave_map.get(curr).cloned();
            boop_medium(&mut cave_map, curr);
            adj
        }
    };

    let adjacents = if let Some(adj) = adjacents {
        adj
    } else {
        return 0;
    };

    let mut num_paths = 0;
    for cave in adjacents.iter() {
        num_paths += traverse(cave_map.clone(), cave);
    }

    num_paths
}

fn boop_medium(cave_map: &mut CaveMap, twice_cave: &Cave) {
    let (twice_cave, list) = cave_map.remove_entry(twice_cave).unwrap();
    for (_, list) in cave_map.iter_mut() {
        if let Some(pos) = list.iter().position(|cave| *cave == twice_cave) {
            let twice_cave = list.swap_remove(pos);
            list.push(twice_cave.boop());
        }
    }
    cave_map.insert(twice_cave.boop(), list);
}

fn bump_small(cave_map: &CaveMap, small: &Cave) -> CaveMap {
    let mut new_cave = cave_map.clone();
    let (small, list) = new_cave.remove_entry(small).unwrap();
    for (_, list) in new_cave.iter_mut() {
        if let Some(pos) = list.iter().position(|cave| *cave == small) {
            let small = list.swap_remove(pos);
            list.push(small.bump());
        }
    }
    new_cave.insert(small.bump(), list);
    new_cave
}

pub fn run(input: &'static str) -> (usize, usize) {
    let caves = map_caves(input);

    let d12p1 = caves
        .get(&Cave::Start)
        .unwrap()
        .iter()
        .map(|to| traverse(caves.clone(), to))
        .sum();

    let d12p2 = caves
        .keys()
        .filter(|cave| matches!(cave, Cave::Small(_)))
        .map(|small| {
            let cave_map = bump_small(&caves, small);
            cave_map
                .get(&Cave::Start)
                .unwrap()
                .iter()
                .map(|to| traverse(cave_map.clone(), to))
                .sum::<usize>()
        })
        .sum();

    (d12p1, d12p2)
}

#[test]
fn test1() {
    let input = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end";
    assert_eq!(run(input), (10, 36));
}

#[test]
fn test2() {
    let input = "dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc";
    assert_eq!(run(input), (19, 103));
}

#[test]
fn test3() {
    let input = "fs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX\npj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW";
    assert_eq!(run(input), (226, 3509));
}
