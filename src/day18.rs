use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::Chars;

struct SnailfishNumber {
    numbers: Vec<u32>,
    depths: Vec<u32>,
}

impl SnailfishNumber {
    fn new(chars: &mut Chars) -> SnailfishNumber {
        let mut numbers = Vec::new();
        let mut depths = Vec::new();

        let mut current_depth = 0;
        for item in chars {
            match item {
                '[' => current_depth += 1,
                ']' => current_depth -= 1,
                ',' => continue,
                x => {
                    numbers.push(x.to_digit(10).unwrap());
                    depths.push(current_depth);
                }
            }
        }

        SnailfishNumber {
            numbers,
            depths,
        }
    }

    fn add(first: SnailfishNumber, second: SnailfishNumber) -> SnailfishNumber {
        let mut numbers = first.numbers;
        numbers.extend(second.numbers.into_iter());
        let mut depths = first.depths;
        depths.extend(second.depths.into_iter());
        depths = depths.into_iter().map(|x| x + 1).collect();
        SnailfishNumber {
            numbers,
            depths,
        }
    }

    fn explode(&mut self) {
        let mut items = self.numbers.len();
        let mut index = 0;
        while index < items {
            if self.depths[index] == 5 {
                if index != 0 {
                    self.numbers[index - 1] += self.numbers[index];
                }
                self.numbers[index] = 0;
                self.depths[index] = 4;
                if index < items - 2 {
                    self.numbers[index + 2] += self.numbers[index + 1];
                }
                self.numbers.remove(index + 1);
                self.depths.remove(index + 1);
                items -= 1;
            }
            index += 1;
        }
    }

    fn split(&mut self) {
        let mut items = self.numbers.len();
        let mut index = 0;

        while index < items {
            if self.numbers[index] > 9 {
                let half = self.numbers[index] / 2;
                let rounded_up = half + (self.numbers[index] & 1);
                self.numbers[index] = half;
                self.depths[index] += 1;
                self.numbers.insert(index + 1, rounded_up);
                self.depths.insert(index + 1, self.depths[index]);
                if self.depths[index] == 5 {
                    return;
                }
                index += 1;
                items += 1;
            }
            index += 1;
        }
    }

    fn get_magnitude(&self) -> u64 {
        let t: &Vec<u32> = self.numbers.as_ref();
        let mut work = t.into_iter().map(|&x| x as u64).collect::<Vec<u64>>();
        let mut depths = self.depths.clone();
        for depth in (1..=4).rev() {
            let mut items = work.len();
            let mut index = 0;
            while index < items {
                if depths[index] == depth {
                    let val = work[index] * 3 + work[index + 1] * 2;
                    work[index] = val;
                    depths[index] -= 1;
                    work.remove(index + 1);
                    depths.remove(index + 1);
                    items -= 1;
                }
                index += 1
            }
        }
        work[0]
    }

    fn can_split(&self) -> bool {
        for item in &self.numbers {
            if item > &9 {
                return true;
            }
        }
        false
    }

    fn can_explode(&self) -> bool {
        let t: &Vec<u32> = self.depths.as_ref();
        t.into_iter().max() == Some(&5)
    }
}


fn main() {
    let reader = BufReader::new(File::open("data/day18.txt").unwrap());

    let mut lines = reader.lines();
    let t = lines.next().unwrap().unwrap();
    let mut r = SnailfishNumber::new(&mut t.chars());
    for line in lines {
        let line = line.unwrap();
        r = SnailfishNumber::add(r, SnailfishNumber::new(&mut line.chars()));
        while r.can_explode() || r.can_split() {
            if r.can_explode() {
                r.explode()
            }
            else if r.can_split() {
                r.split()
            }
        }
    }
    println!("{}", r.get_magnitude());
}