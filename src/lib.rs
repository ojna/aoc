use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[allow(dead_code)]
pub struct InputLine {
    pub value: String,
}

impl From<String> for InputLine {
    fn from(value: String) -> InputLine {
        InputLine { value }
    }
}

pub struct InputLines {
    pub values: Vec<InputLine>,
}

impl Default for InputLines {
    fn default() -> InputLines {
        InputLines { values: vec![] }
    }
}

pub trait Puzzle<T, E, C> {
    fn run(&self, config: C) -> Result<T, E>;
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn input_file(filename: &str) -> Result<InputLines, &'static str> {
    if let Ok(lines) = read_lines(filename) {
        let mut res = InputLines::default();

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(input) = line {
                res.values.push(InputLine::from(input.to_owned()));
            }
        }

        Ok(res)
    } else {
        Err("File not found")
    }
}
