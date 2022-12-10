use advent_of_code_2022::read_to_lines;
use std::collections::HashMap;

fn main() {
    let lines = read_to_lines("inputs/day07.txt");
    let mut path = vec![];
    let mut fs: HashMap<String, (Vec<String>, u32)> = HashMap::new();
    let mut subdir = vec![];
    let mut size = 0;
    let mut com = false;
    for line in lines {
        if &line[..1] == "$" {
            if com {
                fs.insert(path.join("/"), (subdir.clone(), size));
                size = 0;
                subdir = vec![];
                com = false;
            }
            match &line[2..4] {
                "cd" => {
                    if &line[5..] != ".." {
                        path.push(line[5..].to_owned());
                    } else {
                        path.pop();
                    }
                }
                "ls" => com = true,
                _ => {}
            }
        } else if &line[..3] == "dir" {
            path.push(line[4..].to_owned());
            subdir.push(path.join("/"));
            path.pop();
        } else {
            size += line
                .split_whitespace()
                .nth(0)
                .unwrap()
                .parse::<u32>()
                .unwrap();
        }
    }
    if com {
        fs.insert(path.join("/"), (subdir.clone(), size));

    }
    let mut fs_size = HashMap::new();
    for k in fs.keys() {
        get_size(k, &fs, &mut fs_size)
    }
    let sum: u32 = fs_size.values().filter(|&&s| s <= 100000).sum();
    dbg!(sum);

    let need = fs_size.get("/").unwrap() - 40000000;
    let smallest = fs_size.values().filter(|&&s| s >= need).min();
    dbg!(smallest);
}

fn get_size(k: &String, fs: &HashMap<String, (Vec<String>, u32)>, fs_size: &mut HashMap<String, u32>) {
    if !fs_size.contains_key(k) {
        let (i, j) = fs.get(k).unwrap();
        let mut sum = *j;
        for subk in i {
            get_size(subk, fs, fs_size);
            sum += fs_size.get(subk).unwrap();
        }
        fs_size.insert(k.to_owned(), sum);
    }
}
