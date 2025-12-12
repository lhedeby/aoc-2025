use std::{
    error::Error,
    fs::{self},
};

pub fn solve(path: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let binding = fs::read_to_string(path)?;
    let mut input = binding.split("\n\n").peekable();

    let mut shapes: Vec<Shape> = vec![];
    let mut regions: Vec<Region> = vec![];

    while let Some(i) = input.next() {
        match input.peek() {
            Some(_) => shapes.push(
                i.replace("\n", "")
                    .chars()
                    .skip(2)
                    .map(|c| match c {
                        '.' => false,
                        '#' => true,
                        c => panic!("c: {}", c),
                    })
                    .collect(),
            ),
            None => {
                let mut split = i.split("\n");
                while let Some(line) = split.next() {
                    if line.is_empty() {
                        continue;
                    }
                    let mut split = line.split_whitespace();
                    let (width, height) = split
                        .next()
                        .unwrap()
                        .split_once(":")
                        .unwrap()
                        .0
                        .split_once("x")
                        .unwrap();
                    let quantities = split.map(|x| x.parse().unwrap()).collect();
                    regions.push(Region {
                        width: width.parse().unwrap(),
                        height: height.parse().unwrap(),
                        quantities,
                    })
                }
            }
        }
    }

    let (mut p1, p2) = (0, 0);
    for region in regions {
        let area = region.area();
        let mut bits = 0;
        for (i, q) in region.quantities.iter().enumerate() {
            let bit = &shapes[i].iter().filter(|&&x| x).count();
            bits += bit * q;
        }
        if bits <= area {
            p1 += 1;
        }
    }

    print!("p1: {}, p2: {} ", p1, p2);

    Ok((p1, p2))
}

type Shape = Vec<bool>;

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    quantities: Vec<usize>,
}

impl Region {
    fn area(&self) -> usize {
        self.width * self.height
    }
}
