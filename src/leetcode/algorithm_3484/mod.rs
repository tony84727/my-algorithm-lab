use regex::Regex;

pub struct Spreadsheet {
    cells: Vec<Vec<i32>>,
    pattern: Regex,
}

impl Spreadsheet {
    pub fn new(rows: i32) -> Self {
        Self {
            cells: vec![vec![0; rows as usize]; 26],
            pattern: Regex::new(r#"=([^+]+)\+([^+]+)"#).unwrap(),
        }
    }

    pub fn set_cell(&mut self, cell: String, value: i32) {
        if let Some((column, row)) = Self::parse_coordinate(&cell) {
            self.set_cell_at(column, row, value);
        }
    }

    pub fn reset_cell(&mut self, cell: String) {
        self.set_cell(cell, 0);
    }

    pub fn get_value(&self, formula: String) -> i32 {
        let captures = self.pattern.captures(&formula).unwrap();
        let a = &formula[captures.get(1).unwrap().range()];
        let b = &formula[captures.get(2).unwrap().range()];
        let a = match Self::parse_coordinate(a) {
            Some((column, row)) => self.cells[column][row],
            None => a.parse().unwrap(),
        };
        let b = match Self::parse_coordinate(b) {
            Some((column, row)) => self.cells[column][row],
            None => b.parse().unwrap(),
        };
        a + b
    }

    fn set_cell_at(&mut self, column: usize, row: usize, value: i32) {
        self.cells[column][row] = value;
    }

    fn parse_coordinate(coordinate: &str) -> Option<(usize, usize)> {
        let chars: Vec<char> = coordinate.chars().collect();
        if !chars[0].is_ascii_uppercase() {
            return None;
        }
        let column = (chars[0] as u8 - b'A') as usize;
        let row = coordinate[1..].parse::<usize>().unwrap() - 1;
        Some((column, row))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_example_1() {
        let mut spreedsheet = Spreadsheet::new(3);
        assert_eq!(12, spreedsheet.get_value(String::from("=5+7")));
        spreedsheet.set_cell(String::from("A1"), 10);
        assert_eq!(16, spreedsheet.get_value(String::from("=A1+6")));
        spreedsheet.set_cell(String::from("B2"), 15);
        assert_eq!(25, spreedsheet.get_value(String::from("=A1+B2")));
        spreedsheet.reset_cell(String::from("A1"));
        spreedsheet.get_value(String::from("=A1+B2"));
    }

    #[test]
    fn test_solution_case_1() {
        let mut spreedsheet = Spreadsheet::new(24);
        spreedsheet.set_cell(String::from("B24"), 66688);
        spreedsheet.reset_cell(String::from("O15"));
    }
}
