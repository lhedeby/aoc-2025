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
            if let Some(a) = find_repeats(&temp) {
                if a.1 {
                    p1 += a.0;
                }
                p2 += a.0;
            }
            dec_vec(&mut temp, 0);
        }

        if let Some(a) = find_repeats(&temp) {
            if a.1 {
                p1 += a.0;
            }
            p2 += a.0;
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

fn find_repeats(v: &Vec<u8>) -> Option<(usize, bool)> {
    let mut start = 0;
    let mut len = v.len();

    while v[start] == 0 {
        start += 1;
        len -= 1;
    }
    let max_len = len / 2;
    for l in (1..(max_len + 1)).rev() {
        if len % l != 0 {
            continue;
        }
        let mut has_repeat = true;

        'outer: for i in 0..l {
            let expected = v[start + i];
            let groups = len / l;
            for j in 1..groups {
                if v[start + i + j * l] != expected {
                    has_repeat = false;
                    break 'outer;
                }
            }
        }

        if has_repeat {
            let mut res: usize = 0;
            for r in start..(len + start) {
                res *= 10;
                res += v[r] as usize;
            }
            let is_p1 = if len % 2 == 0 && l == len / 2 {
                true
            } else {
                false
            };

            return Some((res, is_p1));
        }
    }
    None
}
