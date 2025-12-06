use std::{fs::read_to_string, str::FromStr};

pub fn solve() {
    let matrix = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .flat_map(Element::from_str)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = matrix[0].len();
    let height = matrix.len();
    let mut sum = 0;

    for x in 0..width {
        let op = matrix[height - 1][x];
        let Element::Num(mut accum) = matrix[0][x] else {
            panic!()
        };

        for y in 1..height - 1 {
            let elem = matrix[y][x];
            accum = op.apply(accum, elem);
        }

        sum += accum;
    }

    println!("{sum}");
}

#[derive(Clone, Copy)]
enum Element {
    Num(u64),
    Add,
    Mul,
}

impl FromStr for Element {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Mul),
            s => Ok(Self::Num(s.parse().map_err(|_| ())?)),
        }
    }
}

impl Element {
    fn apply(&self, lhs: u64, rhs: Element) -> u64 {
        let Self::Num(rhs) = rhs else { panic!() };

        match self {
            Element::Num(_) => panic!(),
            Element::Add => lhs + rhs,
            Element::Mul => lhs * rhs,
        }
    }
}
