use std::{collections::{HashMap}, fs::File};
use std::io::{self, BufRead};
use std::path::Path;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



fn main() {
    if let Ok(lines) = read_lines("./inputs/input_day10.txt") {
        let mut nums: Vec<u64> = lines.into_iter().map(|e| e.unwrap().parse::<u64>().unwrap()).collect();
        nums.insert(0, 0);
        nums.sort();
        nums.push(nums.last().unwrap() + 3);

        let mut diff_count: HashMap<u64, usize> = HashMap::new();
        let mut iter_nums = nums.into_iter().peekable();
        while let Some(num) = iter_nums.next() {
            let next = iter_nums.peek();
            match next {
                None => {}
                Some(&next) => {
                    let diff = next - num;
                    if let Some(count) = diff_count.get_mut(&diff) {
                        *count += 1;
                    }
                    else {
                        diff_count.insert(diff, 1);
                    }
                }
            }
        }

        let c1 = diff_count.get(&1).unwrap_or(&0);
        let c3 = diff_count.get(&3).unwrap_or(&0);
        println!("Diffs: {:?}, product: {} * {} = {}", diff_count, c1, c3, c1 * c3);
    }
}
