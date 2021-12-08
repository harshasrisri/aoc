pub fn run(input: &'static str) -> (usize, usize) { 
    (0,0)
}

#[test]
pub fn test() { 
    let input = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    assert_eq!(run(input), (0,0));
}
