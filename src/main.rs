mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod shared_utils;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let target = &args[1].parse::<i32>().unwrap();

    println!("Executing day {}", target);
    match target {
        1 => day01::execute(),
        2 => day02::execute(),
        3 => day03::execute(),
        4 => day04::execute(),
        5 => day05::execute(),
        6 => day06::execute(),
        _ => println!("That doesn't exist!"),
    }
}
