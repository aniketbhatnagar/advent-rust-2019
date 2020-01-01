use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct TextFile {
    pub path: String
}

impl TextFile {

    pub fn read_lines<T>(&self, mapper: impl Fn(String) -> T) -> impl Iterator<Item = T> {
        let file = File::open(&self.path).unwrap();
        let reader = BufReader::new(file);
        reader.lines()
              .map(Result::unwrap)
              .map(mapper)
    }

    pub fn read_lines_i64(&self) -> impl Iterator<Item = i64> {
        self.read_lines(|line| line.parse::<i64>().unwrap())
    }
}

