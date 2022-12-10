use advent_of_code_2022::read_to_lines;
use std::collections::HashSet;

fn main() {
    let lines = read_to_lines("inputs/day09.txt");
    dbg!(rope_move::<2>(&lines));
    dbg!(rope_move::<10>(&lines));
}

fn new_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let x = head.0 - tail.0;
    let y = head.1 - tail.1;
    if x * x + y * y > 2 {
        if y == 0 {
            (tail.0 + x / 2, tail.1)
        } else if x == 0 {
            (tail.0, tail.1 + y / 2)
        } else {
            (
                tail.0 + (x > 0) as i32 * 2 - 1,
                tail.1 + (y > 0) as i32 * 2 - 1,
            )
        }
    } else {
        tail
    }
}

fn rope_move<const N: usize>(lines: &Vec<String>) -> usize {
    let mut visited = HashSet::new();
    let mut knots = vec![(0, 0); N];
    visited.insert(knots[knots.len() - 1].clone());
    for line in lines {
        let (dir, step) = line.split_once(" ").unwrap();
        let step = step.parse::<u8>().unwrap();
        let op = match dir {
            "L" => (-1, 0),
            "R" => (1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => unreachable!(),
        };
        for _ in 0..step {
            knots[0] = (knots[0].0 + op.0, knots[0].1 + op.1);
            for i in 1..knots.len() {
                knots[i] = new_tail(knots[i - 1], knots[i]);
            }
            visited.insert(knots[knots.len() - 1].clone());
        }
    }
    visited.len()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(new_tail((0, 0), (0, 0)), (0, 0));
        assert_eq!(new_tail((0, 1), (0, 0)), (0, 0));
        assert_eq!(new_tail((1, 0), (0, 0)), (0, 0));
        assert_eq!(new_tail((0, -1), (0, 0)), (0, 0));
        assert_eq!(new_tail((-1, 0), (0, 0)), (0, 0));
        assert_eq!(new_tail((1, 1), (0, 0)), (0, 0));
        assert_eq!(new_tail((1, -1), (0, 0)), (0, 0));
        assert_eq!(new_tail((-1, 1), (0, 0)), (0, 0));
        assert_eq!(new_tail((-1, -1), (0, 0)), (0, 0));
        assert_eq!(new_tail((1, 2), (0, 0)), (1, 1));
        assert_eq!(new_tail((1, -2), (0, 0)), (1, -1));
        assert_eq!(new_tail((-1, 2), (0, 0)), (-1, 1));
        assert_eq!(new_tail((-1, -2), (0, 0)), (-1, -1));
    }

    #[test]
    fn t2() {
        assert_eq!(new_tail((3, 0), (1, 0)), (2, 0));
    }
}
