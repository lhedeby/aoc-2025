use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};


pub fn solve(path: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    let line = reader.lines().next().unwrap()?;

    let mut p1 = 0;
    let mut p2 = 0;

    for r in line.split(",") {
        let (l, r) = r.split_once("-").expect("will always have '-'");
        let n1: usize = l.parse()?;
        let n2: usize = r.parse()?;
        let mut temp: Vec<u8> = r.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();

        for _ in n1..n2 {
            if let Some(a) = find_repeats(&temp, true) {
                p1 += a;
            }
            if let Some(a) = find_repeats(&temp, false) {
                p2 += a;
            }
            dec_vec(&mut temp, 0);
        }

        if let Some(a) = find_repeats(&temp, true) {
            p1 += a;
        }
        if let Some(a) = find_repeats(&temp, false) {
            p2 += a;
        }
    }

    print!("p1: {}, p2: {} ", p1, p2);

    Ok((p1, p2))
}

fn dec_vec(v: &mut Vec<u8>, n: usize) {
    let p = v.len() - 1 - n;
    if v[p] == 0 {
        v[p] = 9;
        dec_vec(v, n + 1)
    } else {
        v[p] -= 1;
    }
}

fn find_repeats(v: &Vec<u8>, is_p1: bool) -> Option<usize> {
    let mut start = 0;
    let mut len = v.len();
    while v[start] == 0 {
        start += 1;
        len -= 1;
    }
    let max_len = (len) / 2;
    for l in 1..(max_len + 1) {
        if len % l != 0 {
            continue;
        }
        let mut has_repeat = true;

        for i in 0..l {
            let expected = v[start + i];
            let groups = len / l;
            for j in 1..groups {
                if v[start + i + j * l] != expected {
                    has_repeat = false;
                }
            }
        }
        if has_repeat {
            if !is_p1 || (len % 2 == 0 && l == len / 2) {
                let mut res: usize = 0;
                for r in start..(len + start) {
                    res += v[r] as usize;
                    res *= 10;
                }
                res /= 10;
                return Some(res);
            }
        }
    }
    None
}

