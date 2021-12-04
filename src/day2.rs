use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    first();
    second();
}

fn first() {
    let reader = BufReader::new(File::open("data/day2.txt").unwrap());

    let mut horizontal = 0;
    let mut vertical = 0;

    for line in reader.lines() {

        let line = line.unwrap();
        match line.chars().nth(0) {
            Some('f') => horizontal += line.chars().last().unwrap().to_digit(10).unwrap() as i32,
            Some('d') => vertical += line.chars().last().unwrap().to_digit(10).unwrap() as i32,
            Some('u') => vertical -= line.chars().last().unwrap().to_digit(10).unwrap() as i32,
            _ => panic!(),
        }
    }
    println!("{}", horizontal * vertical)
}

fn second() {
    let file = match File::open("data/day2.txt") {
        Ok(s) => s,
        Err(_) => panic!("File doesn't exist")
    };
    let reader = BufReader::new(file);

    let mut horizontal = 0;
    let mut vertical = 0;
    let mut aim = 0;

    for line in reader.lines() {

        let line = line.unwrap();
        match line.chars().nth(0) {
            Some('f') => {
                let i = line.chars().last().unwrap().to_digit(10).unwrap() as i32;
                horizontal += i;
                vertical += i * aim;
            }
            Some('d') => aim += line.chars().last().unwrap().to_digit(10).unwrap() as i32,
            Some('u') => aim -= line.chars().last().unwrap().to_digit(10).unwrap() as i32,
            _ => panic!(),
        }
    }
    println!("{}", horizontal * vertical)
}