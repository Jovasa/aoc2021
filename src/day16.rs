use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct HexDecoder {
    buffer: u32,
    bits_left: usize,
    data: String,
    hex_read: usize,
    bits_read: usize,
}

impl HexDecoder {
    fn new(data: String) -> HexDecoder {
        HexDecoder{
            buffer: 0,
            bits_left: 0,
            data,
            hex_read: 0,
            bits_read: 0
        }
    }
}

impl Iterator for HexDecoder {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bits_left == 0 {
            if self.hex_read == self.data.len() {
                return None;
            }
            self.buffer = self.data.chars().nth(self.hex_read).unwrap().to_digit(16).unwrap() << 28;
            self.hex_read += 1;
            self.bits_left += 4;
        }
        let return_value = (self.buffer & 1 << 31) >> 31;
        self.buffer <<= 1;
        self.bits_left -= 1;
        self.bits_read += 1;
        Some(return_value)
    }


}

fn read_n_bytes(r: &mut HexDecoder, n: usize) -> u64 {
    r.by_ref().take(n).fold(0, |acc, x| {
        (acc << 1) + x
    } ) as u64
}

fn read_packet(r: &mut HexDecoder) -> u64 {
    let _version = read_n_bytes(r, 3);
    let mut value = 0;
    let id = read_n_bytes(r, 3);
    if id == 4 {
        let mut v = read_n_bytes(r, 5);
        while v & 0x10 != 0 {
            value <<= 4;
            value += v & 0xf;
            v = read_n_bytes(r, 5);
        }
        value <<= 4;
        value += v  & 0xf;
        value
    }
    else {
        let mut values = Vec::new();
        match read_n_bytes(r, 1) {
            0 => {
                let length = read_n_bytes(r, 15);
                let sub_length = r.bits_read + length as usize;
                while r.bits_read < sub_length {
                    values.push(read_packet(r));
                }
            },
            1 => {
                let num_packets = read_n_bytes(r, 11);
                for _ in 0..num_packets {
                    values.push(read_packet(r));
                }
            },
            _ => unreachable!()
        }
        match id {
            0 => {
                return values.iter().sum();
            }
            1 => {
                return values.iter().fold(1, |acc, x | acc * x);
            }
            2 => {return values.iter().min().unwrap().to_owned();}
            3 => {return values.iter().max().unwrap().to_owned();}
            5 => {return (values[0] > values[1]) as u64}
            6 => {return (values[0] < values[1]) as u64}
            7 => {return (values[0] == values[1]) as u64}
            _ => unreachable!()
        }
    }
}


fn main() {
    let reader = BufReader::new(File::open("data/day16.txt").unwrap());

    for line in reader.lines() {
        let line = line.unwrap();
        let t = HexDecoder::new(line);
        let mut a = t.into_iter();
        let mut v = 0;
        while &a.hex_read != &a.data.len() {
            let version = read_packet(&mut a);
            v += version;
        }
        println!("{}", v);
        break
    }

}