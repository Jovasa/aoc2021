use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("data/day7.txt").unwrap());

    let heights: Vec<i64> = reader.lines().next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut fuel_usage: Vec<i64> = Vec::new();

    for h in 0..*heights.iter().max().unwrap() {
        fuel_usage.push(
            heights
                .iter()
                .map(|x| (h - x).abs()).into_iter()
                .sum()
        );
    }
    println!("{}", *fuel_usage.iter().min().unwrap());

    fuel_usage.clear();

    for h in 0..*heights.iter().max().unwrap() {
        fuel_usage.push(
            heights
                .iter()
                .map(|x| {
                    let t = (h - x).abs();
                    t * (t + 1) / 2
                }).into_iter()
                .sum()
        );
    }
    println!("{}", *fuel_usage.iter().min().unwrap());
}
