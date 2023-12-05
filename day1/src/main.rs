use regex::Regex;
use std::fs::read_to_string;

fn first_part() {
    let mut counter = 0;

    for line in read_to_string("file.txt").unwrap().lines() {
        let mut first = 0;
        let mut last = 0;

        for ch in line.chars() {
            match ch.is_digit(10) {
                true => {
                    if first == 0 {
                        first = ch.to_digit(10).unwrap();
                        continue;
                    };

                    if first != 0 {
                        last = ch.to_digit(10).unwrap();
                    }
                }
                false => continue,
            }
        }

        if last == 0 {
            counter += first * 11;
        } else {
            counter += first * 10 + last;
        };
    }

    println!("The sum of all numbers for part 1 is {}", counter);
}

fn match_word_to_digit(w: &str) -> u32 {
    match w {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    }
}

fn create_calibration_number(v: Vec<&str>) -> u32 {
    let first = v.first().unwrap();
    let last = v.last().unwrap();
    let mut calibration = 0;

    match first.parse::<u32>() {
        Ok(n) => calibration += n * 10,
        Err(_) => calibration += match_word_to_digit(&first) * 10,
    }

    match last.parse::<u32>() {
        Ok(n) => calibration += n,
        Err(_) => calibration += match_word_to_digit(&last),
    }

    calibration
}

fn second_part() {
    let re = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let mut sum = 0;

    for line in read_to_string("file.txt").unwrap().lines() {
        let matches: Vec<_> = re.find_iter(line).map(|m| m.as_str()).collect();
        sum += create_calibration_number(matches);
    }

    println!("The sum of all numbers for part 2 is {}", sum);
}

#[cfg(test)]
mod tests {
    use crate::create_calibration_number;
    use regex::Regex;

    #[test]
    fn test_regex_1() {
        let re = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
        let haystack = "qbsixfour6six89pqxspnr8";
        let matches: Vec<_> = re.find_iter(haystack).map(|m| m.as_str()).collect();
        assert_eq!(matches, vec!["six", "four", "6", "six", "8", "9", "8"]);
        assert_eq!(create_calibration_number(matches), 68);
    }

    #[test]
    fn test_regex_2() {
        let re = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
        let haystack = "df1df2two1";
        let matches: Vec<_> = re.find_iter(haystack).map(|m| m.as_str()).collect();
        assert_eq!(matches, vec!["1", "2", "two", "1"]);
        assert_eq!(create_calibration_number(matches), 11);
    }

    #[test]
    fn test_regex_3() {
        let re = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
        let haystack = "1hellotwo";
        let matches: Vec<_> = re.find_iter(haystack).map(|m| m.as_str()).collect();
        assert_eq!(matches, vec!["1", "two"]);
        assert_eq!(create_calibration_number(matches), 12);
    }

    #[test]
    fn test_regex_4() {
        let re = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
        let haystack = "asdhaiuyasdabdntwo";
        let matches: Vec<_> = re.find_iter(haystack).map(|m| m.as_str()).collect();
        assert_eq!(matches, vec!["two"]);
        assert_eq!(create_calibration_number(matches), 22);
    }

    #[test]
    fn test_regex_5() {
        let re = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
        let haystack = "asdhaiu6asdabdntdo";
        let matches: Vec<_> = re.find_iter(haystack).map(|m| m.as_str()).collect();
        assert_eq!(matches, vec!["6"]);
        assert_eq!(create_calibration_number(matches), 66);
    }
}

fn main() {
    first_part();
    second_part();
}
