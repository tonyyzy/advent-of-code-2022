use advent_of_code_2022::read_to_lines;

fn main() {
    let lines: Vec<String> = read_to_lines("inputs/day02.txt");
    let res: u32 = lines
        .iter()
        .map(|l| {
            score(
                l.bytes().nth(0).unwrap() - 64,
                l.bytes().nth(2).unwrap() - 87,
            ) as u32
        })
        .sum::<u32>();
    println!("{res}");
    let res: u32 = lines
        .iter()
        .map(|l| score1(l.bytes().nth(0).unwrap() - 64, l.chars().nth(2).unwrap()) as u32)
        .sum::<u32>();
    println!("{res}")
}

fn score(left: u8, right: u8) -> u8 {
    match left.cmp(&right) {
        std::cmp::Ordering::Less => {
            if left == 1 && right == 3 {
                right
            } else {
                6 + right
            }
        }
        std::cmp::Ordering::Equal => 3 + right,
        std::cmp::Ordering::Greater => {
            if left == 3 && right == 1 {
                6 + right
            } else {
                right
            }
        }
    }
}

fn score1(left: u8, right: char) -> u8 {
    match right {
        'X' => {
            if left == 1 {
                3
            } else {
                left - 1
            }
        }
        'Y' => 3 + left,
        'Z' => {
            if left == 3 {
                6 + 1
            } else {
                6 + left + 1
            }
        }
        _ => unreachable!(),
    }
}
