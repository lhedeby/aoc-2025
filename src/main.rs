mod days;

use std::{env, time::Instant};

const DAYS: [&str; 10] = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];

fn main() {
    let start = Instant::now();

    let args: Vec<String> = env::args().collect();

    let folder = if args.iter().any(|a| a == "ex") {
        "examples"
    } else {
        "input"
    };

    if let Some(day) = args.get(1) {
        run(folder, day)
    } else {
        println!("Running all...");
        for day in DAYS {
            print!("Day {day}: ");
            let curr = Instant::now();
            run(folder, day);
            println!(" (time: {:.3?})", curr.elapsed())
        }
    }

    println!("Total time: {:.3?}", start.elapsed())
}

fn run(folder: &str, day: &str) {
    let path = format!("./{}/day{:0>2}", folder, day);
    let path = path.as_str();
    match day {
        "1" => assert_eq!(days::day01::solve(path).unwrap(), (1055, 6386)),
        "2" => assert_eq!(days::day02::solve(path).unwrap(), (32976912643, 54446379122)),
        "3" => assert_eq!(days::day03::solve(path).unwrap(), (17430, 171975854269367)),
        "4" => assert_eq!(days::day04::solve(path).unwrap(), (1393, 8643)),
        "5" => assert_eq!(days::day05::solve(path).unwrap(), (726, 354226555270043)),
        "6" => assert_eq!(days::day06::solve(path).unwrap(), (6371789547734, 11419862653216)),
        "7" => assert_eq!(days::day07::solve(path).unwrap(), (1504, 5137133207830)),
        "8" => assert_eq!(days::day08::solve(path).unwrap(), (131580, 6844224)),
        "9" => assert_eq!(days::day09::solve(path).unwrap(), (4763932976, 1501292304)),
        "10" => assert_eq!(days::day10::solve(path).unwrap(), (477, 0)),
        _ => panic!("unexpected arg"),
    }
}
