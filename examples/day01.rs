use advent_of_code_2022::read_to_lines;

fn main() {
    let lines = read_to_lines("inputs/day01.txt");
    let mut highest = [0; 3];
    let mut acc = 0;
    for line in lines {
        match line.as_str() {
            "" => {
                if acc > highest[0] {
                    highest[2] = highest[1];
                    highest[1] = highest[0];
                    highest[0] = acc
                } else if acc > highest[1] {
                    highest[2] = highest[1];
                    highest[1] = acc
                } else if acc > highest[2] {
                    highest[2] = acc
                }
                acc = 0
            }
            _ => {
                let num = line.parse::<i32>().unwrap();
                acc += num
            }
        }
    }
    println!("{}", highest[0]);
    println!("{}", highest.iter().sum::<i32>());
}
