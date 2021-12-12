use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("data/day10.txt").unwrap());

    let mut invalid_score = 0;
    let mut scores = Vec::new();

    for l in reader.lines() {
        let l = l.unwrap();
        match get_leftover_valid(&mut invalid_score, l) {
            Some(stack) => {
                let mut score = 0u64;
                for i in stack.iter().rev() {
                    score *= 5;
                    match i {
                        '(' => score += 1,
                        '[' => score += 2,
                        '{' => score += 3,
                        '<' => score += 4,
                        _ => unreachable!()
                    }
                }
                scores.push(score)
            }
            None => continue
        }
    }
    scores.sort();
    println!("{}", invalid_score);
    println!("{}", scores[scores.len() / 2]);
}

fn get_leftover_valid(invalid_score: &mut i32, l: String) -> Option<Vec<char>> {
    let mut stack = Vec::new();
    for c in l.chars() {
        match c {
            '[' | '{' | '<' | '(' => stack.push(c),
            ']' => {
                let a = stack.pop().unwrap();
                if a != '[' {
                    *invalid_score += 57;
                    return None;
                }
            }
            '>' => {
                let a = stack.pop().unwrap();
                if a != '<' {
                    *invalid_score += 25137;
                    return None;
                }
            }
            ')' => {
                let a = stack.pop().unwrap();
                if a != '(' {
                    *invalid_score += 3;
                    return None;
                }
            }
            '}' => {
                let a = stack.pop().unwrap();
                if a != '{' {
                    *invalid_score += 1197;
                    return None;
                }
            }
            _ => panic!()
        }
    }
    Some(stack)
}
