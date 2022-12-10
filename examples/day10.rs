use advent_of_code_2022::read_to_lines;

fn main() {
    let lines = read_to_lines("inputs/day10.txt");
    let mut cycle = 0;
    let mut x = 1;
    let mut sum = 0;
    let mut screen = vec![vec!["."; 40]; 6];
    for line in lines {
        match &line[..4] {
            "addx" => {
                let val = line[5..].parse::<i32>().unwrap();
                cycle += 1;
                draw_pixel(&mut screen, cycle, x);
                if cycle % 40 == 20 {
                    sum += cycle * x;
                }
                cycle += 1;
                draw_pixel(&mut screen, cycle, x);
                if cycle % 40 == 20 {
                    sum += cycle * x;
                }
                x += val;
            }
            "noop" => {
                cycle += 1;
                draw_pixel(&mut screen, cycle, x);
                if cycle % 40 == 20 {
                    sum += cycle * x;
                }
            }
            _ => unreachable!(),
        }
    }
    println!("{sum}");
    for row in screen {
        println!("{}", row.join(""));
    }
}

fn draw_pixel(screen: &mut Vec<Vec<&str>>, cycle: i32, sprite: i32) {
    let pos = (cycle - 1) % 40;
    let row = ((cycle - 1) / 40) as usize;
    if pos >= sprite - 1 && pos <= sprite + 1 {
        screen[row as usize][pos as usize] = "#";
    }
}
