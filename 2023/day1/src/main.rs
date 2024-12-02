use std::fs::read_to_string;

fn main() {
    let mut counter = 0;
    let mut sum = 0;

    for line in read_to_string("file.txt").unwrap().lines() {
	// Part 1
	let digits = line.chars()
	    .filter_map(|c| c.to_digit(10))
	    .collect::<Vec<u32>>();

        counter += digits.first().unwrap() * 10 + digits.last().unwrap();

	// Part 2
	//let d = extract_all_digits_from_line(line);
	//sum += d.first().unwrap() * 10;
	//sum += d.last().unwrap();
    }

    println!("The sum of all numbers for part 1 is {}", counter);
    println!("The sum of all numbers for part 2 is {}", sum);
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
