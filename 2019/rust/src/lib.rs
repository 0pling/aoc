use std::{
    collections::HashSet,
    io::{self, BufRead, Read},
};

fn get_input_lines() -> Vec<String> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    handle.lines().map(|l| l.unwrap()).collect()
}

fn get_input_string() -> String {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer).unwrap();
    buffer
}

pub fn solve_day01() {
    let s: i32 = get_input_lines()
        .into_iter()
        .map(|l| (l.parse::<i32>().unwrap() / 3) - 2)
        .sum();
    println!("{s}");
}

pub fn solve_day02() {
    let s = get_input_string();
    let mut codes: Vec<usize> = s.split(',').map(|x| x.trim().parse().unwrap()).collect();
    codes[1] = 12;
    codes[2] = 2;

    let mut i = 0;
    while i < codes.len() {
        let code = codes[i];
        match code {
            99 => {
                break;
            }
            1 => {
                let x = codes[i + 1];
                let y = codes[i + 2];
                let t = codes[i + 3];
                codes[t] = codes[x] + codes[y];
                i += 4;
            }
            2 => {
                let x = codes[i + 1];
                let y = codes[i + 2];
                let t = codes[i + 3];
                codes[t] = codes[x] * codes[y];
                i += 4;
            }
            _ => {
                i += 1;
            }
        };
    }

    println!("{:?}", codes);
}

fn manhanttan_dist(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn day03_trace_path(path: &str, pos: (i32, i32)) -> Vec<(i32, i32)> {
    let dir = &path[..1];
    let val = &path[1..].parse::<i32>().unwrap();
    match dir {
        "U" => (pos.1..=pos.1 + val).map(|y| (pos.0, y)).collect(),
        "R" => (pos.0..=pos.0 + val).map(|x| (x, pos.1)).collect(),
        "D" => (pos.1 - val..=pos.1).map(|y| (pos.0, y)).rev().collect(),
        "L" => (pos.0 - val..=pos.0).map(|x| (x, pos.1)).rev().collect(),
        _ => Vec::new(),
    }
}

pub fn solve_day03() {
    let lines = get_input_lines();
    let path1: Vec<&str> = lines[0].split(',').collect();
    let path2: Vec<&str> = lines[1].split(',').collect();

    let mut trace1 = HashSet::new();
    let mut pos = (0, 0);
    for p in path1.iter() {
        let curr_path = day03_trace_path(p, pos);
        trace1.extend(curr_path.iter());
        pos = *curr_path.last().unwrap();
    }

    let mut trace2 = HashSet::new();
    pos = (0, 0);
    for p in path2.iter() {
        let curr_path = day03_trace_path(p, pos);
        trace2.extend(curr_path.iter());
        pos = *curr_path.last().unwrap();
    }

    let res = trace1
        .intersection(&trace2)
        .filter(|&&p| p != (0, 0))
        .map(|&p| manhanttan_dist(p, (0, 0)))
        .min()
        .unwrap();
    println!("{}", res);
}

fn get_day04_inputs() -> Vec<i32> {
    get_input_string()
        .split('-')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

pub fn solve_day04() {
    let inputs = get_day04_inputs();
    let (s, e) = (inputs[0], inputs[1]);
    let count = (s + 1..e)
        .filter(|&x| {
            let xs = format!("{:}", x);
            xs.len() == 6
                && xs
                    .char_indices()
                    .any(|(i, c)| i + 1 < xs.len() && c.to_string() == xs[i + 1..i + 2])
                && xs.char_indices().all(|(i, c)| {
                    i + 1 == xs.len()
                        || c.to_string().parse::<i32>().unwrap()
                            <= xs[i + 1..i + 2].parse::<i32>().unwrap()
                })
        })
        .count();
    println!("{:}", count);
}
