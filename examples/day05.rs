use advent_of_code_2022::read_to_lines;

fn main() {
    let lines = read_to_lines("inputs/day05.txt");
    let n_stack = lines[0].len() / 4 + 1;
    let mut stacks: Vec<Vec<char>> = vec![vec![]; n_stack];
    for line in lines {
        match line.chars().nth(0) {
            Some('[') => {
                for (ind, c) in line.char_indices() {
                    if c != ' ' && ind % 4 == 1 {
                        stacks[ind / 4].push(c)
                    }
                }
            }
            Some('m') => {
                let moves = line.split(' ').collect::<Vec<&str>>();
                let a = moves[1].parse::<usize>().unwrap();
                let b = moves[3].parse::<usize>().unwrap() - 1;
                let c = moves[5].parse::<usize>().unwrap() - 1;
                for _ in 0..a {
                    let l = stacks[b].pop().unwrap();
                    stacks[c].push(l);
                }
            }
            Some(' ') => {
                if line.len() > 0 {
                    for s in stacks.iter_mut() {
                        s.reverse()
                    }
                }
            }
            _ => {}
        }
    }

    for s in stacks {
        if s.len() != 0 {
            print!("{}", s[s.len() - 1]);
        }
    }
    println!();

    let lines = read_to_lines("inputs/day05.txt");
    let n_stack = lines[0].len() / 4 + 1;
    let mut stacks: Vec<Vec<char>> = vec![vec![]; n_stack];
    for line in lines {
        match line.chars().nth(0) {
            Some('[') => {
                for (ind, c) in line.char_indices() {
                    if c != ' ' && ind % 4 == 1 {
                        stacks[ind / 4].push(c)
                    }
                }
            }
            Some('m') => {
                let moves = line.split(' ').collect::<Vec<&str>>();
                let a = moves[1].parse().unwrap();
                let b = moves[3].parse::<usize>().unwrap() - 1;
                let c = moves[5].parse::<usize>().unwrap() - 1;
                let k = stacks[b][stacks[b].len() - a..].to_owned();
                stacks[c].extend_from_slice(&k);
                for _ in 0..a {
                    stacks[b].pop();
                }
            }
            Some(' ') => {
                if line.len() > 0 {
                    for s in stacks.iter_mut() {
                        s.reverse()
                    }
                }
            }
            _ => {}
        }
    }

    for s in stacks {
        if s.len() != 0 {
            print!("{}", s[s.len() - 1]);
        }
    }
}
