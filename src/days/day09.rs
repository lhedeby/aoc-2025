use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve(path: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let red_tiles: Vec<Tile> = lines
        .map(|line| {
            let line = line.unwrap();
            let split = line.split_once(",").unwrap();
            Tile {
                x: split.0.parse().unwrap(),
                y: split.1.parse().unwrap(),
            }
        })
        .collect();

    let mut p1 = 0;
    let mut p2 = 0;

    for i in 0..red_tiles.len() - 1 {
        let t1 = red_tiles[i];
        for j in i + 1..red_tiles.len() {
            let t2 = red_tiles[j];
            let area = t1.area(&t2);

            if test(&t1, &t2, i, j, &red_tiles) && area > p2 {
                p2 = area
            }
            if area > p1 {
                p1 = area;
            }
        }
    }

    print!("p1: {}, p2: {} ", p1, p2);

    Ok((p1, p2 as usize))
}

fn test(r1: &Tile, t2: &Tile, idx1: usize, idx2: usize, tiles: &Vec<Tile>) -> bool {
    let rect0 = rect(r1, t2);
    let len = tiles.len();
    for i in 0..len {
        let mut n = i + 1;
        if n >= len {
            n = 0;
        }
        if i == idx1 || i == idx2 || n == idx1 || n == idx2 {
            continue;
        }
        let rect1 = rect(&tiles[i], &tiles[n]);
        if rect0.0 < rect1.1 && rect0.1 > rect1.0 && rect0.2 < rect1.3 && rect0.3 > rect1.2 {
            return false;
        }
    }

    return true;
}

fn rect(t1: &Tile, t2: &Tile) -> (usize, usize, usize, usize) {
    let mut left = t1.x;
    let mut right = t2.x;
    let mut top = t1.y;
    let mut bottom = t2.y;

    if right < left {
        left = t2.x;
        right = t1.x
    }
    if bottom < top {
        top = t2.y;
        bottom = t1.y
    }
    (left, right, top, bottom)
}

#[derive(Clone, Copy, Debug)]
struct Tile {
    x: usize,
    y: usize,
}

impl Tile {
    fn area(&self, other: &Tile) -> usize {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}
