use std::fs;
use std::io::Read;

fn main() {
    let mut line = String::new();
    fs::File::open("inputs/day06.txt")
        .unwrap()
        .read_to_string(&mut line)
        .unwrap();
    line.truncate(line.trim_end().len());
    find_start(&line, 4);
    find_start(&line, 14);
}

fn l264(c: char) -> i8 {
    let b = c as i8;
    match b {
        65..=90 => b - 39,
        97..=122 => b - 97,
        _ => unreachable!(),
    }
}

fn find_start(line: &String, marker: usize) {
    let mut cont = vec![0u64; marker];
    for (i, c) in line.chars().enumerate() {
        let n = 1u64 << l264(c);
        for j in 0..marker - 1 {
            cont[j] = cont[j + 1]
        }
        cont[marker - 1] = n;
        let mask = cont.iter().fold(0, |acc, x| acc | x);
        if mask.count_ones() == marker as u32{
            println!("{}", i + 1);
            break;
        }
    }   
}
