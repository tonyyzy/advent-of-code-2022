use advent_of_code_2022::read_to_lines;

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<i64>,
    ops: (char, i64),
    test: i64,
    t: usize,
    f: usize,
}

impl Monkey {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            ops: (' ', 0),
            test: 0,
            t: 0,
            f: 0,
        }
    }
}

fn main() {
    let lines = read_to_lines("inputs/day11.txt");
    let mut monkeys = vec![];
    let mut monkey = Monkey::new();
    for line in lines {
        if line.len() == 0 {
            monkeys.push(monkey.clone())
        } else {
            match &line[..8] {
                "  Starti" => {
                    monkey.items = line
                        .split_at(18)
                        .1
                        .split(", ")
                        .map(|s| s.parse().unwrap())
                        .collect();
                }
                "  Operat" => {
                    let (op, val) = line.split_at(23).1.split_at(2);
                    if val == "old" {
                        // only every old * old so we can put it as power 2
                        monkey.ops = ('^', 2);
                    } else {
                        monkey.ops.0 = op.chars().nth(0).unwrap();
                        monkey.ops.1 = val.parse().unwrap();
                    }
                }
                "  Test: " => {
                    monkey.test = line.split(" ").last().unwrap().parse().unwrap();
                }
                "    If t" => {
                    monkey.t = line.split(" ").last().unwrap().parse().unwrap();
                }
                "    If f" => {
                    monkey.f = line.split(" ").last().unwrap().parse().unwrap();
                }
                _ => {}
            }
        }
    }
    monkeys.push(monkey);
    let mut counter = vec![0; monkeys.len()];
    for _ in 0..20 {
        for mi in 0..monkeys.len() {
            let items: Vec<_> = monkeys[mi].items.drain(..).collect();
            counter[mi] += items.len();
            for v in items {
                let test = monkeys[mi].test;
                let wl = match monkeys[mi].ops {
                    ('+', k) => (v + k),
                    ('*', k) => (v * k),
                    ('^', k) => v.pow(k as u32),
                    _ => unreachable!(),
                } / 3;
                let t = monkeys[mi].t;
                let f = monkeys[mi].f;
                if wl % test == 0 {
                    monkeys[t].items.push(wl);
                } else {
                    monkeys[f].items.push(wl);
                }
            }
        }
    }
    counter.sort();
    println!(
        "{}",
        counter[counter.len() - 1] * counter[counter.len() - 2]
    );
    let div = monkeys.iter().map(|m| m.test).fold(1, |acc, x| acc * x);
    let mut counter = vec![0; monkeys.len()];
    for _ in 0..10000 {
        for mi in 0..monkeys.len() {
            let items: Vec<_> = monkeys[mi].items.drain(..).collect();
            counter[mi] += items.len();
            for v in items {
                let test = monkeys[mi].test;
                let wl = match monkeys[mi].ops {
                    ('+', k) => (v + k),
                    ('*', k) => (v * k),
                    ('^', k) => v.pow(k as u32),
                    _ => unreachable!(),
                };
                let t = monkeys[mi].t;
                let f = monkeys[mi].f;
                if wl % test == 0 {
                    monkeys[t].items.push(wl % div);
                } else {
                    monkeys[f].items.push(wl % div);
                }
            }
        }
    }
    counter.sort();
    println!(
        "{}",
        counter[counter.len() - 1] * counter[counter.len() - 2]
    );
}
