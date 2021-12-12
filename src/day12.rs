use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn has_visited_small_cave_once(route: &Vec<String>) -> bool {
    let mut visited = HashSet::new();
    for i in route {
        if i == &i.to_lowercase() {
            if visited.contains(i) {
                return true;
            } else {
                visited.insert(i);
            }
        }
    }
    false
}

fn main() {
    let reader = BufReader::new(File::open("data/day12.txt").unwrap());

    let mut edges = HashMap::new();
    for lin in reader.lines() {
        let lin = lin.unwrap();
        let mut split = lin.split("-");
        let a = split.next().unwrap().to_owned();
        let b = split.next().unwrap().to_owned();
        edges.entry(a.to_owned()).or_insert(Vec::new()).push(b.to_owned());
        edges.entry(b.to_owned()).or_insert(Vec::new()).push(a.to_owned());
    }

    let mut work_set = vec![vec!["start".to_owned()]];
    let mut in_end = Vec::new();

    while work_set.len() != 0 {
        let mut temp = Vec::new();
        for item in work_set {
            let last = item.last().unwrap();
            for i in &edges[last] {
                if i == "start" { continue; };
                if i != &i.to_lowercase() || !item.contains(i) || !has_visited_small_cave_once(&item) {
                    let mut a = item.clone();
                    a.push(i.to_owned());
                    if i == "end" {
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
