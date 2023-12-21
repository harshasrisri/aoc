fn hash(input: &str) -> usize {
    input.chars()
        .filter(|c| !c.is_ascii_whitespace())
        .map(|c| c as usize)
        .fold(0, |cur_value, ascii| {
            ((cur_value + ascii) * 17) % 256
        })

}
pub fn run(input: &'static str) -> (usize, usize) {
    let insts = input.split(',').collect::<Vec<_>>();
    let p1 = insts.into_iter().map(hash).sum();
    (p1, 0)
}

#[test]
fn test1() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(run(input), (1320, 0));
}
