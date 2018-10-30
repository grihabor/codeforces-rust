use std::rc::Rc;
use std::collections::*;
use std::io::{self, Read};
use std::{result};


type State = Vec<bool>;
type ILong = i64;
type ULong = u64;

#[derive(Debug)]
struct StateGenerator {
    v: Option<State>,
}

trait Increment {
    fn inc(self) -> Option<Self> where Self: std::marker::Sized;
}

fn _inc_state(mut state: State, idx: usize) -> Option<State> {
    if idx >= state.len() {
        return None
    }
    state[idx] = !state[idx];
    if !state[idx] {
        _inc_state(state, idx + 1)
    } else {
        Some(state)
    }
}

impl Increment for State {
    fn inc(self) -> Option<State> {
        _inc_state(self, 0)
    }
}

impl StateGenerator {
    fn new(n_columns: usize) -> Self {
        let mut v = Vec::with_capacity(2 * n_columns);
        v.resize(2 * n_columns, false);
        Self{v: Some(v)}
    }
}

impl Iterator for StateGenerator {
    type Item = State;
    fn next(&mut self) -> Option<Self::Item> {
        let cloned_v = self.v.clone();
        let next_value = cloned_v.and_then(|value| {
            value.inc()
        });
        std::mem::replace(&mut self.v, next_value)
    }
}

struct Grid {
    n_components: usize,
    iterator: StateGenerator,
}

impl Grid {
    fn new(args: Args) -> Grid {
        Grid{
            n_components: args.n_components, 
            iterator: StateGenerator::new(args.n_columns),
        }
    }
}

type Row<'a> = std::slice::Iter<'a, bool>;

trait Columns<'a> {
    fn first_row(&'a self) -> Row<'a>;
    fn last_row(&'a self) -> Row<'a>;
    fn columns(&'a self) -> std::iter::Zip<Row<'a>, Row<'a>>;
}

impl<'a> Columns<'a> for State {
    fn first_row(&'a self) -> Row<'a> {
        let half = self.len() / 2;
        self[..half].iter()
    }
    fn last_row(&'a self) -> Row<'a> {
        let half = self.len() / 2;
        self[half..].iter()
    }
    fn columns(&'a self) -> std::iter::Zip<Row<'a>, Row<'a>> {
        self.first_row().zip(self.last_row())
    }
}

trait Components {
    fn n_components(&self) -> usize;
}

impl Components for State {
    fn n_components(&self) -> usize {
        let columns = self.columns();
        let opt = columns.clone().next();
        if let None = opt {
            return 0
        } 
        let first = opt.unwrap();

        let mut count = if first.0 != first.1 {2} else {1};

        for (prev, cur) in columns.clone().zip(columns.skip(1)) {
            count = match (cur.0 == prev.0, cur.1 == prev.1, cur.0 == cur.1) {
                ( true,  true,     _) => count,
                ( true, false,  true) => count,
                (false,  true,  true) => count,
                ( true, false, false) => count + 1,
                (false,  true, false) => count + 1,
                (false, false,  true) => count + 1,
                (false, false, false) => count + 2,
            }
        }
        count
    }
}

trait CustomDisplay {
    fn display(&self) -> String;
}

fn into(row: Row) -> Vec<u8> {
    row.map(|item| match item {&true => 1u8, &false => 0u8}).collect()
}

impl CustomDisplay for State {
    fn display(&self) -> String {

        format!(
            "[{:?}\n {:?}]", 
            into(self.first_row()),
            into(self.last_row().into()),
        )   
    }
}

impl Iterator for Grid {
    type Item = State;
    fn next(&mut self) -> Option<Self::Item> {
        //
        // In newer versions replace the code with Option::filter
        //
        while let Some(state) = self.iterator.next() {
            eprintln!("{} -> {}", state.display(), state.n_components());
            eprintln!("DEBUG: {}", state.n_components());
            if state.n_components() == self.n_components {
                return Some(state)
            }
        }
        None
    }
}

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

fn answer_slow() -> () {
    if let Ok(args) = get_args() {
        let grid = Grid::new(args);
        println!("{}", grid.into_iter().count());
    } else {
        eprintln!("Failed to read arguments, pass 2 integers to the stdin");
    }
}


trait Merge {
    fn merge(&self, rhs: &Self) -> Self;
}

struct Birow {
    head: (bool, bool),
    tail: (bool, bool),

    /// Mapping from n_components to the count
    components: Rc<HashMap<ULong, ULong>>,
}

impl Birow {
    fn new(column: (bool, bool)) -> Birow {
        let mut components = HashMap::new();
        if column.0 == column.1 {
            components.insert(2, 1);
        } else {
            components.insert(1, 2);
        }
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
            ((false, false), ( true,  true)) => 0,
            (( true,  true), (false, false)) => 0,
            ((false,  true), (false,  true)) => -2,
            (( true, false), ( true, false)) => -2,
            ((false,  true), ( true, false)) => 0,
            (( true, false), (false,  true)) => 0,
            _ => -1,
        };
        let mut merged_components = HashMap::new();
        for (key, count) in self.components.iter() {
            for (rhs_key, rhs_count) in rhs.components.iter() {
                let merged_key = (key + rhs_key) as ILong + shift;
                let e = merged_components.entry(merged_key as ULong).or_insert(0);
                *e += count * rhs_count;
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

impl Merge for BirowPerm {
    fn merge(&self, rhs: &Self) -> Self {
        let mut merged_samples = Vec::new();
        for sample in self.samples.iter() {
            for rhs_sample in rhs.samples.iter() {
                merged_samples.push(sample.merge(rhs_sample));
            }
        }
        BirowPerm {
            len: self.len + rhs.len,
            samples: Rc::new(merged_samples),
        }
    }
}

fn get_stats_slow(i: usize) -> HashMap<usize, usize> {
    StateGenerator::new(i).map(|x| x.n_components()).fold(
        HashMap::new(),
        |mut acc, x| {*acc.entry(x).or_insert(0) += 1; acc},
    )
}

fn main() -> () {
    for i in 1..10 {
        println!("{:?}", get_stats_slow(i));
    }
}
