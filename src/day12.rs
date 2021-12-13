use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn has_visited_small_cave_once(route: &Vec<u64>) -> bool {
    let mut visited = 0u64;
    for i in route {
        if (i & 1 << 63) == 0  {
            if visited & i != 0 {
                return true;
            } else {
                visited |= i;
            }
        }
    }
    false
}

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
                let is_uppercase = t.chars().next().unwrap().is_uppercase();
                used_edges.insert(t.to_owned(), (1 << i1) + ((is_uppercase as u64 ) << 63 ));
            }
            temp_stuff.push(used_edges[&t]);
        }
        edges.entry(temp_stuff[0]).or_insert(Vec::new()).push(temp_stuff[1]);
        edges.entry(temp_stuff[1]).or_insert(Vec::new()).push(temp_stuff[0]);
    }

    let mut work_set = vec![vec![0]];
    let mut in_end = Vec::new();

    while work_set.len() != 0 {
        let mut temp = Vec::new();
        for item in work_set {
            let last = item.last().unwrap();
            for i in &edges[last] {
                if *i == 0 { continue; };
                if (i & 1 << 63) != 0 || !item.contains(i) || !has_visited_small_cave_once(&item) {
                    let mut a = item.clone();
                    a.push(i.to_owned());
                    if *i == 1 {
                        in_end.push(a);
                    } else {
                        temp.push(a);
                    }
                }
            }
        }
        work_set = temp;
    }
    println!("{}", in_end.len())
}
