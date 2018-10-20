use std::io::{self, Read};
use std::{result};


type State = Vec<bool>;

#[derive(Debug)]
struct StateGenerator {
    v: Option<State>,
}

trait Increment {
    fn inc(&mut self) -> Option<()>;
}

impl Increment for [bool] {
    fn inc(&mut self) -> Option<()> {
        if self.len() == 0 {
        	println!("None! {:?}", self);
            return None
        }
        match self[0] {
            true => {
                self[0] = false;
                self[1..].inc()
            }
            false => {
                self[0] = true;
                Some(())
            }
        }
    }
}

impl Increment for State {
    fn inc(&mut self) -> Option<()> {
    	(&mut self[..]).inc()
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
        let next_value = cloned_v.map(|mut value| {
        	value.inc();
        	value
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

trait Components {
    fn n_components(&self) -> usize;
}

impl Components for State {
    fn n_components(&self) -> usize {
        0
    }
}

impl Iterator for Grid {
    type Item = State;
    fn next(&mut self) -> Option<Self::Item> {
        //
        // In newer versions replace the code with Option::filter
        //
        while let Some(state) = self.iterator.next() {
        	println!("{:?}", state);
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

fn main() -> () {
    if let Ok(args) = get_args() {
	    eprintln!("DEBUG: {:?}", args);
	    let grid = Grid::new(args);
	    for state in grid {
	        println!("{:?}", state);
	    }
	} else {
		eprintln!("Failed to read arguments, pass 2 integers to the stdin");
	}
}