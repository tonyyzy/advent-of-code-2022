use advent_of_code_2022::read_to_lines;

fn main() {
    let lines = read_to_lines("inputs/day04.txt");
    let mut counter = 0;
    let mut counter1 = 0;
    for line in lines.iter() {
        let (left, right) = line.split_once(",").unwrap();
        let (lr, le) = left.split_once("-").unwrap();
        let (rr, re) = right.split_once("-").unwrap();
        let lr = lr.parse::<u32>().unwrap();
        let le = le.parse::<u32>().unwrap();
        let rr = rr.parse::<u32>().unwrap();
        let re = re.parse::<u32>().unwrap();
        if (lr <= rr && le >= re) || (rr <= lr && re >= le) {
            counter += 1
        }
        if (lr <= re && le >= re) || (rr <= le && re >= le) {
            counter1 += 1
        }
    }
    dbg!(counter, counter1);
}
