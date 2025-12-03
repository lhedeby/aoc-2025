mod days;

use std::{env, time::Instant};

const DAYS: [&str; 3] = ["1", "2", "3"];

fn main() {
    let start = Instant::now();

    let args: Vec<String> = env::args().collect();

    if let Some(day) = args.get(1) {
        run(day)
    } else {
        println!("Running all...");
        for day in DAYS {
            println!("running day {day}");
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
        _ => panic!("unexpected arg"),
    }
}
