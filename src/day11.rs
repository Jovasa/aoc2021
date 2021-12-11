use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

struct JellyfishGrid {
    grid: Vec<i16>,
    flashes: i32,
    flashes_this_tick: i32
}

impl JellyfishGrid {
    fn new(data_in: &mut Lines<BufReader<File>>) -> JellyfishGrid {
        let mut grid = Vec::new();
        for line in data_in {
            grid.extend(
                line.unwrap().chars().map(|x| x.to_digit(10).unwrap() as i16)
            )
        }
        JellyfishGrid {
            grid,
            flashes: 0,
            flashes_this_tick: 0,
        }
    }

    fn flash(&mut self, x: i32, y: i32) {
        self.flashes += 1;
        self.flashes_this_tick += 1;
        for i in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
            self.increment(x + i.0, y + i.1);
        }
    }
    fn increment(&mut self, x: i32, y: i32) {
        if 10 <= x || x < 0 || 10 <= y || y < 0 {
            return;
        }
        let index = (x + y * 10) as usize;
        self.grid[index] += 1;
        if self.grid[index] == 10 {
            self.flash(x, y);
        }
    }
    fn clear_flashed(&mut self) {
        self.grid = self.grid.clone().iter().map(|&x| if x >= 10 {0} else {x}).collect()
    }
    fn tick(&mut self) {
        self.flashes_this_tick = 0;
        for i in 0..100 {
            self.increment(i % 10, i / 10);
        }
        self.clear_flashed();
    }
}

fn main() {
    let reader = BufReader::new(File::open("data/day11.txt").unwrap());

    let mut jellyfish_grid = JellyfishGrid::new(&mut reader.lines());
    for _ in 0..100 {
        jellyfish_grid.tick();
    }
    println!("{}", jellyfish_grid.flashes);
    let mut iteration = 101;
    loop {
        jellyfish_grid.tick();
        if jellyfish_grid.flashes_this_tick == 100 { break }
        iteration += 1;
    }
    println!("{}", iteration);
}