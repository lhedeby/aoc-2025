use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve() -> Result<(), Box<dyn Error>> {
    let f = File::open("./input/day02")?;
    let reader = BufReader::new(f);

    let line = reader.lines().next().unwrap()?;

    let mut p1 = 0;
    let mut p2 = 0;

    for r in line.split(",") {
        let (l, r) = r.split_once("-").expect("will always have '-'");
        let n1: usize = l.parse()?;
        let n2: usize = r.parse()?;
        for i in n1..n2 + 1 {
            let s = i.to_string();
            if let Some(_) = find_subs(&s) {
                p2 += i;
            }
            let (l, r) = s.split_at(s.len() / 2);

            if l == r {
                p1 += i;
            }
        }
    }

    print!("p1: {}, p2: {} ", p1, p2);

    Ok(())
}

fn find_subs(s: &str) -> Option<()> {
    let len = s.len();
    for j in 1..(len / 2) + 1 {
        let sub = &s[0..j];
        if len % sub.len() != 0 {
            continue;
        }
        let count = len / sub.len();

        let mut sub_count = 0;
        for c in 0..count {
            if *sub == s[c * sub.len()..c * sub.len() + sub.len()] {
                sub_count += 1;
            }
        }
        if sub_count == count {
            return Some(());
        }
    }
    None
}
