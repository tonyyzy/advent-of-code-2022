use advent_of_code_2022::read_to_lines;
use regex::Regex;

fn main() {
    let lines = read_to_lines("inputs/day15.txt");
    let re = Regex::new(r"x=(.\d*), y=(.\d*).*x=(.\d*), y=(.\d*)").unwrap();
    // let mut grid = HashSet::new();
    let mut range = vec![];
    let row = 2000000;
    for line in lines.iter().map(|f| re.captures(f)) {
        if let Some(vals) = line {
            let sx = vals[1].parse::<i32>().unwrap();
            let sy = vals[2].parse::<i32>().unwrap();
            let bx = vals[3].parse::<i32>().unwrap();
            let by = vals[4].parse::<i32>().unwrap();
            if let Some(r) = add_nb((sx, sy), (bx, by), row) {
                range.push(r);
            }
        }
    }
    let r = merge_range(range);
    println!("{}", r.1 - r.0);

    let lines = read_to_lines("inputs/day15.txt");
    let re = Regex::new(r"x=(.\d*), y=(.\d*).*x=(.\d*), y=(.\d*)").unwrap();
    let mut input = vec![];
    for line in lines.iter().map(|f| re.captures(f)) {
        if let Some(vals) = line {
            let sx = vals[1].parse::<i32>().unwrap();
            let sy = vals[2].parse::<i32>().unwrap();
            let bx = vals[3].parse::<i32>().unwrap();
            let by = vals[4].parse::<i32>().unwrap();
            input.push(((sx, sy), (bx, by)));
        }
    }
    for row in 0..=4000000 {
        let mut range = vec![];

        for (sensor, beacon) in input.iter() {
            if let Some(r) = add_nb(*sensor, *beacon, row) {
                range.push(r);
            }
        }
        if let Some(y) = check_gap(range) {
            println!("{}", 4000000 * y as i64 + row as i64)
        }
    }
}

fn add_nb(sensor: (i32, i32), beacon: (i32, i32), row: i32) -> Option<(i32, i32)> {
    let distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
    let dist = distance - (sensor.1 - row).abs();
    if dist < 0 {
        return None;
    } else {
        Some((sensor.0 - dist, sensor.0 + dist))
    }
}

fn merge_range(range: Vec<(i32, i32)>) -> (i32, i32) {
    let mut out = range[0];
    for &(l, r) in range[1..].iter() {
        if l < out.0 {
            if r > out.1 {
                out = (l, r)
            } else {
                out = (l, out.1)
            }
        } else {
            if r > out.1 {
                out = (out.0, r)
            }
        }
    }

    (out.0, out.1)
}

fn check_gap(range: Vec<(i32, i32)>) -> Option<i32> {
    let mut r = range.clone();
    r.sort_by(|a, b| a.0.cmp(&b.0));
    // println!("{r:?}");
    let mut right = r[0].1;
    for &(a, b) in r[1..].iter() {
        if (a - 1) > right {
            return Some(a - 1);
        } else {
            right = right.max(b)
        }
    }
    None
}
