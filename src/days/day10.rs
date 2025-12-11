use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve(path: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let (mut p1, mut p2) = (0, 0);

    for line in lines {
        let line = line.unwrap();
        let machine = Machine::from_line(&line);
        p1 += machine.solve_p1();
    }

    print!("p1: {}, p2: {} ", p1, p2);

    Ok((p1, p2))
}

fn binstr_to_usize(s: &str) -> usize {
    let s = &s[1..s.len() - 1];
    let mut g2 = 0;
    s.split(",").for_each(|n| {
        g2 += n.parse::<usize>().unwrap();
        g2 = g2 << 1;
    });

    g2 >> 1
}

// using binary because why not
#[derive(Debug)]
struct Machine {
    light_goal: usize,
    jolt_goal: usize,
    instructions: Vec<usize>,
}

impl Machine {
    fn solve_p2(&self) -> usize {
        todo!();
    }
    fn solve_p1(&self) -> usize {
        let mut visited_states: HashSet<usize> = HashSet::new();
        visited_states.insert(0);
        let mut states = vec![0 as usize];
        let mut presses = 0;

        'outside: loop {
            presses += 1;
            let mut new_states = vec![];
            for s in &states {
                for i in &self.instructions {
                    let new_state = s ^ i;
                    if new_state == self.light_goal {
                        break 'outside;
                    }
                    if !visited_states.contains(&new_state) {
                        visited_states.insert(new_state);
                        new_states.push(new_state);
                    }
                }
            }
            states = new_states;
        }
        presses
    }
    fn from_line(line: &str) -> Self {
        let mut split = line.split_whitespace();
        let light_goal: String = split
            .next()
            .unwrap()
            .chars()
            .map(|c| match c {
                '.' => '0',
                '#' => '1',
                _ => ' ',
            })
            .filter(|&c| c != ' ')
            .collect();
        let len = light_goal.len();
        let light_goal = usize::from_str_radix(&light_goal, 2).unwrap();

        let mut instructions = vec![];
        let mut jolt_goal = 0;

        while let Some(n) = split.next() {
            if n.chars().nth(0).unwrap() != '{' {
                let ns = n
                    .replace("(", "")
                    .replace(")", "")
                    .split(",")
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<usize>>();
                instructions.push(ns);
            } else {
                jolt_goal = binstr_to_usize(n);
            }
        }

        let instructions = instructions
            .iter()
            .map(|ins| {
                let mut res = 0;
                for i in 0..len {
                    if ins.contains(&i) {
                        res += 1;
                    }
                    res = res << 1;
                }
                res >> 1
            })
            .collect();

        Self {
            light_goal,
            jolt_goal,
            instructions,
        }
    }
}

// 0000
// 0101
