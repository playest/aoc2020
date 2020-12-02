use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("./inputs/input_day2.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut valid_count = 0;
        for line in lines {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split(":").collect();
                let policy = parts[0];
                let password = parts[1].trim();

                let policy_parts: Vec<&str> = policy.split(" ").collect();
                let policy_letter = policy_parts[1];
                let policy_nums: Vec<&str> = policy_parts[0].split("-").collect();
                let policy_min_times: u32 = policy_nums[0].parse::<u32>().unwrap();
                let policy_max_times: u32 = policy_nums[1].parse::<u32>().unwrap();

                //println!("{} at [{}-{}] times in {}", policy_letter, policy_min_times, policy_max_times, password);

                let letter_count = password.matches(policy_letter).count() as u32;
                if letter_count >= policy_min_times && letter_count <= policy_max_times {
                    println!("{}", password);
                    valid_count += 1;
                }
            }
        }

        println!("valid count: {}", valid_count);
    }
}
