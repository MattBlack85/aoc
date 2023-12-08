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

fn extract_all_digits_from_line(l: &str) -> Vec<u32> {
    let str_to_num = [
	("one", 1),
	("two", 2),
	("three", 3),
	("four", 4),
	("five", 5),
	("six", 6),
	("seven", 7),
	("eight", 8),
	("nine", 9),
    ];

    let mut s = l.to_string();
    str_to_num.iter().for_each(|(k, v)| s = s.replace(k, &format!("{}{}{}", k, v, k).to_string()));
    let digits = s.chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>();

    digits
}

fn second_part() {
    let mut sum = 0;

    for line in read_to_string("file.txt").unwrap().lines() {
        let digits = extract_all_digits_from_line(line);

	sum += digits.first().unwrap() * 10;
	sum += digits.last().unwrap()
    }

    println!("The sum of all numbers for part 2 is {}", sum);
}

fn main() {
    first_part();
    second_part();
}
