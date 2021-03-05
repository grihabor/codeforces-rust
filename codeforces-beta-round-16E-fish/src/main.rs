use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::ops::{Shl, Sub};

#[derive(Debug)]
struct FishSet(u32);

impl FishSet {
    fn new(n: u32) -> Self {
        Self((1 << n) - 1)
    }
}

impl Sub<Fish> for FishSet {
    type Output = FishSet;

    fn sub(self, rhs: Fish) -> Self::Output {
        FishSet(self.0 & !(1 << rhs.0))
    }
}

impl IntoIterator for FishSet {
    type Item = Fish;
    type IntoIter = FishSetIter;

    fn into_iter(self) -> Self::IntoIter {
        FishSetIter::new(self)
    }
}

struct FishSetIter(FishSet, Option<Fish>);

impl FishSetIter {
    fn new(set: FishSet) -> Self {
        Self(set, Some(Fish(0)))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Fish(u32);

impl Iterator for FishSetIter {
    type Item = Fish;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_fish = self.1?.0;
        loop {
            if next_fish > 20 {
                self.1 = None;
                return None;
            }
            let fish_present = self.0 .0 & (1 << next_fish);
            if fish_present != 0 {
                self.1 = Some(Fish(next_fish + 1));
                return Some(Fish(next_fish));
            }
            next_fish += 1;
        }
    }
}

struct WinProbability(HashMap<(usize, FishSet), f64>);

impl WinProbability {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn get(&mut self, i: usize, set: FishSet) -> f64 {
        // if let Some(probability) = self.0.get(&(i, set)) {
        //     return probability.clone()
        // }
        0.
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("read n failed");
    let n = input.trim().parse().expect("failed to parse n");
    let probabilities = (0..n)
        .map(|i| {
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect(&format!("failed to read line {}", i));
            input
                .split(" ")
                .map(|v| {
                    v.trim()
                        .parse::<f64>()
                        .expect(&format!("failed to parse value: {}", v))
                })
                .collect()
        })
        .collect::<Vec<Vec<f64>>>();

    // let memoized = WinProbability::new();
    // (0..n).map(|i| {
    //     memoized.get()
    // })
    print!("{:?} {:?}", FishSet::new(n), probabilities);
}

mod tests {
    use crate::{Fish, FishSet, FishSetIter};

    #[test]
    fn iter_fish_set_all() {
        let mut it = FishSet::new(5).into_iter();
        assert_eq!(it.next().unwrap(), Fish(0));
        assert_eq!(it.next().unwrap(), Fish(1));
        assert_eq!(it.next().unwrap(), Fish(2));
        assert_eq!(it.next().unwrap(), Fish(3));
        assert_eq!(it.next().unwrap(), Fish(4));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn iter_fish_set() {
        let mut set = FishSet::new(5);
        set = set - Fish(0) - Fish(3);
        let mut it = set.into_iter();
        assert_eq!(it.next().unwrap(), Fish(1));
        assert_eq!(it.next().unwrap(), Fish(2));
        assert_eq!(it.next().unwrap(), Fish(4));
        assert_eq!(it.next(), None);
    }
}
