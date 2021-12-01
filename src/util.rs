use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

pub fn read_file_of_numbers<T: FromStr>(filename: &str) -> Vec<T> {
    let file = match File::open(filename) {
        Ok(s) => s,
        Err(_) => panic!("File doesn't exist")
    };
    let reader = BufReader::new(file);

    let mut numbers: Vec<T> = Vec::new();

    for line in reader.lines() {
        let a = match line {
            Ok(s) => match s.parse::<T>(){
                Ok(i) => i,
                Err(_) => panic!("Failed to convert {} to int.", s)
            },
            Err(_) => panic!()
        };

        numbers.push(a);
    }
    numbers
}