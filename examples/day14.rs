use std::collections::HashSet;

use advent_of_code_2022::read_to_lines;

fn main() {
    let lines = read_to_lines("inputs/day14.txt");
    let mut grid: HashSet<(i32, i32)> = HashSet::new();
    for line in lines.iter() {
        let points = line.split(" -> ").collect::<Vec<&str>>();
        for (start, end) in points[..points.len() - 1].iter().zip(points[1..].iter()) {
            let (sx, sy) = parse_point(start);
            let (ex, ey) = parse_point(end);
            if sx == ex {
                let yr = if sy < ey { sy..ey + 1 } else { ey..sy + 1 };
                for y in yr {
                    grid.insert((sx, y));
                }
            } else {
                let xr = if sx < ex { sx..ex + 1 } else { ex..sx + 1 };
                for x in xr {
                    grid.insert((x, sy));
                }
            }
        }
    }
    let mut grid1 = grid.clone();
    let bottom = *grid.iter().map(|(_, b)| b).max().unwrap();

    let mut pos = (500, 0);
    let len = grid.len();
    'l: loop {
        while let Some(p) = next_step(pos, &grid) {
            pos = p;
            if pos.1 > bottom {
                break 'l;
            }
        }
        grid.insert(pos);
        pos = (500, 0);
    }
    println!("{}", grid.len() - len);

    let bottom = *grid1.iter().map(|(_, b)| b).max().unwrap() + 1;

    let mut pos = (500, 0);
    loop {
        while let Some(p) = next_step(pos, &grid1) {
            pos = p;
            if pos.1 == bottom {
                break;
            }
        }
        grid1.insert(pos);
        if pos == (500, 0) {
            break;
        }
        pos = (500, 0);
    }
    println!("{}", grid1.len() - len);
}

fn parse_point(s: &str) -> (i32, i32) {
    let (x, y) = s.split_once(",").unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

fn next_step(p: (i32, i32), grid: &HashSet<(i32, i32)>) -> Option<(i32, i32)> {
    if !grid.contains(&(p.0, p.1 + 1)) {
        Some((p.0, p.1 + 1))
    } else if !grid.contains(&(p.0 - 1, p.1 + 1)) {
        Some((p.0 - 1, p.1 + 1))
    } else if !grid.contains(&(p.0 + 1, p.1 + 1)) {
        Some((p.0 + 1, p.1 + 1))
    } else {
        None
    }
}
