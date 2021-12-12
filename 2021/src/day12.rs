pub fn run(input: &'static str) -> (usize, usize) { 
    todo!()
}

#[test]
fn test1() { 
    let input = "
start-A
start-b
A-c
A-b
b-d
A-end
b-end
";
    assert_eq!(run(input), (10, 0));
}

#[test]
fn test2() { 
    let input = "
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";
    assert_eq!(run(input), (19, 0));
}

#[test]
fn test3() { 
    let input = "
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
";
    assert_eq!(run(input), (226, 0));
}
