use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::Read;
use std::ops::{Shl, Sub};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct FishSet(u32);

impl FishSet {
    fn new(n: u32) -> Self {
        Self((1 << n) - 1)
    }
}

impl fmt::Debug for FishSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("{")?;
        f.write_str(
            &self
                .into_iter()
                .map(|f| format!("{:?}", f))
                .collect::<Vec<String>>()
                .join(","),
        )?;
        f.write_str("}")
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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

struct WinProbability(HashMap<Member, f64>);
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Member {
    fish: Fish,
    set: FishSet,
}
impl WinProbability {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn get(&mut self, member: Member) -> f64 {
        if let Some(probability) = self.0.get(&member) {
            return probability.clone();
        }
        // let subset = member.set - member.fish;
        // subset.into_iter().map(|fish| {
        //     let member = Member{fish, set: subset};
        //     self.get(member)
        // }).fold(0, )
        0.
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("read n failed");
    let n = input.trim().parse().expect("failed to parse n");
    let probabilities = HashMap::new();
    for i in 0..n {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect(&format!("failed to read line {}", i));
        let row = input.split(" ").map(|v| {
            v.trim()
                .parse::<f64>()
                .expect(&format!("failed to parse value: {}", v))
        });
        for (j, probability) in row.enumerate() {
            let member = Member {
                fish: Fish(i),
                set: Fish(i) + Fish(j.into()),
            };
            probabilities[member] = probability
        }
    }

    let mut memoized = WinProbability::new();
    for i in 0..n {
        let member = Member {
            fish: Fish(i),
            set: FishSet::new(n),
        };
        let probability = memoized.get();
        println!("{}", probability)
    }
    // print!("{:?} {:?}", FishSet::new(n), probabilities);
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
