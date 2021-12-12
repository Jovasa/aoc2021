use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("data/day3.txt").unwrap());

    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    // Not very happy with this one
    let bits = count_bits(lines.iter().collect::<Vec<&String>>().as_ref());

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in bits.iter() {
        epsilon <<= 1;
        gamma <<= 1;
        if i < &((lines.len() / 2) as u32) {
            epsilon += 1;
        } else {
            gamma += 1;
        }
    }
    println!("{}", epsilon * gamma);

    let oxygen = find_generator_rating(lines.as_ref(), &bits, false);
    let co2 = find_generator_rating(lines.as_ref(), &bits, true);

    let o = i32::from_str_radix(&*oxygen, 2).unwrap();
    let c = i32::from_str_radix(&*co2, 2).unwrap();

    println!("{}", o * c);
}

fn find_generator_rating(lines: &Vec<String>, bits: &Vec<u32>, fewest_bits: bool) -> String {
    let mut temp: Vec<&String> = lines.iter().collect();
    let mut index = 0;
    let mut working_bits = bits.to_vec();
    while temp.len() > 1 && index < 12 {
        let matched_char = if (working_bits[index] >= ((temp.len() + 1) / 2) as u32) ^ fewest_bits
        { '1' } else { '0' };
        temp = temp
            .into_iter()
            .filter(|&x|
                x.chars().nth(index).unwrap() == matched_char)
            .collect();
        working_bits = count_bits(temp.as_ref());
        index += 1;
    }
    let word = temp[0];
    word.to_owned()
}

fn count_bits(lines: &Vec<&String>) -> Vec<u32> {
    let mut bits: Vec<u32> = vec![0; 12];
    for line in lines {
        for (i, a) in line.chars().enumerate() {
            match a {
                '1' => bits[i] += 1,
                _ => continue,
            }
        }
    }
    bits
}
