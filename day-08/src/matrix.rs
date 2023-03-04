use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct Matrix {
    width: usize,
    height: usize,
    elems: HashMap<(usize, usize), u8>
}

impl Matrix {

    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height, elems: HashMap::new() }
    }

    pub fn size(&self) -> (usize, usize) {
        return (self.width, self.height)
    }

    pub fn set(&mut self, x: usize, y: usize, value: u8) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }

        self.elems.insert((x, y), value);
        true
    }

    pub fn get(&self, x: usize, y: usize) -> &u8 {
        self.elems.get(&(x, y)).unwrap_or(&0)
    }

    pub fn get_row(&self, y: usize) -> Vec<&u8> {
        let mut row: Vec<&u8> = Vec::new();
        for x in 0..self.width {
            row.push(self.get(x, y))
        }

        row
    }

    pub fn get_col(&self, x: usize) -> Vec<&u8> {
        let mut col: Vec<&u8> = Vec::new();
        for y in 0..self.height {
            col.push(self.get(x, y))
        }

        col
    }

    fn display(&self) -> String {
        let mut lines: Vec<String> = Vec::new();

        for y in 0..self.height {
            let mut line: Vec<String> = Vec::new();
            for x in 0..self.width {
                line.push(self.get(x, y).to_string());
            }
            lines.push(line.join(" "));
        }

        lines.join("\n")
    }
}

impl fmt::Display for Matrix {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}