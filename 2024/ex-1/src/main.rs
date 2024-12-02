use std::collections::HashMap;
use std::fs::read_to_string;

fn read_file() -> String {
    read_to_string("file.txt").unwrap()
}

fn first_part() -> u32 {
    let mut diff = 0;
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for line in read_file().lines() {
        let parts: Vec<&str> = line.split(" ").collect();

        left.push(parts.get(0).unwrap().parse::<u32>().unwrap());
        right.push(parts.get(3).unwrap().parse::<u32>().unwrap());
    }

    left.sort();
    right.sort();

    for idx in 0..left.len() {
        let a = left[idx];
        let b = right[idx];

        if a > b {
            diff += a - b;
        } else {
            diff += b - a;
        }
    }

    diff
}

fn second_part() -> u32 {
    let mut similarity = 0;
    let mut left: Vec<u32> = Vec::new();
    let mut right: HashMap<u32, u32> = HashMap::new();

    for line in read_file().lines() {
        let parts: Vec<&str> = line.split(" ").collect();

        left.push(parts.get(0).unwrap().parse::<u32>().unwrap());

        let n = parts.get(3).unwrap().parse::<u32>().unwrap();

        if right.contains_key(&n) {
            let update = right.get_mut(&n).unwrap();
            *update += 1;
        } else {
            right.insert(n, 1);
        }
    }

    for n in left.iter() {
        let nr = right.get(&n);

        if nr.is_some() {
            similarity += n * nr.unwrap();
        }
    }

    similarity
}

fn main() {
    println!("First part result: {}", first_part());
    println!("Second part result: {}", second_part());
}
