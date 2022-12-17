use std::collections::HashMap;

use advent_of_code_2022::read_to_lines;
use regex::Regex;

struct Problem {
    graph: HashMap<String, (usize, Vec<String>)>,
    memo: HashMap<(String, usize, Vec<bool>), usize>,
    ind: HashMap<String, usize>,
}

impl Problem {
    fn new(lines: &Vec<String>) -> Self {
        let re_valves = Regex::new(r"([A-Z]{2})").unwrap();
        let re_rate = Regex::new(r"rate=(\d+)").unwrap();
        let mut graph = HashMap::new();
        for line in lines {
            let mut v = re_valves.find_iter(&line);
            let from = v.next().unwrap().as_str().to_string();
            let to = v.map(|i| i.as_str().to_string()).collect::<Vec<String>>();
            let rate = re_rate.captures(&line).unwrap()[1]
                .parse::<usize>()
                .unwrap();
            graph.insert(from, (rate, to));
        }
        let mut ind = HashMap::new();
        for (i, k) in graph.keys().enumerate() {
            ind.insert(k.clone(), i);
        }
        Self {
            graph,
            memo: HashMap::new(),
            ind,
        }
    }

    fn init_status(&self) -> Vec<bool> {
        vec![false; self.graph.len()]
    }
    fn next_move(&mut self, valve: &str, remain_minute: usize, status: Vec<bool>) -> usize {
        if remain_minute == 0 {
            return 0;
        }
        if remain_minute <= 1 {
            return self.calc_pressure(&status);
        }
        if let Some(x) = self
            .memo
            .get(&(valve.to_string(), remain_minute, status.clone()))
        {
            return *x;
        }

        let (rate, next) = { self.graph.get(valve).unwrap().clone() };
        let mut temp = HashMap::new();
        let flow = self.calc_pressure(&status);
        for n in next.iter() {
            temp.insert(
                (n.clone(), remain_minute - 1),
                self.next_move(n, remain_minute - 1, status.clone()) + flow,
            );
        }
        if (rate != 0) && !status[*self.ind.get(valve).unwrap()] {
            // println!("{} {} {}", valve, remain_minute, "open");
            let mut sc = status.clone();
            sc[*self.ind.get(valve).unwrap()] = true;
            temp.insert(
                (valve.to_string(), remain_minute - 1),
                self.next_move(valve, remain_minute - 1, sc.clone()) + flow,
            );
        }
        let max = temp.iter().map(|(_, v)| *v).max().unwrap();
        self.memo
            .insert((valve.to_string(), remain_minute, status.clone()), max);
        // dbg!(valve, remain_minute, max);
        max
    }

    fn calc_pressure(&self, status: &Vec<bool>) -> usize {
        self.ind
            .iter()
            .map(|(k, v)| {
                if status[*v] {
                    self.graph.get(k).unwrap().0
                } else {
                    0
                }
            })
            .sum()
    }
}

struct Problem2 {
    graph: HashMap<u16, (usize, Vec<u16>)>,
    memo: HashMap<((u16, u16), usize, u64), usize>,
    ind: HashMap<u16, usize>,
    full: usize,
    complete: usize,
}

impl Problem2 {
    fn new(lines: &Vec<String>) -> Self {
        let re_valves = Regex::new(r"([A-Z]{2})").unwrap();
        let re_rate = Regex::new(r"rate=(\d+)").unwrap();
        let mut graph = HashMap::new();
        let mut full = 0;
        let mut complete = 0;
        for line in lines {
            let mut v = re_valves.find_iter(&line);
            let from = v2u(v.next().unwrap().as_str());
            let to = v.map(|i| v2u(i.as_str())).collect::<Vec<u16>>();
            let rate = re_rate.captures(&line).unwrap()[1]
                .parse::<usize>()
                .unwrap();
            graph.insert(from, (rate, to));
            if rate > 0 {
                full += 1;
                complete += rate;
            }
        }
        let mut ind = HashMap::new();
        for (i, k) in graph.keys().enumerate() {
            ind.insert(k.clone(), i);
        }
        Self {
            graph,
            memo: HashMap::new(),
            ind,
            full,
            complete,
        }
    }

