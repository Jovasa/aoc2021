use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("data/day6.txt").unwrap());

    let mut phases = [0u64; 9];
    for a in reader.lines().next().unwrap().unwrap()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap()) {
        phases[a as usize] += 1;
    }
    count_fish_after_n_days(&phases, 80);
    count_fish_after_n_days(&phases, 256);
}

fn count_fish_after_n_days(phases: &[u64; 9], n: i32) {
    let mut phases = phases.to_owned();
    for _ in 0..n {
        let roll_over = phases[0];
        for i in 1..=8 {
            phases[i - 1] = phases[i];
        }
        phases[6] += roll_over;
        phases[8] = roll_over;
    }
    println!("{}", phases.iter().sum::<u64>());
}
