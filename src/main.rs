mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod shared_utils;

use std::env;
use std::time::Instant;

fn main() {
    let timer = Instant::now();

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
        7 => day07::execute(),
        8 => day08::execute(),
        9 => day09::execute(),
        10 => day10::execute(),
        11 => day11::execute(),
        12 => day12::execute(),
        13 => day13::execute(),
        14 => day14::execute(),
        15 => day15::execute(),
        _ => println!("That doesn't exist!"),
    }

    let elapsed = timer.elapsed();
    println!("\nexecution time: {:.2?}", elapsed);
}
