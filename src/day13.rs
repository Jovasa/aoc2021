use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("data/day13.txt").unwrap());

    let mut points = HashSet::new();
    let mut lines = reader.lines();
    for line in lines.by_ref() {
        let line = line.unwrap();
        if line.len() == 0 {
            break;
        }
        let temp: Vec<_> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        points.insert((temp[0], temp[1]));
    }
    println!("{}", points.len());
    let mut width = 0;
    let mut height = 0;
    for line in lines {
        let line = line.unwrap();
        let temp: Vec<&str> = line.split("=").collect();
        let axis = temp[0].chars().last().unwrap();
        let value = temp[1].parse::<i32>().unwrap();
        points = points.iter().map(|&x|
            if axis == 'x' {
                if x.0 < value {
                    x
                } else {
                    (value - (x.0 - value), x.1)
                }
            } else {
                if x.1 < value {
                    x
                } else {
                    (x.0, value - (x.1 - value))
                }
            }
        ).collect();
        if axis == 'x' {
            width = value;
        } else {
            height = value;
        }

        println!("{}", points.len());
    }
    for y in 0..height {
        for x in 0..width {
            if points.contains(&(x, y)) {
                print!("#")
            } else {
                print!(" ")
            }
        }
        println!();
    }
}
