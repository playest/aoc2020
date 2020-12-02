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
                let policy_letter = policy_parts[1].chars().nth(0).unwrap();
                let policy_nums: Vec<&str> = policy_parts[0].split("-").collect();
                let policy_min_times: usize = policy_nums[0].parse::<usize>().unwrap() - 1;
                let policy_max_times: usize = policy_nums[1].parse::<usize>().unwrap() - 1;

                //println!("{} at [{}-{}] times in {}", policy_letter, policy_min_times, policy_max_times, password);

                let pattern = String::from("                                    ");
                let mut pattern_bytes = pattern.into_bytes();
                pattern_bytes[policy_min_times] = '*' as u8;
                pattern_bytes[policy_max_times] = '*' as u8;
                let pattern = String::from_utf8(pattern_bytes).unwrap();

                let c1 = password.chars().nth(policy_min_times);
                let c2 = password.chars().nth(policy_max_times);
                if  c1.ne(&c2) && (
                    c1.map_or_else(|| false, |c| c.eq(&policy_letter)) ||
                    c2.map_or_else(|| false, |c| c.eq(&policy_letter))
                ) {
                    println!("+ {:20} valid   {} at pos [{:>2}-{:>2}]({}, {}) len={:>2}\n{:20}", password, policy_letter, policy_min_times, policy_max_times, c1.unwrap_or('?'), c2.unwrap_or('?'), password.len(), pattern);
                    valid_count += 1;
                }
                else {
                    println!("- {:20} invalid {} at pos [{:>2}-{:>2}]({}, {}) len={:>2}\n{:20}", password, policy_letter, policy_min_times, policy_max_times, c1.unwrap_or('?'), c2.unwrap_or('?'), password.len(), pattern);
                }
            }
        }

        println!("valid count: {}", valid_count);
    }
}
