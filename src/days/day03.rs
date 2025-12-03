use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve() -> Result<(), Box<dyn Error>> {
    let f = File::open("./input/day03")?;
    let reader = BufReader::new(f);

    let mut p1 = 0;
    let mut p2 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        p1 += find_voltage(&line, 2);
        p2 += find_voltage(&line, 12);
    }

    print!("p1: {}, p2: {} ", p1, p2);

    Ok(())
}

pub fn find_voltage(s: &str, count: usize) -> usize {
    let mut res = vec!['0'; count];
    let mut chars = s.chars();
    for i in 0..s.len() {
        let mut f = false;
        let c = chars.next().unwrap();
        for j in 0..res.len() {
            if f {
                res[j] = '0'
            } else if i + res.len() - j <= s.len() && res[j] < c {
                res[j] = c;
                f = true;
            }
        }
    }

    let mut multiplier = 1;
    let mut ans = 0;
    for asd in res.iter().rev() {
        ans += asd.to_digit(10).unwrap() as usize * multiplier;
        multiplier *= 10;
    }
    ans
}
