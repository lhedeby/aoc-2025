use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve(path: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let M: Vec<Vec<char>> = lines.map(|x| x.unwrap().chars().collect()).collect();

    let mut p1 = 0;
    let mut p2 = 0;

    let mut starts = M
        .last()
        .unwrap()
        .iter()
        .enumerate()
        .filter(|s| s.1 != &' ')
        .peekable();

    while let Some((start, operator)) = starts.next() {
        let end = if let Some(e) = starts.peek() {
            e.0 - 1
        } else {
            M[0].len()
        };

        // p1
        {
            let mut sum: usize = match operator {
                '*' => 1,
                '+' => 0,
                _ => panic!("unexpected operator"),
            };
            for i in 0..M.len() - 1 {
                let mut n = 0;
                for aa in start..end {
                    n += match M[i][aa].to_digit(10) {
                        Some(d) => {
                            n *= 10;
                            d
                        }
                        None => 0,
                    };
                }

                match operator {
                    '*' => sum *= n as usize,
                    '+' => sum += n as usize,
                    _ => panic!("unexpected operator"),
                };
            }
            p1 += sum;
        }

        // p2
        {
            let mut sum: usize = match operator {
                '*' => 1,
                '+' => 0,
                _ => panic!("unexpected operator"),
            };
            for aa in start..end {
                let mut n = 0;

                for i in 0..M.len() - 1 {
                    n += match M[i][aa].to_digit(10) {
                        Some(d) => {
                            n *= 10;
                            d
                        }
                        None => 0,
                    };
                }

                match operator {
                    '*' => sum *= n as usize,
                    '+' => sum += n as usize,
                    _ => panic!("unexpected operator"),
                };
            }
            p2 += sum;
        }
    }

    print!("p1: {}, p2: {} ", p1, p2);

    Ok((p1, p2))
}
