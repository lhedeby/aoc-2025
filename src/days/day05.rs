use std::{
    error::Error, fs::File, io::{BufRead, BufReader}
};

pub fn solve() -> Result<(), Box<dyn Error>> {
    let f = File::open("./input/day05")?;
    let reader = BufReader::new(f);
    let mut lines = reader.lines();

    let mut ranges = vec![];

    let mut p1 = 0;

    loop {
        let line = lines.next().unwrap().unwrap();
        if line.is_empty() {
            break;
        }
        match line.split_once("-") {
            Some((low, high)) => {
                ranges.push(Range {
                    low: low.parse().unwrap(),
                    high: high.parse().unwrap(),
                });
            }
            _ => panic!("malformed range."),
        }
    }

    while let Some(Ok(line)) = lines.next() {
        for r in &ranges {
            if r.in_bounds(line.parse().unwrap()) {
                p1 += 1;
                break;
            }
        }
    }

    let mut i = 0;
    while i < ranges.len() - 1 {
        let mut j = i + 1;
        while j < ranges.len() {
            if ranges[i].overlaps(&ranges[j]) {
                ranges[i].low = ranges[i].low.min(ranges[j].low);
                ranges[i].high = ranges[i].high.max(ranges[j].high);
                ranges.remove(j);
                j = i + 1;
            } else {
                j += 1
            }
        }
        i += 1;
    }
    let p2: usize = ranges.iter().map(|r| r.count()).sum();


    print!("p1: {}, p2: {} ", p1, p2);

    Ok(())
}

#[derive(Debug)]
struct Range {
    low: usize,
    high: usize,
}

impl Range {
    fn in_bounds(&self, n: usize) -> bool {
        n >= self.low && n <= self.high
    }
    fn count(&self) -> usize {
        self.high - self.low + 1
    }
    fn overlaps(&self, other: &Range) -> bool {
        !(self.high < other.low || self.low > other.high)
    }
}


