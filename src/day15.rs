use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("data/day15.txt").unwrap());

    let mut grid = Vec::new();
    let mut width = 0;
    let mut height = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        width = line.len();
        grid.extend(line.chars().map(|x| x.to_digit(10).unwrap()));
        height += 1;
    }
    let last = find_lowest_risk(grid.as_ref(), width, height);

    println!("{}", last);

    let mut larger_grid = Vec::new();
    for y_out in 0..5 {
        for y in 0..100 {
            for x_out in 0..5 {
                for x in 0..100 {
                    let mut i = grid[x + y * width] + x_out + y_out;
                    while i > 9 { i -= 9 }
                    larger_grid.push(i)
                }
            }
        }
    }

    let last = find_lowest_risk(larger_grid.as_ref(), width * 5, height* 5);

    println!("{}", last);
}

fn find_lowest_risk(grid: &[u32], width: usize, height: usize) -> u32 {
    let mut visited = vec![false; width * height];
    let mut grid = grid.to_vec();
    visited[0] = true;
    grid[0] = 0;
    for i in 0.. {
        for y in 0..height {
            for x in 0..width {
                let index = x + y * width;
                if visited[index] == true {
                    if x != width - 1 {
                        let right = index + 1;
                        check_and_mark(&mut grid, &mut visited, i, index, right)
                    }
                    if x != 0 {
                        let left = index - 1;
                        check_and_mark(&mut grid, &mut visited, i, index, left)
                    }
                    if y != height - 1 {
                        let down = index + width;
                        check_and_mark(&mut grid, &mut visited, i, index, down)
                    }
                    if y != 0 {
                        let up = index - width;
                        check_and_mark(&mut grid, &mut visited, i, index, up)
                    }
                }
            }
        }
        if visited.last().unwrap() == &true { break }
    }

    let last = grid.last().unwrap();
    last.to_owned()
}

#[inline]
fn check_and_mark(grid: &mut Vec<u32>, visited: &mut Vec<bool>, i: u32, index: usize, right: usize) {
    if visited[right] == false && grid[index] + grid[right] == i {
        grid[right] = i;
        visited[right] = true;
    }
}
