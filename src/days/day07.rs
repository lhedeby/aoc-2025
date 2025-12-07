use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve(path: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut beam_test = vec![0; first_line.len()];

    let start = first_line.find('S').expect("should always exist a start");

    let mut p1 = 0;

    let mut beams = HashSet::new();
    beams.insert(start);

    beam_test[start] = 1;

    while let Some(Ok(line)) = lines.next() {
        let mut chars = line.chars().enumerate();

        while let Some((idx, _)) = chars.find(|&x| x.1 == '^') {
            if beam_test[idx] != 0 {
                beam_test[idx + 1] += beam_test[idx];
                beam_test[idx - 1] += beam_test[idx];
                beam_test[idx] = 0;
                p1 += 1;
            }
        }
    }
    let p2 = beam_test.iter().sum::<usize>();

    print!("p1: {}, p2: {} ", p1, p2);

    Ok((p1, p2))
}
