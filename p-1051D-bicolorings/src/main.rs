#![feature(test)]

extern crate test;

use std::rc::Rc;
use std::collections::*;
use std::io::{self, Read};
use std::{result};


type ILong = i64;
type ULong = u64;
type Map<K, V> = BTreeMap<K, V>;
type Counter = Map<usize, ULong>;


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

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
enum Edge {
    FF,
    FT,
    TF,
    TT,
}

trait Merge {
    fn merge(&self, rhs: &Self) -> Self;
}

#[derive(Debug)]
struct Birow {
    head: Edge,
    tail: Edge,

    /// Mapping from n_components to the count
    components: Rc<Counter>,
}

impl Birow {
    fn new(column: Edge) -> Birow {
        let mut components = Map::new();
        let n_components = match column {
            Edge::FF | Edge::TT => 1,
            Edge::FT | Edge::TF => 2,
        };
        components.insert(n_components, 1);
        Birow {
            head: column,
            tail: column,
            components: Rc::new(components),
        }
    }
}

impl Merge for Birow {
    fn merge(&self, rhs: &Self) -> Self {
        let shift: ILong = match (self.tail, rhs.head) {
            (Edge::FF, Edge::TT) => 0,
            (Edge::TT, Edge::FF) => 0,
            (Edge::FT, Edge::FT) => -2,
            (Edge::TF, Edge::TF) => -2,
            (Edge::FT, Edge::TF) => 0,
            (Edge::TF, Edge::FT) => 0,
            _ => -1,
        };
        let mut merged_components = Counter::new();
        for (key, count) in self.components.iter() {
            for (rhs_key, rhs_count) in rhs.components.iter() {
                let merged_key = (key + rhs_key) as ILong + shift;
                {
                    let e = merged_components.entry(merged_key as usize).or_insert(0);
                    *e = (*e + (((count % TOP) * (rhs_count % TOP)) % TOP)) % TOP;
                }
            }
        }
        Birow {
            head: self.head, 
            tail: rhs.tail,
            components: Rc::new(merged_components),
        }
    }
}

struct BirowPerm {
    len: usize,

    /// List of all possible Birow instances for the particular len
    samples: Rc<Vec<Birow>>,
}

impl BirowPerm {
    fn new() -> Self {
        BirowPerm {
            len: 1,
            samples: Rc::new(vec![
                Birow::new(Edge::FF),
                Birow::new(Edge::FT),
                Birow::new(Edge::TF),
                Birow::new(Edge::TT),
            ]),
        }
    }

    fn build(n_columns: usize) -> Self {
        match n_columns {
            0 => panic!("n_columns must be > 0, it should have been validated during argument validation"),
            1 => BirowPerm::new(),
            n_columns => {
                let bp = BirowPerm::build(n_columns / 2);
                let semiresult = bp.merge(&bp);
                if n_columns % 2 == 1 {
                    semiresult.merge(&BirowPerm::new())
                } else {
                    semiresult
                }
            }
        }
    }

    fn components(&self) -> Counter {
        self.samples.iter().fold(
            Counter::new(),
            |acc, x| {acc.merge(Rc::make_mut(&mut x.components.clone()))}
        )
    }
}

impl Merge for Counter {
    fn merge(&self, rhs: &Self) -> Self {
        let mut result = self.clone();
        for (key, value) in rhs.iter() {
            let e = result.entry(*key).or_insert(0);
            *e += value;
        }
        result
    }
}

impl Merge for BirowPerm {
    fn merge(&self, rhs: &Self) -> Self {
        let mut merged_samples = Map::new();
        for sample in self.samples.iter() {
            for rhs_sample in rhs.samples.iter() {
                let birow = sample.merge(rhs_sample);
                let key = (birow.head, birow.tail);
                let e = merged_samples.entry(key).or_insert(Rc::new(Map::new()));
                *e = Rc::new((*e).merge(&birow.components));
            }
        }
        let birows: Vec<Birow> = merged_samples.iter()
            .map(|(key, value)| {
                let (head, tail) = key;
                Birow{
                    head: *head,
                    tail: *tail,
                    components: Rc::clone(value),
                }
            }).collect();
        BirowPerm {
            len: self.len + rhs.len,
            samples: Rc::new(birows),
        }
    }
}

fn get_stats_fast(args: &Args) -> Counter {
    BirowPerm::build(args.n_columns).components()
}

static TOP: u64 = 998244353;

fn main() -> () {
    let args = get_args().unwrap();
    println!("{}", get_stats_fast(&args)[&args.n_components]);
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_get_stats_fast() {
        let args = Args {n_columns: 10, n_components: 10};
        assert_eq!(63862, get_stats_fast(&args)[&args.n_components]);
    }

    #[bench]
    fn bench_get_stats_fast(b: &mut Bencher) {
        let args = Args {n_columns: 80, n_components: 80};
        b.iter(|| get_stats_fast(&args)[&args.n_components]);
    }
}
