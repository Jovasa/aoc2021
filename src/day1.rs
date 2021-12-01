use crate::util::read_file_of_numbers;
use itertools::izip;

mod util;

fn main() {
    let numbers = read_file_of_numbers::<i32>("./data/day1.txt");

    first(numbers.as_ref());
    second(numbers.as_ref());
}

fn first(numbers: &Vec<i32>) {
    let mut increasing = 0;
    let mut previous = numbers.first().unwrap();

    for number in numbers.iter().skip(1) {
        if number > previous {
            increasing += 1;
        }
        previous = number;
    }
    println!("{}", increasing);
}

fn second(numbers: &Vec<i32>) {
    let n: Vec<i32> = izip!(numbers.iter(), numbers.iter().skip(1), numbers.iter().skip(2)).map(|(x, y, z)| x +y + z).collect();
    first(n.as_ref());
}
