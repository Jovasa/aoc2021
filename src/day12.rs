use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

const UPPERCASE_MASK: u64 = 0x7fff << 15;
const VISITED_SMALL: u64 = 1u64 << 63;
const LAST_MASK: u64 = 0x3fffffffu64 << 30; // 30 bits

fn main() {
    let reader = BufReader::new(File::open("data/day12.txt").unwrap());

    let mut edges = HashMap::new();
    let mut used_edges = HashMap::new();
    used_edges.insert("start".to_owned(), 0u64);
    used_edges.insert("end".to_owned(), 1);

    for lin in reader.lines() {
        let lin = lin.unwrap();
        let split = lin.split("-");
        let mut temp_stuff = Vec::new();
        for i in split {
            let i1 = used_edges.len();
            let t = i.to_owned();
            if !used_edges.contains_key(&t) {
                let is_uppercase = if t.chars().next().unwrap().is_uppercase() {15} else { 0 };
                used_edges.insert(t.to_owned(), 1 << (i1 + is_uppercase));
            }
            temp_stuff.push(used_edges[&t]);
        }

        edges.entry(temp_stuff[0]).or_insert(Vec::new()).push(temp_stuff[1]);
        edges.entry(temp_stuff[1]).or_insert(Vec::new()).push(temp_stuff[0]);
    }

    let mut work_set = vec![0];
    let mut total = 0;
    let mut in_end = Vec::new();

    while work_set.len() != 0 {
        let mut temp = Vec::new();
        for item in work_set {
            let last = (item & LAST_MASK) >> 30;
            for i in &edges[&last] {
                if *i == 0 { continue; };
                let not_visited_before = item & i == 0;
                let large_cavern = (i & UPPERCASE_MASK) != 0;
                if large_cavern || not_visited_before || item & VISITED_SMALL == 0 {
                    let mut a = item | i;
                    a &= !LAST_MASK;
                    a |= i << 30;
                    if !large_cavern && !not_visited_before { a |= VISITED_SMALL }
                    if *i == 1 {
                        total += 1;
                        in_end.push(a);
                    } else {
                        temp.push(a);
                    }
                }
            }
        }
        work_set = temp;
    }
    println!("{}", total)
}
