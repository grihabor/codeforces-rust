use std::io::{self, BufRead};

struct CharSetMeta{
    present: bool,
    index: usize,
    count: usize
}

impl std::default::Default for CharSetMeta {
    fn default() -> CharSetMeta {
        CharSetMeta {
            present: false,
            index: 0,
            count: 0
        }
    }
}

impl CharSetMeta {
    fn hit(&mut self, index: usize) {
        self.count += 1;
        self.present = true;
        self.index = index
    }
}

trait ReplaceCharAt {
    fn replace_char_at(&self, idx: usize, char_to_replace: char) -> Self;
}

impl ReplaceCharAt for std::string::String {
    fn replace_char_at(&self, idx: usize, char_to_replace: char) -> Self {
        self.chars().enumerate().map(|(i, ch)| if i == idx {
            char_to_replace
        } else {
            ch
        }).collect()
    }
}


fn main() -> io::Result<()> {
    let s = io::stdin();
    let lock = s.lock();
    let mut first_line = true;
    for line_r in lock.lines() {
        if first_line {
            first_line = false;
            continue
        };
        let line = line_r.unwrap();
        let mut lower: CharSetMeta = Default::default();
        let mut upper: CharSetMeta = Default::default();
        let mut digit: CharSetMeta = Default::default();
        for (index, ch) in line.chars().enumerate() {
            match ch {
                'a' ... 'z' => {lower.hit(index)},
                'A' ... 'Z' => {upper.hit(index)},
                '0' ... '9' => {digit.hit(index)},
                _ => {println!("{}", ch)}
            }
        }
        
        println!("{}", match (lower.present, upper.present, digit.present) {
            (false, false, false) => 0.to_string(),
            (false, false, true) => line.replace_char_at(0, 'a').replace_char_at(1, 'A'),
            (false, true, false) => line.replace_char_at(0, 'a').replace_char_at(1, '1'),
            (false, true, true) => line.replace_char_at(if upper.count > 1 {upper.index} else {digit.index}, 'a'),
            (true, false, false) => line.replace_char_at(0, 'A').replace_char_at(1, '1'),
            (true, false, true) => line.replace_char_at(if lower.count > 1 {lower.index} else {digit.index}, 'A'),
            (true, true, false) => line.replace_char_at(if lower.count > 1 {lower.index} else {upper.index}, '1'),
            (true, true, true) => line
        })
    }
    Ok(()) 
}
