use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    let reader = BufReader::new(File::open("data/day5.txt").unwrap());

    let mut points = HashSet::new();
    let mut seconds = HashSet::new();

    let pattern = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    for line in reader.lines() {
        let line = line.unwrap();
        let cap: Vec<i32> = pattern.captures(&line).unwrap()
            .iter().skip(1)
            .map(|x|
                x.unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap()
            ).collect();

        let x = if cap[0] - cap[2] < 0 { 1 } else if cap[0] - cap[2] > 0 { -1 } else { 0 };
        let y = if cap[1] - cap[3] < 0 { 1 } else if cap[1] - cap[3] > 0 { -1 } else { 0 };

        // Uncomment this part for the first task
        // if x != 0 && y != 0 {
        //     continue
        // }

        let mut i = 0;
        loop {
            let temp = (cap[0] + x * i, cap[1] + y * i);
            if points.contains(&temp) && !seconds.contains(&temp) {
                seconds.insert(temp);
            }
            points.insert(temp);
            if temp.0 == cap[2] && temp.1 == cap[3] {
                break
            }
            i += 1;
        }
    }
    println!("{}", seconds.len());
}