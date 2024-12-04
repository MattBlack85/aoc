use std::fs::read_to_string;

fn read_file() -> String {
    read_to_string("file.txt").unwrap()
}

#[derive(Debug)]
enum State {
    Idle,
    SearchActionDo,
    SearchActionDoNext,
    SearchActionDoFinalize,
    SearchActionDont,
    SearchActionDontFinalize,
    SearchingM,
    SearchingU,
    SearchingL,
    FParenthesis,
    FNumFirst,
    FNum,
    SNumFirst,
    SNum,
    End,
    Done,
}

struct StateMachine {
    state: State,
    idx: usize,
    nums: u32,
    f_start: usize,
    f_end: usize,
    s_start: usize,
    s_end: usize,
    skip: bool,
    ignore_dos: bool,
}

impl StateMachine {
    fn new(ignore: bool) -> Self {
        StateMachine {
            state: State::Idle,
            idx: 0,
            nums: 0,
            f_start: 0,
            f_end: 0,
            s_start: 0,
            s_end: 0,
            skip: false,
            ignore_dos: ignore,
        }
    }
    fn process(&mut self, text: &Vec<char>) -> u32 {
        loop {
            if self.idx >= text.len() {
                self.state = State::Done;
            }
            let actual_char = text.get(self.idx).unwrap_or_else(|| &'0');

            self.state = match self.state {
                State::Idle => State::SearchingM,
                State::SearchActionDo => match actual_char {
                    'o' => State::SearchActionDoNext,
                    _ => State::SearchingM,
                },
                State::SearchActionDoNext => match actual_char {
                    '(' => State::SearchActionDoFinalize,
                    'n' => State::SearchActionDont,
                    _ => State::SearchingM,
                },
                State::SearchActionDoFinalize => match actual_char {
                    ')' => {
                        self.skip = false;
                        State::SearchingM
                    }
                    _ => State::SearchingM,
                },
                State::SearchActionDont => match actual_char {
                    '\'' => State::SearchActionDontFinalize,
                    _ => State::SearchingM,
                },
                State::SearchActionDontFinalize => match actual_char {
                    't' => {
                        self.skip = true;
                        State::SearchingM
                    }
                    _ => State::SearchingM,
                },
                State::SearchingM => match actual_char {
                    'm' => State::SearchingU,
                    'd' => State::SearchActionDo,
                    _ => State::SearchingM,
                },
                State::SearchingU => match actual_char {
                    'u' => State::SearchingL,
                    _ => State::SearchingM,
                },
                State::SearchingL => match actual_char {
                    'l' => State::FParenthesis,
                    _ => State::SearchingM,
                },
                State::FParenthesis => match actual_char {
                    '(' => State::FNumFirst,
                    _ => State::SearchingM,
                },
                State::FNumFirst => match actual_char {
                    '0'..='9' => {
                        self.f_start = self.idx;
                        State::FNum
                    }
                    _ => {
                        self.f_start = 0;
                        State::SearchingM
                    }
                },
                State::FNum => match actual_char {
                    '0'..='9' => State::FNum,
                    ',' => {
                        self.f_end = self.idx - 1;
                        State::SNumFirst
                    }
                    _ => {
                        self.f_start = 0;
                        State::SearchingM
                    }
                },
                State::SNumFirst => match actual_char {
                    '0'..='9' => {
                        self.s_start = self.idx;
                        State::SNum
                    }
                    _ => {
                        self.f_start = 0;
                        self.f_end = 0;
                        self.s_start = 0;
                        State::SearchingM
                    }
                },
                State::SNum => match actual_char {
                    '0'..='9' => State::SNum,
                    ')' => {
                        self.s_end = self.idx - 1;
                        State::End
                    }
                    _ => {
                        self.f_start = 0;
                        self.f_end = 0;
                        self.s_start = 0;
                        State::SearchingM
                    }
                },
                State::End => {
                    if self.ignore_dos || !self.skip {
                        let mut f: String = String::new();
                        for c in &text[self.f_start..self.f_end + 1] {
                            f.push(*c);
                        }
                        let mut s: String = String::new();
                        for c in &text[self.s_start..self.s_end + 1] {
                            s.push(*c);
                        }
                        self.nums += f.parse::<u32>().unwrap() * s.parse::<u32>().unwrap();
                    }
                    self.f_start = 0;
                    self.f_end = 0;
                    self.s_start = 0;
                    self.s_end = 0;
                    self.idx -= 1;
                    State::SearchingM
                }
                State::Done => break,
            };
            self.idx += 1;
        }
        self.nums
    }
}

fn first_part() -> u32 {
    let text: Vec<char> = read_file().chars().collect();

    let mut machine = StateMachine::new(true);
    machine.process(&text)
}

fn second_part() -> u32 {
    let text: Vec<char> = read_file().chars().collect();

    let mut machine = StateMachine::new(false);
    machine.process(&text)
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
        let s =
            String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
                .chars()
                .collect();
        let mut machine = StateMachine::new(true);
        let res = machine.process(&s);
        assert_eq!(res, 161);
    }

    #[test]
    fn test_input_part2() {
        let s = String::from(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        )
        .chars()
        .collect();
        let mut machine = StateMachine::new(false);
        let res = machine.process(&s);
        assert_eq!(res, 48);
    }
}
