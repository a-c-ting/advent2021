mod day01;
mod day02;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let target = &args[1].parse::<i32>().unwrap();

    println!("Executing day {}\n", target);
    match target {
        1 => day01::execute(),
        2 => day02::execute(),
        _ => println!("That doesn't exist!"),
    }
}
