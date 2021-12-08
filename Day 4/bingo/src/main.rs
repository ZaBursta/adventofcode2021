use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::str::Lines;
use std::collections::HashMap;
use ansi_term::Style;
use ansi_term::Colour::{Yellow};

#[derive(Debug, Clone, Copy)]
struct BingoSquare {
    value: u32,
    hit: bool,
}

impl Default for BingoSquare {
    fn default() -> BingoSquare {
        BingoSquare {
            value: 0,
            hit: false,
        }
    }
}

impl Display for BingoSquare {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", if self.hit { Yellow.bold().paint(self.value.to_string()) } else { Style::new().paint(self.value.to_string()) })
    }
}

#[derive(Debug, Clone, Copy)]
struct BingoBoard {
    board: [[BingoSquare; 5]; 5],
    has_won: bool,
}

impl Default for BingoBoard {
    fn default() -> BingoBoard {
        BingoBoard {
            board: Default::default(),
            has_won: false,
        }
    }
}

impl Display for BingoBoard {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        println!("");
        println!("Has {}won", if !self.has_won { "not "} else { "" });
        return Ok(for row in self.board {
             println!("{} {} {} {} {}", row[0], row[1], row[2], row[3], row[4]);
        })
    }
}


fn parse_bingo_board(lines: &mut Lines) -> BingoBoard {
    let mut bingo_board: BingoBoard = Default::default();

    for row in 0..5 {
        bingo_board.board[row] = lines
        .next()
        .expect("Malformed Bingo Board")
        .trim()
        .split_whitespace()
        .map(|x|
            BingoSquare {
                value:x.parse::<u32>().unwrap(),
                hit:false
            }
        )
        .collect::<Vec<BingoSquare>>()
        .try_into()
        .unwrap();
    }

    return bingo_board;
}

fn has_bingo_with_new_hit(bingo_board: &BingoBoard, row_index: usize, column_index: usize) -> bool {
    let mut has_bingo = true;
    for square in bingo_board.board[row_index] {
        if !square.hit {
            has_bingo = false;
            break;
        }
    }

    if has_bingo {
        return true;
    }

    has_bingo = true;
    for row in bingo_board.board {
        if !row[column_index].hit {
            has_bingo = false;
            break;
        }
    }

    return has_bingo;
}

fn score_board(bingo_board: &BingoBoard, last_call: u32) -> u32 {
    let mut score = 0;
    bingo_board.board
        .iter()
        .for_each(|row|
            row.iter()
                .for_each(|square|
                    if !square.hit { score += square.value }
                )
        );
    return score * last_call;
}

fn generate_square_to_board_hash_map(bingo_boards: &mut Vec<BingoBoard>) -> HashMap<u32, Vec<(usize, (usize, usize))>> {
    let mut square_to_board_map = HashMap::new();

    bingo_boards
    .iter()
    .enumerate()
    .for_each(|(board_index, bingo_board)| {
        bingo_board.board
            .iter()
            .enumerate()
            .for_each(|(row_index, row)|
                row
                    .iter()
                    .enumerate()
                    .for_each(|(column_index, square)| {
                        square_to_board_map
                            .entry(square.value)
                            .and_modify(|v: &mut Vec<(usize, (usize, usize))>| v.push((board_index, (row_index, column_index))))
                            .or_insert(vec![(board_index, (row_index, column_index))]);
                    }
                )
            )
    });

    return square_to_board_map;
}

fn part1(bingo_calls: &Vec<u32>, bingo_boards: &mut Vec<BingoBoard>) {
    let mut square_to_board_map = generate_square_to_board_hash_map(bingo_boards);

    for call in bingo_calls {
        let hit_board_maps = square_to_board_map.entry(*call).or_default();

        for (board_index, (row_index, column_index)) in hit_board_maps {
            let mut bingo_board = bingo_boards.get_mut(*board_index).expect("Invalid Index");

            bingo_board.board[*row_index][*column_index].hit = true;
            if has_bingo_with_new_hit(bingo_board, *row_index, *column_index) {
                println!("First Board won with {}, on board {}", call, bingo_board);
                println!("Score: {}", score_board(bingo_board, *call));
                return;
            }
        }
    }
}

fn part2(bingo_calls: &Vec<u32>, bingo_boards: &mut Vec<BingoBoard>) {
    let mut square_to_board_map = generate_square_to_board_hash_map(bingo_boards);

    let mut winning_board_count = 0;
    let total_board_count = bingo_boards.len();

    for call in bingo_calls {
        let hit_board_maps = square_to_board_map.entry(*call).or_default();

        for (board_index, (row_index, column_index)) in hit_board_maps {
            let mut bingo_board = bingo_boards.get_mut(*board_index).expect("Invalid Index");
            if bingo_board.has_won {
                continue;
            }

            bingo_board.board[*row_index][*column_index].hit = true;
            if has_bingo_with_new_hit(bingo_board, *row_index, *column_index) {
                bingo_board.has_won = true;
                winning_board_count += 1;
                if winning_board_count == total_board_count {
                    println!("Last Board won with {}, on board {}", call, bingo_board);
                    println!("Score: {}", score_board(bingo_board, *call));
                }
            }
        }
    }
}


fn main() {
    let input = include_str!("../input.txt");
    let mut lines = input.lines();
    let bingo_calls: Vec<u32> = lines
        .next()
        .expect("Missing Bingo Call Line!")
        .trim()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let mut bingo_boards: Vec<BingoBoard> = <Vec<BingoBoard>>::new();

    while lines.next().is_some() {
        bingo_boards.push(parse_bingo_board(&mut lines));
    }

    part1(&bingo_calls, &mut bingo_boards);
    part2(&bingo_calls, &mut bingo_boards);
}
