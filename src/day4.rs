use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[derive(Debug, Clone)]
struct BingoBoard {
    values: Vec<i32>,
    lines: Vec<Vec<i32>>,
    score: i32,
}

impl BingoBoard {
    fn new(data: &mut Lines<BufReader<File>>) -> BingoBoard {
        data.next();
        let mut values = Vec::new();
        let mut lines = Vec::new();
        for _ in 0..5 {
            values.extend(
                data
                    .next().unwrap().unwrap()
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            );
        }
        for i in 0..5 {
            lines.push(
                values.to_owned().into_iter().skip(i * 5).take(5).collect::<Vec<i32>>()
            );
            lines.push(
                values.to_owned().into_iter().skip(i).step_by(5).take(5).collect()
            );
        }
        BingoBoard {
            values,
            lines,
            score: 0,
        }
    }

    fn has_lines(&self) -> bool {
        self.lines.len() > 0
    }

    fn remove_lines(&mut self, draw: i32) {
        self.lines = self.lines
            .iter()
            .filter(|x| !x.contains(&draw))
            .cloned()
            .collect();
        if self.values.contains(&draw) {
            self.score += draw;
        }
    }
}

fn main() {
    let reader = BufReader::new(File::open("data/day4.txt").unwrap());

    let mut lines = reader.lines();

    let draw_order: Vec<i32> = lines
        .next().unwrap().unwrap()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut boards = Vec::new();

    for _ in 0..100 {
        boards.push(
            BingoBoard::new(&mut lines)
        );
    }

    let mut valid_boards;
    let mut winning_draw = 0;
    let mut last_valid_board = 0;
    let mut last_to_win = false;
    for i in (0..draw_order.len()).rev() {
        valid_boards = 0;
        let draw = draw_order[i];
        let mut c = 0;
        for a in &mut boards {
            a.remove_lines(draw);
            if a.has_lines() {
                last_valid_board = c;
                valid_boards += 1
            } else if !last_to_win {
                println!("Last to win: {} {} {}", draw, a.score - draw, draw * (a.score - draw));
                last_to_win = true;
            }
            c += 1;
        }
        if valid_boards == 0 {
            winning_draw = draw_order[i];
            break;
        }
    }
    let valid_board = &boards[last_valid_board];
    let score = valid_board.score - winning_draw;
    println!("First to win: {} {} {}", winning_draw, score, score * winning_draw as i32);
}
