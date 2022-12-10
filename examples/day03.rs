use advent_of_code_2022::read_to_lines;

fn main() {
    let lines = read_to_lines("inputs/day03.txt");
    let mut sum_prio = 0;
    for line in lines.iter() {
        let (l, r) = line.split_at(line.len() / 2);
        let lacc = sack2int(l);
        let racc = sack2int(r);
        let common = lacc & racc;
        sum_prio += 64 - common.leading_zeros() - 1;
    }
    println!("{sum_prio}");

    let mut sum_prio = 0;
    for g in lines
        .iter()
        .map(|a| sack2int(a))
        .collect::<Vec<u64>>()
        .chunks(3)
    {
        let common = g[0] & g[1] & g[2];
        sum_prio += 64 - common.leading_zeros() - 1;
    }
    println!("{sum_prio}");
}

fn letter2priority(c: char) -> i8 {
    let b = c as i8;
    match b {
        65..=90 => b - 38,
        97..=122 => b - 96,
        _ => unreachable!(),
    }
}

fn sack2int(s: &str) -> u64 {
    s.chars()
        .map(|a| letter2priority(a))
        .fold(0u64, |acc, a| acc | 1 << a)
}
