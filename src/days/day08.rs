use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve(path: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let jboxes: Vec<(i64, i64, i64)> = lines
        .map(|line| {
            let line = line.unwrap();
            let mut split = line.split(",");
            (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut len_heap = BinaryHeap::with_capacity(jboxes.len() * (jboxes.len() - 1) / 2);

    for i in 0..jboxes.len() - 1 {
        let (x1, y1, z1) = unsafe { jboxes.get_unchecked(i) };
        for j in i + 1..jboxes.len() {
            let (x2, y2, z2) = unsafe { jboxes.get_unchecked(j) };
            let dx = x1 - x2;
            let dy = y1 - y2;
            let dz = z1 - z2;
            let dist = dx * dx + dy * dy + dz * dz;
            len_heap.push(Reverse((dist, i, j)));
        }
    }

    let mut circuits: Vec<HashSet<usize>> = Vec::with_capacity(128);
    let mut p1 = 0;
    let mut i = 0;

    let p2_connection = loop {
        if i == 1000 {
            circuits.sort_by_key(|a| a.len());
            circuits.reverse();
            p1 = 1;
            for c in &circuits[0..3] {
                p1 *= c.len();
            }
        }

        let connection = len_heap.pop().unwrap().0;

        let mut aa = None;
        let mut bb = None;
        for circuit in circuits.iter().enumerate() {
            if circuit.1.contains(&connection.1) {
                aa = Some(circuit.0)
            }
            if circuit.1.contains(&connection.2) {
                bb = Some(circuit.0)
            }
        }
        match (aa, bb) {
            (None, None) => {
                let mut new = HashSet::new();
                new.insert(connection.1);
                new.insert(connection.2);
                circuits.push(new)
            }
            (None, Some(b)) => {
                circuits[b].insert(connection.1);
            }
            (Some(a), None) => {
                circuits[a].insert(connection.2);
            }
            (Some(mut a), Some(b)) => {
                if a != b {
                    let temp = circuits.remove(b);
                    if b < a {
                        a -= 1;
                    }
                    circuits[a].extend(temp);
                }
            }
        }
        if circuits.get(0).is_some_and(|x| x.len() == jboxes.len()) {
            break connection;
        }

        i += 1;
    };

    let p2 = jboxes[p2_connection.1].0 * jboxes[p2_connection.2].0;

    print!("p1: {}, p2: {} ", p1, p2);

    Ok((p1, p2 as usize))
}
