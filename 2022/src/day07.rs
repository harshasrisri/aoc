use std::{collections::HashMap, path::PathBuf};
use sscanf::sscanf;

pub fn run(input: &'static str) -> (usize, usize) {
    let mut pwd = PathBuf::new();
    let mut dir_sizes: HashMap<PathBuf, usize> = HashMap::new();

    for line in input.lines() {
        // eprintln!("{}({}): {line}", pwd.display(), dir_sizes.get(&pwd).unwrap_or(&0));
        if line.starts_with("$ cd") {
            match line.splitn(3, ' ').last().unwrap() {
                ".." => { pwd.pop(); }
                child => { pwd.push(child); }
            };
        } else if line.starts_with("$ ls") {
            continue;
        } else if line.starts_with("dir") {
            continue;
        } else {
            let (size, _file) = sscanf!(line, "{} {}", usize, str).unwrap();
            pwd
                .ancestors()
                .for_each(|anc| {
                    dir_sizes.entry(anc.to_path_buf()).and_modify(|sz| *sz += size).or_insert(size);
                });
        }
    }

    let p1 = dir_sizes
        .values()
        // .inspect(|v| eprintln!("{v}"))
        .filter(|v| **v < 100000)
        // .inspect(|v| eprintln!("{v}"))
        .sum();

    let mut sorted_sizes = dir_sizes.values().collect::<Vec<_>>();
    sorted_sizes.sort();
    let space_needed = 30000000 - (70000000 - **sorted_sizes.last().unwrap());
    let candidate_pos = match sorted_sizes.binary_search(&&space_needed) {
        Ok(pos) => pos,
        Err(pos) => pos,
    };

    (p1, **sorted_sizes.get(candidate_pos).unwrap())
}

#[test]
fn test() {
    let input = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";
    assert_eq!(run(input), (95437, 24933642));
}
