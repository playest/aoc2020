use std::{collections::VecDeque, fs::File};
use std::io::{self, BufRead};
use std::path::Path;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



fn main() {
    if let Ok(lines) = read_lines("./inputs/input_day10.txt") {
        let nums: Vec<u64> = lines.into_iter().map(|e| e.unwrap().parse::<u64>().unwrap()).collect();
        let mut nums_iter = nums.iter();

        for &num in nums_iter {
            println!("{}", num);
        }
    }
}
