fn main() {
    let x_limit = (124, 174);
    let y_limit = (-123, -86);

    let x_start = {
        let mut x = 1;
        let mut acc = 1;
        while acc < x_limit.0 {
            x += 1;
            acc += x;
        }
        x
    };
    let y_end = (y_limit.0 + 1i32).abs();

    let mut valid_throws = 0;

    for y_c in y_limit.0..=y_end {
        for x_c in x_start..=x_limit.1 {
            let mut y = y_c;
            let mut x = x_c;
            let mut it = 1;
            while x < x_limit.0 || y > y_limit.1 {
                y += y_c - it;
                x += (x_c - it).max(0);
                it += 1;
            }
            if x_limit.0 <= x && x <= x_limit.1 && y_limit.0 <= y && y <= y_limit.1 {
                valid_throws += 1;
            }
        }
    }
    println!("{}", valid_throws);
}