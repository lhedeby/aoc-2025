mod days;

use std::{env, time::Instant};

const DAYS: [&str; 5] = ["1", "2", "3", "4", "5"];

fn main() {
    let start = Instant::now();

    let args: Vec<String> = env::args().collect();

    if let Some(day) = args.get(1) {
        run(day)
    } else {
        println!("Running all...");
        for day in DAYS {
            print!("Day {day}: ");
            let curr = Instant::now();
            run(day);
            println!(" (time: {:.3?})", curr.elapsed())
        }
    }

    println!("Total time: {:.3?}", start.elapsed())
}

fn run(day: &str) {
    match day {
        "1" => days::day01::solve().unwrap(),
        "2" => days::day02::solve().unwrap(),
        "3" => days::day03::solve().unwrap(),
        "4" => days::day04::solve().unwrap(),
        "5" => days::day05::solve().unwrap(),
        _ => panic!("unexpected arg"),
    }
}
