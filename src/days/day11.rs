use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve(path: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        let key = split.next().unwrap().replace(":", "");
        let values = split.map(|s| s.to_string()).collect();
        map.insert(key, values);
    }

    let p1 = traverse("you", "out", &map, &mut HashMap::new());

    let mut p2 = traverse("svr", "fft", &map, &mut HashMap::new());
    p2 *= traverse("fft", "dac", &map, &mut HashMap::new());
    p2 *= traverse("dac", "out", &map, &mut HashMap::new());

    print!("p1: {}, p2: {} ", p1, p2);

    Ok((p1, p2))
}

fn traverse(
    pos: &str,
    end: &str,
    map: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if memo.contains_key(pos) {
        return memo[pos];
    }
    if pos == end {
        return 1;
    }
    let mut res = 0;
    if let Some(list) = map.get(pos) {
        for l in list {
            res += traverse(l, end, map, memo)
        }
    }
    memo.insert(pos.to_string(), res);
    res
}
