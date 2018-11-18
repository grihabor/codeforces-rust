use std::io::{self, Read};
use std::{result};


#[derive(Debug)]
struct Args {
    n_columns: usize,
    n_components: usize,
}

fn get_args() -> Result<Args, std::io::Error> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    let args: Vec<usize> = buf
                        .split_whitespace()
                        .map(|s| s.parse::<usize>())
                        .filter_map(result::Result::ok)
                        .collect();
    Ok(Args{
        n_columns: *args.get(0).unwrap(),
        n_components: *args.get(1).unwrap(), 
    })
}

#[derive(Debug)]
struct Power {
    x: u64,
    y: u64,
}

struct Layer {
    powers: Vec<Power>,
}

impl Layer {
    fn new() -> Self {
        Layer {
            powers: vec![
                Power {x: 1, y: 0},
                Power {x: 0, y: 1},
            ]
        }
    }

    fn next(self) -> Self {
        let mut next_powers: Vec<Power> = Vec::with_capacity(self.powers.len() + 2);
        for _ in 0..(self.powers.len() + 2) {
            next_powers.push(Power{x: 0, y: 0});
        }
        for (i, power) in self.powers.iter().enumerate() {
            next_powers[i].x += power.x + 2 * power.y;
            next_powers[i].y += power.y;
            next_powers[i + 1].x += power.x;
            next_powers[i + 1].y += 2 * power.x;
            next_powers[i + 2].y += power.y;
        };
        for power in next_powers.iter_mut() {
            power.x %= TOP;
            power.y %= TOP;
        }
        Layer{ powers: next_powers }
    }

    fn build(n: usize) -> Self {
        let mut layer = Layer::new();
        for _ in 1..n {
            layer = layer.next();
        }
        layer
    }

    fn get_case_count(&self, n_components: usize) -> u64 {
        let power = &self.powers[n_components - 1];
        (2 * (power.x + power.y)) % TOP
    }
}

fn get_stats_quadratic(args: &Args) -> u64 {
    Layer::build(args.n_columns).get_case_count(args.n_components)
}

static TOP: u64 = 998244353;

fn main() -> () {
    let args = get_args().unwrap();
    println!("{}", get_stats_quadratic(&args));
}
