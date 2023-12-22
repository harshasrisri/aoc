fn hash(input: &str) -> usize {
    input.chars()
        .filter(|c| !c.is_ascii_whitespace())
        .fold(0, |cur_value, c| {
            ((cur_value + c as usize) * 17) % 256
        })

}
pub fn run(input: &'static str) -> (usize, usize) {
    let insts = input.split(',').collect::<Vec<_>>();
    let p1 = insts.iter().map(|inst| hash(inst)).sum();

    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];
    insts
        .into_iter()
        .filter_map(|inst| inst.split_once('=').or(inst.split_once('-')))
        .map(|(inst, lens)| (inst, lens.parse::<usize>().ok()))
        .map(|(label, some_lens)| (hash(label), label, some_lens))
        .for_each(|(box_num, label, some_lens)| {
            let some_pos = boxes[box_num].iter().position(|bx| bx.0 == label);
            match (some_pos, some_lens) {
                (Some(pos), Some(lens)) => boxes[box_num][pos].1 = lens,
                (None, Some(lens)) => boxes[box_num].push((label, lens)),
                (Some(pos), None) => { boxes[box_num].remove(pos); },
                (None, None) => {},
            }
        });

    let p2 = boxes.into_iter().enumerate().flat_map(|(box_num, lenses)| 
            lenses.into_iter().enumerate().map(move |(slot, (_, focal_length))| {
                (box_num + 1) * (slot + 1) * focal_length
        })).sum();

    (p1, p2)
}

#[test]
fn test1() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(run(input), (1320, 145));
}
