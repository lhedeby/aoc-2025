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

    let mut len_heap = BinaryHeap::new();

    for i in 0..jboxes.len() - 1 {
        for j in i + 1..jboxes.len() {
            let dist = (jboxes[i].0 - jboxes[j].0) * (jboxes[i].0 - jboxes[j].0)
                + (jboxes[i].1 - jboxes[j].1) * (jboxes[i].1 - jboxes[j].1)
                + (jboxes[i].2 - jboxes[j].2) * (jboxes[i].2 - jboxes[j].2);
            len_heap.push(Reverse((dist, i, j)))
        }
    }

    let mut circuits: Vec<HashSet<usize>> = Vec::new();

    let mut p1 = 0;

    let mut i = 0;
    let mut connection_out = None;
    // break out
    while !circuits.get(0).is_some_and(|x| x.len() == jboxes.len()) {
        if i == 1000 {
            circuits.sort_by_key(|a| a.len());
            circuits.reverse();
            p1 = 1;
            for c in &circuits[0..3] {
                p1 *= c.len();
            }
        }

        let temp = len_heap.pop().unwrap().0;
        let connection = (temp.1, temp.2, temp.0);
        connection_out = Some(connection);

        let mut aa = None;
        let mut bb = None;
        for circuit in circuits.iter().enumerate() {
            if circuit.1.contains(&connection.0) {
                aa = Some(circuit.0)
            }
            if circuit.1.contains(&connection.1) {
                bb = Some(circuit.0)
            }
        }
        match (aa, bb) {
            (None, None) => {
                let mut new = HashSet::new();
                new.insert(connection.0);
                new.insert(connection.1);
                circuits.push(new)
            }
            (None, Some(b)) => {
                circuits[b].insert(connection.0);
            }
            (Some(a), None) => {
                circuits[a].insert(connection.1);
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
        i += 1;
    }
    let p2 = jboxes[connection_out.unwrap().0].0 * jboxes[connection_out.unwrap().1].0;

    print!("p1: {}, p2: {} ", p1, p2);

    Ok((p1, p2 as usize))
}
