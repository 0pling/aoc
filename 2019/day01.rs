use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lines: Vec<String> = handle.lines().map(|l| l.unwrap()).collect();

    let s: i32 = lines.into_iter()
        .map(|x| (x.parse::<i32>().unwrap() / 3) - 2)
        .sum();
    println!("{s}");
}
