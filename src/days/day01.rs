use std::{error::Error, fs::File, io::{BufRead, BufReader}};

pub fn solve() -> Result<(), Box<dyn Error>> {

    let f = File::open("./input/day01")?;
    let reader = BufReader::new(f);

    let mut n: i32 = 50;

    let mut p1 = 0;
    let mut p2 = 0;

    for line in reader.lines() {
        let line = line?;
        let (l, r) = line.split_at(1);

        let dir = match l {
            "L" => 1,
            "R" => -1,
            _ => panic!()
        };

        let count: i32 = r.parse()?;

        for _ in 0..count {
            n += dir;
            if n % 100 == 0 {
                p2 += 1;
            }
        }
        
        if n % 100 == 0 {
            p1 += 1;
        }
    }
    print!("p1: {}, p2: {}", p1, p2);

    Ok(())
}


