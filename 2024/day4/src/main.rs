use std::fs::read_to_string;

fn read_file() -> String {
    read_to_string("file.txt").unwrap()
}

struct MachinePart2 {
    input: Vec<Vec<char>>,
    count: u32,
    row_idx: usize,
}

impl MachinePart2 {
    fn new(input: String) -> Self {
        Self {
            input: input
                .split("\n")
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|l| l.chars().collect::<Vec<char>>())
                .collect(),
            count: 0,
            row_idx: 0,
        }
    }

    fn extract_data(&self, row: usize, column: usize) -> Option<char> {
        if let Some(v) = self.input.get(row) {
            if let Some(c) = v.get(column) {
                return Some(*c);
            }
        }

        None
    }

    fn find_xmas(&self, row: usize, column: usize) -> u32 {
        let mut count = 0;

        // Down left
        let dl = if column >= 1 {
            self.extract_data(row + 1, column - 1)
        } else {
            None
        };
        // Down right
        let dr = self.extract_data(row + 1, column + 1);
        // Up right
        let upr = if row >= 1 {
            self.extract_data(row - 1, column + 1)
        } else {
            None
        };
        // Up left
        let upl = if row >= 1 && column >= 1 {
            self.extract_data(row - 1, column - 1)
        } else {
            None
        };

        if (upl == Some('M') && dl == Some('M')) && (upr == Some('S') && dr == Some('S'))
            || (upl == Some('S') && upr == Some('S')) && (dl == Some('M') && dr == Some('M'))
            || (upl == Some('M') && upr == Some('M')) && (dl == Some('S') && dr == Some('S'))
            || (upl == Some('S') && dl == Some('S')) && (upr == Some('M') && dr == Some('M'))
        {
            count += 1
        }

        count
    }

    fn process(&mut self) -> u32 {
        // All rows are the same length
        for row in self.input.clone().into_iter() {
            // Scan the whole row char by char, if X is met try to find
            // other letters around
            for c_idx in 0..row.len() {
                let ch = *row.get(c_idx).unwrap();
                if ch == 'A' {
                    self.count += self.find_xmas(self.row_idx, c_idx);
                }
            }
            self.row_idx += 1;
        }

        self.count
    }
}

struct Machine {
    input: Vec<Vec<char>>,
    count: u32,
    row_idx: usize,
}

impl Machine {
    fn new(input: String) -> Self {
        Self {
            input: input
                .split("\n")
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|l| l.chars().collect::<Vec<char>>())
                .collect(),
            count: 0,
            row_idx: 0,
        }
    }

    fn extract_data(&self, row: usize, column: usize) -> Option<char> {
        if let Some(v) = self.input.get(row) {
            if let Some(c) = v.get(column) {
                return Some(*c);
            }
        }

        None
    }

    fn look_up(&self, row: usize, column: usize) -> bool {
        if row < 3 {
            return false;
        }

        if self.extract_data(row - 1, column) == Some('M') {
            if self.extract_data(row - 2, column) == Some('A') {
                if self.extract_data(row - 3, column) == Some('S') {
                    return true;
                }
            }
        }
        false
    }

    fn look_up_right(&self, row: usize, column: usize) -> bool {
        if row < 3 {
            return false;
        }

        if self.extract_data(row - 1, column + 1) == Some('M') {
            if self.extract_data(row - 2, column + 2) == Some('A') {
                if self.extract_data(row - 3, column + 3) == Some('S') {
                    return true;
                }
            }
        }
        false
    }

    fn look_up_left(&self, row: usize, column: usize) -> bool {
        if row < 3 || column < 3 {
            return false;
        }

        if self.extract_data(row - 1, column - 1) == Some('M') {
            if self.extract_data(row - 2, column - 2) == Some('A') {
                if self.extract_data(row - 3, column - 3) == Some('S') {
                    return true;
                }
            }
        }
        false
    }

    fn look_down(&self, row: usize, column: usize) -> bool {
        if self.extract_data(row + 1, column) == Some('M') {
            if self.extract_data(row + 2, column) == Some('A') {
                if self.extract_data(row + 3, column) == Some('S') {
                    return true;
                }
            }
        }
        false
    }

    fn look_right(&self, row: usize, column: usize) -> bool {
        if self.extract_data(row, column + 1) == Some('M') {
            if self.extract_data(row, column + 2) == Some('A') {
                if self.extract_data(row, column + 3) == Some('S') {
                    return true;
                }
            }
        }
        false
    }

    fn look_down_right(&self, row: usize, column: usize) -> bool {
        if self.extract_data(row + 1, column + 1) == Some('M') {
            if self.extract_data(row + 2, column + 2) == Some('A') {
                if self.extract_data(row + 3, column + 3) == Some('S') {
                    return true;
                }
            }
        }
        false
    }

    fn look_down_left(&self, row: usize, column: usize) -> bool {
        if column < 3 {
            return false;
        }

        if self.extract_data(row + 1, column - 1) == Some('M') {
            if self.extract_data(row + 2, column - 2) == Some('A') {
                if self.extract_data(row + 3, column - 3) == Some('S') {
                    return true;
                }
            }
        }
        false
    }

    fn look_left(&self, row: usize, column: usize) -> bool {
        if column < 3 {
            return false;
        }

        if self.extract_data(row, column - 1) == Some('M') {
            if self.extract_data(row, column - 2) == Some('A') {
                if self.extract_data(row, column - 3) == Some('S') {
                    return true;
                }
            }
        }
        false
    }

    fn find_xmas(&self, row: usize, column: usize) -> u32 {
        let mut count = 0;
        // Start by looking for a M around the given poisition, there 8 possible poistion around any given letters
        if self.look_up(row, column) {
            count += 1
        }

        if self.look_down(row, column) {
            count += 1
        }
        if self.look_right(row, column) {
            count += 1
        }
        if self.look_left(row, column) {
            count += 1
        }

        if self.look_up_right(row, column) {
            count += 1
        }

        if self.look_up_left(row, column) {
            count += 1
        }

        if self.look_down_right(row, column) {
            count += 1
        }

        if self.look_down_left(row, column) {
            count += 1
        }

        count
    }

    fn process(&mut self) -> u32 {
        // All rows are the same length
        let row_length = self.input.get(0).unwrap().len();
        for row in self.input.clone().into_iter() {
            // Scan the whole row char by char, if X is met try to find
            // other letters around
            for c_idx in 0..row.len() {
                let ch = *row.get(c_idx).unwrap();
                if ch == 'X' {
                    self.count += self.find_xmas(self.row_idx, c_idx);
                }
            }
            self.row_idx += 1;
        }

        self.count
    }
}

fn first_part() -> u32 {
    let input = read_file();
    let mut machine = Machine::new(input);
    machine.process()
}

fn second_part() -> u32 {
    let input = read_file();
    let mut machine = MachinePart2::new(input);
    machine.process()
}

fn main() {
    println!("First part result: {}", first_part());
    println!("Second part result: {}", second_part());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part1() {
        let s = String::from("MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX");
        let mut machine = Machine::new(s);
        let res = machine.process();
        assert_eq!(res, 18);
    }

    #[test]
    fn test_input_part2() {
        let s = String::from("MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX");
        let mut machine = MachinePart2::new(s);
        let res = machine.process();
        assert_eq!(res, 9);
    }
}
