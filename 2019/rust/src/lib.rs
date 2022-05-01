use std::io::{self, BufRead, Read};

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
                i = i + 4;
            }
            2 => {
                let x = codes[i + 1];
                let y = codes[i + 2];
                let t = codes[i + 3];
                codes[t] = codes[x] * codes[y];
                i = i + 4;
            }
            _ => {
                i = i + 1;
            }
        };
    }

    println!("{:?}", codes);
}
