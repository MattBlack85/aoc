use std::fs::read_to_string;

fn check_limit(s: &str) -> u32 {
    match &s[0..1] {
        "b" => 14,
        "r" => 12,
        "g" => 13,
        _ => 0,
    }
}

fn main() {
    let mut sum = 0;
    let mut power = 0;

    for line in read_to_string("file.txt").unwrap().lines() {
        let v: Vec<&str> = line.split(":").collect();

        let mut valid = true;
        let mut min_r = 0;
        let mut min_g = 0;
        let mut min_b = 0;

        let results: Vec<&str> = v[1].split(";").collect();

        for single_res in results {
            let rr: Vec<&str> = single_res.split(",").collect();

            for r in rr {
                let rv: Vec<&str> = r.trim().split(" ").collect();
                let num = rv[0].parse::<u32>().unwrap();
                if num > check_limit(rv[1]) {
                    valid = false;
                };

                match rv[1] {
                    "red" => {
                        if num > min_r {
                            min_r = num;
                        };
                    }
                    "green" => {
                        if num > min_g {
                            min_g = num;
                        };
                    }
                    "blue" => {
                        if num > min_b {
                            min_b = num;
                        };
                    }
                    _ => (),
                }
            }
        }

        println!(
            "The min for this game are: r{} g{} b{}",
            min_r, min_g, min_b
        );

        if valid {
            sum += v[0].split(" ").collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .unwrap()
        }

        power += min_r * min_g * min_b
    }
    println!("The sum of valid games is {sum}");
    println!("The sum of powers of all games is {power}");
}
