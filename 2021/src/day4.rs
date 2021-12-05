#[derive(Debug)]
struct Bingo {
    rows: Vec<usize>,
    cols: Vec<usize>,
    points: Vec<Option<usize>>,
}

impl Bingo {
    pub fn new(board: String) -> Bingo {
        Bingo {
            rows: vec![5; 5],
            cols: vec![5; 5],
            points: board
                .split(' ')
                .map(|s| Some(s.parse::<usize>().unwrap()))
                .collect(),
        }
    }

    pub fn bingo(&mut self, num: usize) -> bool {
        if let Some(pos) = self.points.iter().position(|pt| *pt == Some(num)) {
            let _ = self.points[pos].take();
            self.rows[pos / 5] -= 1;
            self.cols[pos % 5] -= 1;
            self.rows.iter().any(|c| *c == 0) || self.cols.iter().any(|c| *c == 0)
        } else {
            false
        }
    }

    pub fn residual(&self) -> usize {
        self.points.iter().flatten().sum()
    }
}

fn process_input(input: &'static str) -> (String, Vec<String>) {
    let mut input = input.split("\n\n");
    let calls = input.next().unwrap().to_string();
    let boards = input
        .map(|block| block.trim().replace("\n", " ").replace("  ", " "))
        .collect();
    (calls, boards)
}

pub fn run(input: &'static str) -> (usize, usize) {
    let (calls, boards) = process_input(input);
    let mut calls = calls.split(',').map(|s| s.parse::<usize>().unwrap());
    let mut boards = boards.into_iter().map(Bingo::new).collect::<Vec<_>>();

    let (winning_board, winning_call) = loop {
        let call = calls
            .next()
            .expect("Ran out of calls before finishing board");

        let results = boards
            .iter_mut()
            .map(|board| board.bingo(call))
            .collect::<Vec<_>>();

        if let Some(pos) = results.iter().position(|res| *res) {
            break (pos, call);
        }
    };

    let d4p1 = boards[winning_board].residual() * winning_call;
    let d4p2 = 0;
    (d4p1, d4p2)
}

#[test]
fn test() {
    let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

    assert_eq!(run(input), (4512, 0));
}
