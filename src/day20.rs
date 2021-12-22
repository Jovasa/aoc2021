use std::fs::File;
use std::io::{BufRead, BufReader};

fn extend_image(img: Vec<bool>, height: &mut usize, width: &mut usize) -> Vec<bool> {
    *width += 2;
    *height += 2;
    let mut img_out = Vec::new();
    let mut t = img.iter();
    for _ in 0..*width {
        img_out.push(false);
    }
    for _y in 0..*height-2 {
        img_out.push(false);
        img_out.extend(t.by_ref().take(*width-2));
        img_out.push(false);
    }
    for _ in 0..*width {
        img_out.push(false);
    }
    img_out
}

fn enhance(img: Vec<bool>, iae: &Vec<bool>, width: &usize, height: &usize, edge: bool) -> Vec<bool> {
    let window: Vec<(i32, i32)> = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 0), (0, 1), (1, -1), (1, 0), (1, 1)];
    let mut out_img = img.clone();
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let mut index = 0;
            for (t1, t2) in &window {
                let local_x = (x as i32 + *t2) as usize;
                let local_y = (y as i32 + *t1) as usize;
                index <<= 1;
                if local_x <= 1 || local_y <= 1 || local_y >= height - 2 || local_x >= width - 2 {
                    index += edge as usize;
                } else {
                    index += img[local_x + local_y * width] as usize;
                };
            }
            out_img[x + y*width] = iae[index];
        }
    }
    out_img
}

fn print_image(img: &Vec<bool>, width: &usize, height: &usize) {
    for y in 1..*height-1 {
        for x in 1..*width-1 {
            print!("{}", if img[x + y * width] {'#'} else {'.'});
        }
        println!();
    }
    println!();
}

fn main() {
    let reader = BufReader::new(File::open("data/day20.txt").unwrap());

    let mut r = reader.lines();
    let iea = r.next().unwrap().unwrap().chars().map(|x| x == '#').collect::<Vec<bool>>();
    r.next();

    let mut image = Vec::new();
    let mut width = 0;
    let mut height = 0;
    for line in r {
        let line = line.unwrap();
        width = line.len();
        height += 1;
        image.extend(line.chars().map(|x| x == '#'));
    }

    image = extend_image(image, &mut width, &mut height);
    image = extend_image(image, &mut width, &mut height);
    image = enhance(image, &iea, &width, &height, false);
    let mut last_infinity = false;
    for _ in 0..49 {
        image = extend_image(image, &mut width, &mut height);
        last_infinity = if last_infinity {iea[511]} else { iea[0] };
        image = enhance(image, &iea, &width, &height, last_infinity);
    }
    print_image(&image, &width, &height);
    let s = image.iter().fold(0usize, |acc, &x| acc + x as usize);
    println!("{}", s);
}