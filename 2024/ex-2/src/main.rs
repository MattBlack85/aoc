use std::fs::read_to_string;

fn read_file() -> String {
    read_to_string("file.txt").unwrap()
}

fn is_report_safe(line: &[u32]) -> bool {
    let mut asc = false;
    let mut desc = false;

    // Iterate for the next 2 elements of the slice
    for couple in line.windows(2) {
        if couple[0] < couple[1] {
            asc = true;
	} else if couple[0] > couple[1] {
	    desc = true;
	} else {
            return false;
	}

        if couple[0].abs_diff(couple[1]) > 3 {
            return false;
        }
    }

    if asc && desc {
        return false;
    }

    true
}

fn first_part() -> u32 {
    let mut safe_reports: u32 = 0;

    let lines: Vec<Vec<u32>> = read_file().lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    for line in &lines {
	if is_report_safe(&line) {
	    safe_reports += 1;
	}
    }
    

    safe_reports
}

fn second_part() -> u32 {
    let mut safe_reports: u32 = 0;

    let lines: Vec<Vec<u32>> = read_file().lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
	.collect();

    for line in &lines {
	if is_report_safe(&line) {
            safe_reports += 1;
        } else {
	    for i in 0..line.len() {
		let (left, right) = line.split_at(i);
		let slice = [left, &right[1..]].concat();
		if is_report_safe(&slice) {
		    safe_reports += 1;
                    break;
		}
	    }
        }
    }


    safe_reports
}

fn main() {
    println!("First part result: {}", first_part());
    println!("Second part result: {}", second_part());
}
