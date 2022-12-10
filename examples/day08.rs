use advent_of_code_2022::read_to_lines;

fn main() {
    let lines = read_to_lines("inputs/day08.txt");
    let trees = lines
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    let rows = lines.len();
    let cols = lines[0].len();
    let mut marks = init_markers(rows, cols);

    for i in 1..rows - 1 {
        let mut max = 0;
        for j in 0..cols - 1 {
            if trees[i][j] > max {
                marks[i][j] = 1;
                max = trees[i][j]
            }
        }
    }
    for i in 1..rows - 1 {
        let mut max = 0;
        for j in (1..cols).rev() {
            if trees[i][j] > max {
                marks[i][j] = 1;
                max = trees[i][j]
            }
        }
    }
    for i in 1..cols - 1 {
        let mut max = 0;
        for j in 0..rows - 1 {
            if trees[j][i] > max {
                marks[j][i] = 1;
                max = trees[j][i]
            }
        }
    }
    for i in 1..cols - 1 {
        let mut max = 0;
        for j in (1..rows).rev() {
            if trees[j][i] > max {
                marks[j][i] = 1;
                max = trees[j][i]
            }
        }
    }
    let sum: u32 = marks.iter().flatten().sum();
    dbg!(sum);

    let mut max = 0;
    for i in 0..rows {
        for j in 0..cols {
            let s = scenic_score(i, j, &trees);
            if s > max {
                max = s
            }
        }
    }
    dbg!(max);
}

fn init_markers(rows: usize, cols: usize) -> Vec<Vec<u32>> {
    let mut marks = vec![];
    marks.push(vec![1; cols]);
    let mut r = vec![0; cols];
    r[0] = 1;
    r[cols - 1] = 1;
    for _ in 1..rows - 1 {
        marks.push(r.clone());
    }
    marks.push(vec![1; cols]);
    marks
}

fn scenic_score(row: usize, col: usize, tree: &Vec<Vec<u8>>) -> u32 {
    let rows = tree.len();
    let cols = tree[0].len();
    if (row == 0) || (col == 0) || (row == rows - 1) || (col == cols - 1) {
        return 0;
    }
    let mut l = 0;
    let mut r = 0;
    let mut u = 0;
    let mut d = 0;
    // going left
    for i in (0..col).rev() {
        l += 1;
        if tree[row][i] >= tree[row][col] {
            break;
        }
    }
    for i in (col + 1)..cols {
        r += 1;
        if tree[row][i] >= tree[row][col] {
            break;
        }
    }
    for i in (0..row).rev() {
        u += 1;
        if tree[i][col] >= tree[row][col] {
            break;
        }
    }
    for i in (row + 1)..rows {
        d += 1;
        if tree[i][col] >= tree[row][col] {
            break;
        }
    }

    l * r * u * d
}