    fn next_move(&mut self, valve: (u16, u16), remain_minute: usize, status: u64) -> usize {
        if remain_minute == 0 {
            return 0;
        }
        let flow = self.calc_pressure(&status);
        if remain_minute <= 1 {
            return flow;
        }
        if self.check_complete(&status) {
            return self.complete * remain_minute;
        }
        if let Some(x) = self.memo.get(&(
            (valve.0.to_owned(), valve.1.to_owned()),
            remain_minute,
            status.clone(),
        )) {
            return *x;
        }

        let (rate1, next1) = { self.graph.get(&valve.0).unwrap().clone() };
        let (rate2, next2) = { self.graph.get(&valve.1).unwrap().clone() };
        let v1 = valve.0;
        let v2 = valve.1;
        let mut temp = HashMap::new();
        for n1 in next1.iter() {
            for n2 in next2.iter() {
                temp.insert(
                    ((n1, n2), remain_minute - 1),
                    self.next_move((*n1, *n2), remain_minute - 1, status) + flow,
                );
            }
        }
        if (rate1 != 0) && (status >> *self.ind.get(&v1).unwrap() & 1 == 0) {
            let mut sc = status.clone();
            sc |= 1 << *self.ind.get(&v1).unwrap();
            for n2 in next2.iter() {
                temp.insert(
                    ((&v1, n2), remain_minute - 1),
                    self.next_move((v1, *n2), remain_minute - 1, sc) + flow,
                );
            }
        }
        if (rate2 != 0) && (status >> *self.ind.get(&v2).unwrap() & 1 == 0) {
            let mut sc = status.clone();
            sc |= 1 << *self.ind.get(&v2).unwrap();
            for n1 in next1.iter() {
                temp.insert(
                    ((n1, &v2), remain_minute - 1),
                    self.next_move((*n1, v2), remain_minute - 1, sc) + flow,
                );
            }
        }

        if (rate1 != 0)
            && (status >> *self.ind.get(&v1).unwrap() & 1 == 0)
            && (rate2 != 0)
            && (status >> *self.ind.get(&v2).unwrap() & 1 == 0)
        {
            let mut sc = status.clone();
            sc |= 1 << *self.ind.get(&v1).unwrap();
            sc |= 1 << *self.ind.get(&v2).unwrap();
            temp.insert(
                ((&v1, &v2), remain_minute - 1),
                self.next_move(
                    (valve.0.clone(), valve.1.clone()),
                    remain_minute - 1,
                    sc.clone(),
                ) + flow,
            );
        }
        // dbg!(&temp);
        let max = temp.iter().map(|(_, v)| *v).max().unwrap();
        self.memo.insert(
            (
                (valve.0.to_owned(), valve.1.to_owned()),
                remain_minute,
                status,
            ),
            max,
        );
        // dbg!(&valve, remain_minute, max);
        max
    }

    fn calc_pressure(&self, status: &u64) -> usize {
        self.ind
            .iter()
            .map(|(k, v)| {
                if (status >> *v & 1) == 1 {
                    self.graph.get(k).unwrap().0
                } else {
                    0
                }
            })
            .sum()
    }

    fn check_complete(&self, status: &u64) -> bool {
        status.count_ones() as usize == self.full
    }
}

fn v2u(valve: &str) -> u16 {
    let mut out = 0;
    out += valve.chars().nth(0).unwrap() as u16;
    out <<= 8;
    out += valve.chars().nth(1).unwrap() as u16;
    out
}

fn main() {
    let lines = read_to_lines("inputs/day16.txt");
    let mut p = Problem::new(&lines);
    let status = p.init_status();
    println!("{}", p.next_move("AA", 30, status));
    let mut p = Problem2::new(&lines);
    println!("{}", p.next_move((v2u("AA"), v2u("AA")), 26, 0));
}
