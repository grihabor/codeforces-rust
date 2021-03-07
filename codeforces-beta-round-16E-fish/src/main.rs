use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use std::convert;
use std::convert::TryInto;
use std::fmt;
use std::io;
use std::io::Read;
use std::ops;
use std::ops::{Add, Index};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct FishSet(u32);

impl FishSet {
    fn new(n: u32) -> Self {
        Self((1 << n) - 1)
    }

    fn pair(x: Fish, y: Fish) -> Self {
        FishSet(0) + x + y
    }

    fn array(&self) -> ([Fish; 18], usize) {
        let mut fish_vec = [Fish(0); 18];
        let fish_vec_len = self
            .into_iter()
            .enumerate()
            .map(|(i, fish)| fish_vec[i] = fish)
            .count();
        return (fish_vec, fish_vec_len);
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
            if next_fish >= 18 {
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct WinSetFish<SetBits, FishBits>(u32, SetBits, FishBits);

struct FishPair {
    winner: Fish,
    looser: Fish,
}

impl<SetBits, FishBits> convert::TryInto<FishPair> for WinSetFish<SetBits, FishBits>
where
    SetBits: U32 + Default,
    FishBits: U32 + Default,
{
    type Error = ();

    fn try_into(self) -> Result<FishPair, ()> {
        let (arr, len) = self.set().array();
        if len == 2 {
            let winner = self.fish();
            Ok(FishPair {
                winner,
                looser: if arr[0] == winner { arr[1] } else { arr[0] },
            })
        } else {
            Err(())
        }
    }
}

impl FishPair {
    fn new(winner: Fish, looser: Fish) -> Self {
        FishPair { winner, looser }
    }
}

impl<SetBits, FishBits> convert::From<FishPair> for WinSetFish<SetBits, FishBits>
where
    SetBits: U32 + Default,
    FishBits: U32 + Default,
{
    fn from(pair: FishPair) -> Self {
        WinSetFish::new(pair.winner, FishSet::pair(pair.winner, pair.looser))
    }
}

impl<SetBits, FishBits> WinSetFish<SetBits, FishBits>
where
    SetBits: U32 + Default,
    FishBits: U32 + Default,
{
    fn pair(winner: Fish, looser: Fish) -> Self {
        FishPair { winner, looser }.into()
    }

    fn new(fish: Fish, set: FishSet) -> Self {
        let ones = (1 << FishBits::u32()) - 1;
        WinSetFish(
            set.0 << FishBits::u32() | fish.0 & ones,
            SetBits::default(),
            FishBits::default(),
        )
    }

    fn fish(&self) -> Fish {
        let ones = (1 << FishBits::u32()) - 1;
        Fish(self.0 & ones)
    }

    fn set(&self) -> FishSet {
        FishSet(self.0 >> FishBits::u32())
    }
}

trait U32 {
    fn u32() -> u32;
}

#[derive(Debug, Clone, Copy, Default)]
struct Const18;
impl U32 for Const18 {
    #[inline(always)]
    fn u32() -> u32 {
        18
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Const5;
impl U32 for Const5 {
    #[inline(always)]
    fn u32() -> u32 {
        5
    }
}

type Win = WinSetFish<Const18, Const5>;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct WinFishSet<SetBits, FishBits>(u32, SetBits, FishBits);

impl<SetBits, FishBits> WinFishSet<SetBits, FishBits>
where
    SetBits: U32 + Default,
    FishBits: U32 + Default,
{
    fn pair(winner: Fish, looser: Fish) -> Self {
        WinFishSet::new(winner, FishSet::pair(winner, looser))
    }

    fn new(fish: Fish, set: FishSet) -> Self {
        let ones = (1 << SetBits::u32()) - 1;
        WinFishSet(
            fish.0 << SetBits::u32() | set.0 & ones,
            SetBits::default(),
            FishBits::default(),
        )
    }

    fn fish(&self) -> Fish {
        Fish(self.0 >> SetBits::u32())
    }

    fn set(&self) -> FishSet {
        let ones = (1 << SetBits::u32()) - 1;
        FishSet(self.0 & ones)
    }
}

#[derive(Debug)]
struct Matrix<T> {
    data: Vec<T>,
    width: u32,
}

impl<T> Matrix<T>
where
    T: Default + Clone,
{
    fn new(height: u32, width: u32) -> Self {
        Matrix {
            width,
            data: vec![T::default(); (width * height) as usize],
        }
    }
}

impl<T> ops::Index<(u32, u32)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (u32, u32)) -> &Self::Output {
        &self.data[(index.0 * self.width + index.1) as usize]
    }
}

impl<T> ops::IndexMut<(u32, u32)> for Matrix<T> {
    fn index_mut(&mut self, index: (u32, u32)) -> &mut Self::Output {
        &mut self.data[(index.0 * self.width + index.1) as usize]
    }
}

type Float = f64;

type WinProbability = WinProbabilityGeneric<Const18, Const5>;

impl WinProbability {}

#[derive(Debug)]
struct WinProbabilityGeneric<SetBits, FishBits>(Vec<Float>, Matrix<Float>, SetBits, FishBits);

impl<SetBits, FishBits> WinProbabilityGeneric<SetBits, FishBits>
where
    SetBits: U32 + Default,
    FishBits: U32 + Default,
{
    fn new() -> Self {
        WinProbabilityGeneric(
            vec![-1.; 1 << (SetBits::u32() + FishBits::u32())],
            Matrix::new(SetBits::u32(), SetBits::u32()),
            SetBits::default(),
            FishBits::default(),
        )
    }

    fn insert_set(&mut self, win: Win, probability: Float) {
        self.0[win.0 as usize] = probability;
    }

    fn get(&self, target: Win) -> Option<Float> {
        let probability = if let Ok(pair) = target.try_into() {
            self.wins_pair(pair)
        } else {
            self.0[target.0 as usize]
        };
        if probability < 0. {
            None
        } else {
            Some(probability)
        }
    }

    fn wins_set(&mut self, target: Win) -> Float {
        if let Some(probability) = self.get(target) {
            return probability;
        }

        let m = target.set().into_iter().count();
        let branch_count = to_float(m * (m - 1) / 2);

        let result = (target.set() - target.fish())
            .into_iter()
            .map(|first_looser: Fish| -> Float {
                let survivors = target.set() - first_looser;
                survivors
                    .into_iter()
                    .map(|first_winner| self.wins(Win::pair(first_winner, first_looser)))
                    .sum::<Float>()
                    * self.wins(Win::new(target.fish(), survivors))
            })
            .sum::<Float>()
            / branch_count;
        self.insert_set(target.clone(), result);
        result
    }

    fn wins(&mut self, target: Win) -> Float {
        if let Ok(pair) = target.try_into() {
            self.wins_pair(pair)
        } else {
            self.wins_set(target)
        }
    }

    fn wins_pair(&self, pair: FishPair) -> Float {
        self.1[(pair.winner.0, pair.looser.0)]
    }

    fn insert(&mut self, win: Win, probability: Float) {
        if let Ok(pair) = win.try_into() {
            self.insert_pair(pair, probability)
        } else {
            self.insert_set(win, probability)
        }
    }

    fn insert_pair(&mut self, pair: FishPair, probability: Float) {
        self.1[(pair.winner.0, pair.looser.0)] = probability
    }
}

fn to_float(x: usize) -> Float {
    let x: u16 = x.try_into().unwrap();
    x.try_into().unwrap()
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("read n failed");
    let n = buffer.trim().parse().expect("failed to parse n");
    // let n = 4;
    let memoized = WinProbability::new();
    let mut memoized = read_probabilities(&mut buffer, n, memoized);

    // println!("{:?}", memoized);
    for i in 0..n {
        let member = Win::new(Fish(i), FishSet::new(n));
        let probability = memoized.wins(member);
        print!("{} ", fmt_float(probability));
    }
    // println!();
    // println!("{:?}", memoized);
}

fn read_probabilities(
    buffer: &mut String,
    n: u32,
    mut probabilities: WinProbability,
) -> WinProbability {
    for i in 0..n {
        buffer.clear();
        io::stdin()
            .read_line(buffer)
            .expect(&format!("failed to read line {}", i));
        let row = buffer.split(" ").map(|v| {
            v.trim()
                .parse::<Float>()
                .expect(&format!("failed to parse value: {}", v))
        });
        for (j, probability) in row.enumerate() {
            let j = j.try_into().unwrap();
            let win = Win::new(Fish(i), FishSet(0) + Fish(i) + Fish(j));
            probabilities.insert(win, probability);
        }
    }
    probabilities
}

fn fmt_float(x: Float) -> String {
    return format!("{:.6}", x);
}

mod tests {
    use crate::{fmt_float, Fish, FishSet, FishSetIter, Float, Win, WinProbability};
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
    fn equal_probability_3() {
        let mut proba = WinProbability::new();
        proba.insert(Win::pair(Fish(0), Fish(1)), 0.5);
        proba.insert(Win::pair(Fish(1), Fish(0)), 0.5);
        proba.insert(Win::pair(Fish(0), Fish(2)), 0.5);
        proba.insert(Win::pair(Fish(2), Fish(0)), 0.5);
        proba.insert(Win::pair(Fish(1), Fish(2)), 0.5);
        proba.insert(Win::pair(Fish(2), Fish(1)), 0.5);
        for i in 0..3 {
            let actual = proba.wins(Win::new(Fish(i), FishSet::new(3)));
            assert_eq!(fmt_float(actual), "0.333333");
        }
    }

    #[test]
    fn equal_probability_2() {
        let mut proba = WinProbability::new();
        proba.insert(Win::pair(Fish(0), Fish(1)), 0.5);
        proba.insert(Win::pair(Fish(1), Fish(0)), 0.5);
        for i in 0..2 {
            let actual = proba.wins(Win::new(Fish(i), FishSet::new(2)));
            assert_eq!(fmt_float(actual), "0.500000");
        }
    }

    #[test]
    fn win_probability() {
        let mut proba = WinProbability::new();
        proba.insert(Win::pair(Fish(0), Fish(1)), 0.5);
        proba.insert(Win::pair(Fish(1), Fish(0)), 0.5);
        proba.insert(Win::pair(Fish(0), Fish(2)), 0.4);
        proba.insert(Win::pair(Fish(2), Fish(0)), 0.6);
        proba.insert(Win::pair(Fish(1), Fish(2)), 0.3);
        proba.insert(Win::pair(Fish(2), Fish(1)), 0.7);

        let actual: Vec<String> = (0..3)
            .map(|i| proba.wins(Win::new(Fish(i), FishSet::new(3))))
            .map(fmt_float)
            .collect();

        let expected = vec!["0.276667", "0.226667", "0.496667"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn win_probability_zeros() {
        let mut proba = WinProbability::new();
        proba.insert(Win::pair(Fish(0), Fish(1)), 1.0);
        proba.insert(Win::pair(Fish(1), Fish(0)), 0.0);
        proba.insert(Win::pair(Fish(0), Fish(2)), 1.0);
        proba.insert(Win::pair(Fish(2), Fish(0)), 0.0);
        proba.insert(Win::pair(Fish(1), Fish(2)), 0.5);
        proba.insert(Win::pair(Fish(2), Fish(1)), 0.5);

        let actual: Vec<String> = (0..3)
            .map(|i| proba.wins(Win::new(Fish(i), FishSet::new(3))))
            .map(fmt_float)
            .collect();

        let expected = vec!["1.000000", "0.000000", "0.000000"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn large() {
        let n = 18;
        let mut proba = WinProbability::new();
        for i in 0..n {
            for j in 0..n {
                let val = if i == j { 0.0 } else { 0.5 };
                proba.insert(Win::pair(Fish(i), Fish(j)), val);
            }
        }

        let actual: Vec<String> = (0..n)
            .map(|i| proba.wins(Win::new(Fish(i), FishSet::new(n))))
            .map(fmt_float)
            .collect();

        let expected: Vec<String> = (0..n).map(|_| fmt_float(1. / (n as Float))).collect();
        assert_eq!(actual, expected);
    }
}
