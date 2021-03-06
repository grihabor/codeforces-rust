use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt;
use std::io;
use std::io::Read;
use std::ops;
use std::ops::Add;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct FishSet(u32);

impl FishSet {
    fn new(n: u32) -> Self {
        Self((1 << n) - 1)
    }

    fn pair(x: Fish, y: Fish) -> Self {
        FishSet(0) + x + y
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

impl ops::Sub<Fish> for FishSet {
    type Output = FishSet;

    fn sub(self, rhs: Fish) -> Self::Output {
        FishSet(self.0 & !(1 << rhs.0))
    }
}

impl ops::Add<Fish> for FishSet {
    type Output = FishSet;

    fn add(self, rhs: Fish) -> Self::Output {
        FishSet(self.0 | (1 << rhs.0))
    }
}

impl From<Fish> for FishSet {
    fn from(f: Fish) -> Self {
        FishSet(1 << f.0)
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

#[derive(Debug)]
struct WinProbability(HashMap<Win, f64>);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Win {
    fish: Fish,
    set: FishSet,
}

impl Win {
    fn new(winner: Fish, looser: Fish) -> Self {
        Win {
            fish: winner,
            set: FishSet::pair(winner, looser),
        }
    }
}
impl WinProbability {
    fn wins(&mut self, target: Win) -> f64 {
        if let Some(probability) = self.0.get(&target) {
            return probability.clone();
        }
        let m = target.set.into_iter().count();

        let pairs = target
            .set
            .into_iter()
            .flat_map(|x| target.set.into_iter().map(move |y| (x, y)));

        let mut probability = 0f64;
        for (x, y) in pairs {
            if x == y || y == target.fish {
                continue;
            }
            probability += self.wins(Win::new(x, y))
                * self.wins(Win {
                    fish: target.fish,
                    set: target.set - y,
                });
        }

        let branch_count = to_f64(m * (m - 1) / 2);
        let result = probability / branch_count;
        self.0.insert(target, result);
        result
    }
}

fn to_f64(x: usize) -> f64 {
    let x: i32 = x.try_into().unwrap();
    x.try_into().unwrap()
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("read n failed");
    let n = buffer.trim().parse().expect("failed to parse n");
    // let n = 4;
    let probabilities = read_probabilities(&mut buffer, n);

    let mut memoized = WinProbability(probabilities);
    println!("{:?}", memoized);
    for i in 0..n {
        let member = Win {
            fish: Fish(i),
            set: FishSet::new(n),
        };
        let probability = memoized.wins(member);
        print!("{} ", fmt_f64(probability));
    }
    println!();
    println!("{:?}", memoized);
}

fn read_probabilities(buffer: &mut String, n: u32) -> HashMap<Win, f64> {
    let mut probabilities: HashMap<Win, f64> = HashMap::new();
    for i in 0..n {
        buffer.clear();
        io::stdin()
            .read_line(buffer)
            .expect(&format!("failed to read line {}", i));
        let row = buffer.split(" ").map(|v| {
            v.trim()
                .parse::<f64>()
                .expect(&format!("failed to parse value: {}", v))
        });
        for (j, probability) in row.enumerate() {
            let j = j.try_into().unwrap();
            let win = Win {
                fish: Fish(i),
                set: FishSet(0) + Fish(i) + Fish(j),
            };
            probabilities.insert(win, probability);
        }
    }
    probabilities
}

fn fmt_f64(x: f64) -> String {
    return format!("{:.6}", x);
}

mod tests {
    use crate::{fmt_f64, Fish, FishSet, FishSetIter, Win, WinProbability};
    use std::collections::HashMap;
    use std::convert::TryInto;
    use std::num::NonZeroI32;

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

    #[test]
    fn equal_probability() {
        let mut proba = HashMap::new();
        proba.insert(Win::new(Fish(0), Fish(1)), 0.5);
        proba.insert(Win::new(Fish(1), Fish(0)), 0.5);
        proba.insert(Win::new(Fish(0), Fish(2)), 0.5);
        proba.insert(Win::new(Fish(2), Fish(0)), 0.5);
        proba.insert(Win::new(Fish(1), Fish(2)), 0.5);
        proba.insert(Win::new(Fish(2), Fish(1)), 0.5);
        let mut probabilities = WinProbability(proba);
        for i in 0..3 {
            let actual = probabilities.wins(Win {
                fish: Fish(i),
                set: FishSet::new(3),
            });
            assert_eq!(fmt_f64(actual), "0.333333");
        }
    }

    #[test]
    fn win_probability() {
        let mut proba = HashMap::new();
        proba.insert(Win::new(Fish(0), Fish(1)), 0.5);
        proba.insert(Win::new(Fish(1), Fish(0)), 0.5);
        proba.insert(Win::new(Fish(0), Fish(2)), 0.4);
        proba.insert(Win::new(Fish(2), Fish(0)), 0.6);
        proba.insert(Win::new(Fish(1), Fish(2)), 0.3);
        proba.insert(Win::new(Fish(2), Fish(1)), 0.7);
        let mut probabilities = WinProbability(proba);

        let actual: Vec<String> = (0..3)
            .map(|i| {
                probabilities.wins(Win {
                    fish: Fish(i),
                    set: FishSet::new(3),
                })
            })
            .map(fmt_f64)
            .collect();

        let expected = vec!["0.276667", "0.226667", "0.496667"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn win_probability_zeros() {
        let mut proba = HashMap::new();
        proba.insert(Win::new(Fish(0), Fish(1)), 1.0);
        proba.insert(Win::new(Fish(1), Fish(0)), 0.0);
        proba.insert(Win::new(Fish(0), Fish(2)), 1.0);
        proba.insert(Win::new(Fish(2), Fish(0)), 0.0);
        proba.insert(Win::new(Fish(1), Fish(2)), 0.5);
        proba.insert(Win::new(Fish(2), Fish(1)), 0.5);
        let mut probabilities = WinProbability(proba);

        let actual: Vec<String> = (0..3)
            .map(|i| {
                probabilities.wins(Win {
                    fish: Fish(i),
                    set: FishSet::new(3),
                })
            })
            .map(fmt_f64)
            .collect();

        let expected = vec!["1.000000", "0.000000", "0.000000"];
        assert_eq!(actual, expected);
    }
}
