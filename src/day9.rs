use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[derive(Debug, Clone)]
struct HeightMap {
    data: Vec<i32>,
    basin_size: Vec<i32>,
    height: usize,
    width: usize,
}

impl HeightMap {
    fn new(data_in: &mut Lines<BufReader<File>>) -> HeightMap {
        let mut data = Vec::new();
        let mut width = 0;
        let mut height = 0;
        for line in data_in {
            let line = line.unwrap();
            width = line.len();
            height += 1;
            data.extend(line.chars().map(|x| x.to_digit(10).unwrap() as i32))
        }
        let basin_size = data.clone().into_iter().map(|x| if x == 9 { i32::MAX } else { x }).collect();

        HeightMap {
            data,
            basin_size,
            height,
            width,
        }
    }

    fn low_point_plus_one_or_zero(&self, x: usize, y: usize) -> i32 {
        let index = x + y * self.width;
        let current = self.data[index];
        if x != 0 && self.data[index - 1] <= current {
            return 0;
        }
        if x != self.width - 1 && self.data[index + 1] <= current {
            return 0;
        }
        if y != 0 && self.data[index - self.width] <= current {
            return 0;
        }
        if y != self.height - 1 && self.data[index + self.width] <= current {
            return 0;
        }
        current + 1
    }

    fn mark_all_neighbours(&mut self, x: usize, y: usize, mark: i32) {
        let index = x + y * self.width;
        if self.basin_size[index] == i32::MAX || self.basin_size[index] == mark {
            return;
        }
        self.basin_size[index] = mark;
        if x != 0 { self.mark_all_neighbours(x - 1, y, mark) }
        if y != 0 { self.mark_all_neighbours(x, y - 1, mark) }
        if x != self.width - 1 { self.mark_all_neighbours(x + 1, y, mark) }
        if y != self.height - 1 { self.mark_all_neighbours(x, y + 1, mark) }
    }
}

fn main() {
    let reader = BufReader::new(File::open("data/day9.txt").unwrap());

    let mut height_map = HeightMap::new(&mut reader.lines());

    let mut low_points = 0;

    for y in 0usize..height_map.height {
        for x in 0usize..height_map.width {
            let i = height_map.low_point_plus_one_or_zero(x, y);
            low_points += i;
            if i != 0 {
                height_map.mark_all_neighbours(x, y, low_points + 10);
            }
        }
    }
    println!("{}", low_points);

    let mut size_count = HashMap::new();
    for i in height_map.basin_size {
        *size_count.entry(i).or_insert(0) += 1;
    }
    let mut sorted: Vec<_> = size_count.iter().collect();
    sorted.sort_by_key(|x| Reverse(x.1));
    println!("{} * {} * {} = {}", sorted[1].1, sorted[2].1, sorted[3].1, sorted[1].1 * sorted[2].1 * sorted[3].1)
}
