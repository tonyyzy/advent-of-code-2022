use std::fs::read_to_string;

fn main() {
    let mut map: Vec<Vec<char>> = read_to_string("inputs/day12.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let row = map.len();
    let col = map[0].len();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for r in 0..row {
        for c in 0..col {
            if map[r][c] == 'S' {
                start = (r, c)
            }
            if map[r][c] == 'E' {
                end = (r, c)
            }
        }
    }
    map[start.0][start.1] = 'a';
    map[end.0][end.1] = 'z';
    let mapu: Vec<Vec<u8>> = map
        .iter()
        .map(|l| l.iter().map(|&c| c as u8).collect())
        .collect();
    let mut steps = vec![vec![u32::MAX; col]; row];
    steps[start.0 as usize][start.1 as usize] = 0;
    let mut updated = vec![start];
    let mut new = vec![];
    while updated.len() > 0 {
        for p in updated.drain(..) {
            let val = steps[p.0][p.1];
            for n in neighbour(p.0, p.1, row, col) {
                if (mapu[n.0][n.1] <= (mapu[p.0][p.1] + 1)) & (val + 1 < steps[n.0][n.1]) {
                    steps[n.0][n.1] = val + 1;
                    new.push(n);
                }
            }
        }
        updated.append(&mut new);
    }
    println!("{}", steps[end.0][end.1]);

    let mut steps = vec![vec![u32::MAX; col]; row];
    let mut updated = vec![];
    for r in 0..row {
        for c in 0..col {
            if map[r][c] == 'a' {
                steps[r][c] = 0;
                updated.push((r, c));
            }
        }
    }
    let mut new = vec![];
    while updated.len() > 0 {
        for p in updated.drain(..) {
            let val = steps[p.0][p.1];
            for n in neighbour(p.0, p.1, row, col) {
                if (mapu[n.0][n.1] <= (mapu[p.0][p.1] + 1)) & (val + 1 < steps[n.0][n.1]) {
                    steps[n.0][n.1] = val + 1;
                    new.push(n);
                }
            }
        }
        updated.append(&mut new);
    }
    println!("{}", steps[end.0][end.1]);
}

fn neighbour(x: usize, y: usize, row: usize, col: usize) -> Vec<(usize, usize)> {
    let x = x as i8;
    let y = y as i8;
    let row = row as i8;
    let col = col as i8;
    [(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)]
        .into_iter()
        .filter(|(x, y)| (*x >= 0) & (*x < row) & (*y >= 0) & (*y < col))
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}
