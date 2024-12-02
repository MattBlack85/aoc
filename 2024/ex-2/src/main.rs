use std::fs::read_to_string;

fn read_file() -> String {
    read_to_string("file.txt").unwrap()
}

fn first_part() -> u32 {
    let mut safe_reports: u32 = 0;

    for line in read_file().lines() {
        let mut numbers: Vec<u32> = Vec::new();
        let parts: Vec<&str> = line.split(" ").collect();

        if parts.len() <= 2 {
            safe_reports += 1
        }

        for part in parts {
            numbers.push(part.parse::<u32>().unwrap());
        }

	let mut iter_num = numbers.into_iter();
	let mut previous = iter_num.next().unwrap();
	let mut actual = iter_num.next().unwrap();
	let mut fail = false;
	
	if actual < previous && previous - actual <= 3 {
	    for n in iter_num {
		if n > actual || (n < actual && actual - n > 3) || actual == n {
		    fail = true;
		    break;
		} else {
		    previous = actual;
		    actual = n;
		}
	    }

	    if !fail {
		safe_reports += 1
	    }
	    
	} else if actual > previous && actual - previous <= 3 {
	    for n in iter_num {
		if n < actual || (n > actual && n - actual > 3) || actual == n {
		    fail = true;
		    break;
		} else {
		    previous = actual;
		    actual = n;
		}
	    }

	    if !fail {
		safe_reports += 1
	    }
	} else {
	     fail = true;
	}
    }

    safe_reports
}

fn second_part() -> u32 {
    0
}

fn main() {
    println!("First part result: {}", first_part());
    println!("Second part result: {}", second_part());
}
