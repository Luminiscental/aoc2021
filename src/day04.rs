use super::day::Day;

const WINS: [usize; 10] = [
    0b1111100000000000000000000,
    0b11111000000000000000,
    0b111110000000000,
    0b1111100000,
    0b11111,
    0b1000010000100001000010000,
    0b100001000010000100001000,
    0b10000100001000010000100,
    0b1000010000100001000010,
    0b100001000010000100001,
];

#[derive(Clone)]
struct Board {
    rows: Vec<Vec<usize>>,
}

pub struct Bingo {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

pub struct Day04;

impl Day for Day04 {
    type Input = Bingo;
    type ProcessedInput = Bingo;

    const DAY: usize = 4;

    fn parse(input: String) -> Self::Input {
        let lines = input.lines().collect::<Vec<_>>();
        let numbers = lines[0]
            .split(',')
            .map(str::parse)
            .map(Result::unwrap)
            .collect();
        let boards = lines[1..]
            .split(|line| line.is_empty())
            .filter(|chunk| !chunk.is_empty())
            .map(|chunk| Board {
                rows: chunk
                    .iter()
                    .map(|line| line.split(' ').filter_map(|n| n.parse().ok()).collect())
                    .collect(),
            })
            .collect();
        Bingo { numbers, boards }
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let size = input.boards[0].rows.len();
        let mut marks = input.boards.iter().map(|_| 0).collect::<Vec<usize>>();
        for call in input.numbers.iter() {
            for (board, mark) in input.boards.iter().zip(marks.iter_mut()) {
                for (r, row) in board.rows.iter().enumerate() {
                    for (c, number) in row.iter().enumerate() {
                        if number == call {
                            *mark |= 1 << (r * size + c);
                        }
                    }
                }
                if WINS.iter().any(|&win| (*mark & win) == win) {
                    let mut score = 0;
                    for r in 0..size {
                        for c in 0..size {
                            if *mark & (1 << (r * size + c)) == 0 {
                                score += board.rows[r][c];
                            }
                        }
                    }
                    score *= call;
                    return (input, score.to_string());
                }
            }
        }
        panic!()
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        let size = input.boards[0].rows.len();
        let mut boards = input.boards.clone();
        let mut marks = input.boards.iter().map(|_| 0).collect::<Vec<usize>>();
        let mut won_boards = Vec::new();
        for call in input.numbers.iter() {
            for (i, (board, mark)) in boards.iter().zip(marks.iter_mut()).enumerate() {
                for (r, row) in board.rows.iter().enumerate() {
                    for (c, number) in row.iter().enumerate() {
                        if number == call {
                            *mark |= 1 << (r * size + c);
                        }
                    }
                }
                if WINS.iter().any(|&win| (*mark & win) == win) {
                    if boards.len() == 1 {
                        let mut score = 0;
                        for r in 0..size {
                            for c in 0..size {
                                if *mark & (1 << (r * size + c)) == 0 {
                                    score += board.rows[r][c];
                                }
                            }
                        }
                        score *= call;
                        return score.to_string();
                    }
                    won_boards.push(i);
                }
            }
            won_boards.drain(..).rev().for_each(|i| {
                boards.remove(i);
                marks.remove(i);
            });
        }
        panic!()
    }
}
