use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1][..];
    match day {
        "day01" => rust::solve_day01(),
        "day02" => rust::solve_day02(),
        "day03" => rust::solve_day03(),
        _ => unimplemented!(),
    }
}
