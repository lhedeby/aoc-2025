use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve(path: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    let mut p1 = 0;
    let mut lines = reader.lines().peekable();
    let mut map: Vec<u8> = vec![];
    let mut height = 0;

    while let Some(Ok(line)) = lines.next() {
        map.extend(line.chars().map(|c| if c == '@' { 1 } else { 0 }));
        height += 1;
    }
    let width = map.len() / height;

    for i in 0..height {
        for j in 0..width {
            if map[i * width + j] == 0 {
                continue;
            }
            change_surrounding(&mut map, width, height, i, j, true);
        }
    }

    p1 += map.iter().filter(|&&x| x > 0 && x < 5).count();

    let before = map
        .iter()
        .fold(0, |acc, &x| if x > 0 { acc + 1 } else { acc });
    while map.iter().filter(|&&x| x > 0 && x < 5).count() > 0 {
        remove(&mut map, width, height);
    }
    let after = map
        .iter()
        .fold(0, |acc, &x| if x > 0 { acc + 1 } else { acc });

    let p2 = before - after;

    print!("p1: {}, p2: {} ", p1, p2);

    Ok((p1, p2))
}

fn remove(map: &mut Vec<u8>, width: usize, height: usize) {
    let mut to_be_changed = vec![];
    for i in 0..height {
        for j in 0..width {
            let v = map[i * width + j];
            if v > 0 && v < 5 {
                map[i * width + j] = 0;
                to_be_changed.push((i, j));
            }
        }
    }
    for tbc in to_be_changed {
        change_surrounding(map, width, height, tbc.0, tbc.1, false)
    }
}

fn change_surrounding(
    map: &mut Vec<u8>,
    width: usize,
    height: usize,
    i: usize,
    j: usize,
    increase: bool,
) {
    let i_low = i.saturating_sub(1);
    let j_low = j.saturating_sub(1);
    let i_high = (i + 2).min(height);
    let j_high = (j + 2).min(width);
    for ii in i_low..i_high {
        for jj in j_low..j_high {
            if ii == i && jj == j {
                continue;
            }
            if map[ii * width + jj] != 0 {
                if increase {
                    map[ii * width + jj] += 1
                } else {
                    map[ii * width + jj] -= 1
                }
            }
        }
    }
}
