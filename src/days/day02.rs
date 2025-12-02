use std::{error::Error, fs::File, io::{BufRead, BufReader}};

pub fn solve() -> Result<(), Box<dyn Error>> {

    let f = File::open("./input/day02")?;
    let reader = BufReader::new(f);

    let p1 = 0;
    let p2 = 0;

    print!("p1: {}, p2: {}", p1, p2);

    Ok(())
}


