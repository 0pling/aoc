use std::io::{self, BufRead};

fn get_input_lines() -> Vec<String> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    handle.lines().map(|l| l.unwrap()).collect()
}

pub fn solve_day01() {
    let s: i32 = get_input_lines()
        .into_iter()
        .map(|l| (l.parse::<i32>().unwrap() / 3) - 2)
        .sum();
    println!("{s}");
}
