use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() {
    let reader = BufReader::new(File::open("data/day8.txt").unwrap());

    let mut ones_fours_sevens_eights = 0;
    let mut total = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let mut values = [""; 10];
        let mut unfound_sixes = Vec::new();
        let mut unfound_fives = Vec::new();

        let mut line_split = line.split(" | ").into_iter();
        let all = line_split.next().unwrap();
        let c = line_split.next().unwrap();

        for a in all.split(" "){
            match a.len() {
                2 => values[1] = a,  // one is the only character with 2 segments
                3 => values[7] = a,
                4 => values[4] = a,
                7 => values[8] = a,
                5 => unfound_fives.push(a),
                6 => unfound_sixes.push(a),
                _ => {
                    println!("{} {}", a, a.len());
                    panic!()
                }
            }
        }
        let extra_of_4: Vec<char> = values[4].chars().filter(|x| !values[1].chars().contains(x)).collect();

        values[5] = unfound_fives // Five is the only character with 5 segments and the middle and left part of four
            .iter()
            .filter(
                |&x|
                    x.chars().contains(&extra_of_4[0]) && x.chars().contains(&extra_of_4[1]) )
            .next()
            .unwrap();

        values[9] = unfound_sixes // Nine is the only character with 6 segments and shares the segments with four
            .iter()
            .filter(
                |&x|
                    {
                        let mut temp = true;
                        for q in values[4].chars() {
                            if !x.chars().contains(&q) {
                                temp = false
                            }
                        }
                        temp
                    })
            .next()
            .unwrap();

        // We need to identify the upper and lower segment of one to differentiate 2 and 3 and 6 and 0
        let upper_of_one;
        let lower_of_one;
        if values[5].chars().contains(&values[1].chars().next().unwrap()) {
            let mut temp = values[1].chars();
            lower_of_one = temp.next().unwrap();
            upper_of_one = temp.next().unwrap();
        } else {
            let mut temp = values[1].chars();
            upper_of_one = temp.next().unwrap();
            lower_of_one = temp.next().unwrap();
        }

        values[3] = unfound_fives // Unlike two (and five) three has both the upper and lower half of one
            .iter()
            .filter(
                |&x|
                    x.chars().contains(&upper_of_one) && x.chars().contains(&lower_of_one) )
            .next()
            .unwrap();
        values[2] = unfound_fives
            .iter()
            .filter(
                |&x|
                    x != &values[3] && x != &values[5] )
            .next()
            .unwrap();

        values[6] = unfound_sixes // Six has lower but not upper half of one unlike zero and nine
            .iter()
            .filter(
                |&x|
                    !x.chars().contains(&upper_of_one) && x.chars().contains(&lower_of_one) )
            .next()
            .unwrap();
        values[0] = unfound_sixes
            .iter()
            .filter(
                |&x|
                    x != &values[6] && x != &values[9] )
            .next()
            .unwrap();

        let mut summed = 0;
        for a in c.split(" "){
            summed *= 10;
            if a.len() != 5 && a.len() != 6 {
                ones_fours_sevens_eights += 1;
            }
            for i in 0..10 {
                // The characters can be in random order
                if values[i as usize].chars().sorted().collect::<Vec<char>>()
                    == a.chars().sorted().collect::<Vec<char>>() {
                    summed += i;
                    break;
                }
            }
        }
        total += summed;
    }
    println!("{}", ones_fours_sevens_eights);
    println!("{}", total);
}