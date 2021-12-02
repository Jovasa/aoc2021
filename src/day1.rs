use crate::util::read_file_of_numbers;

mod util;

fn main() {
    let numbers = read_file_of_numbers::<i32>("./data/day1.txt");

    first(numbers.as_ref());
    second(numbers.as_ref());
}

fn first(numbers: &Vec<i32>) {
    let mut increasing = 0;
    for a in numbers.windows(2) {
        if let [first, second] = a {
            increasing += (first < second) as i32;
        }
    }
    println!("{}", increasing);
}

fn second(numbers: &Vec<i32>) {
    let i = numbers
        .windows(3)
        .map(|pair| (pair[0] + pair[1] + pair[2]))
        .collect::<Vec<i32>>();
    first(i.as_ref());
}
