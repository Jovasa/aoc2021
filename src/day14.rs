use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("data/day14.txt").unwrap());

    let mut lines = reader.lines();
    let start: Vec<_> = lines.next().unwrap().unwrap().chars().map(|x| x as u8).collect();
    lines.next();
    let mut pairs= HashMap::new();
    for line in lines {
        let line = line.unwrap();
        let mut temp = line.split(" -> ");
        let mut x = temp.next().unwrap().chars();
        pairs.insert((x.next().unwrap() as u16 ) * 256 + (x.next().unwrap() as u16 ), temp.next().unwrap().chars().next().unwrap() as u8);
    }
    let mut current_pair_count = HashMap::new();
    let mut start_iter = start.iter();
    let mut value = *start_iter.next().unwrap() as u16;
    for &item in start_iter {
        value = value.wrapping_mul(256) + item as u16;
        *current_pair_count.entry(value).or_insert(0) += 1;
    }

    for i in 0..40 {
        let mut temp= HashMap::new();
        for item in current_pair_count {
            let i1 = item.0 & !255 | (pairs[&item.0] as u16);
            let i2 = item.0 & 255  | ((pairs[&item.0] as u16) << 8);
            *temp.entry(i1).or_insert(0) += item.1;
            *temp.entry(i2).or_insert(0) += item.1;
        }
        current_pair_count = temp;
    }

    let mut size_count: HashMap<u8, u64> = HashMap::new();
    size_count.insert(*start.last().unwrap(), 1);
    for i in current_pair_count {
        *size_count.entry((i.0 >> 8u16) as u8).or_insert(0) += i.1;
    }
    let mut sorted: Vec<_> = size_count.iter().collect();
    sorted.sort_by_key(|x| Reverse(x.1));
    println!("{}", sorted[0].1 - sorted.last().unwrap().1);
}