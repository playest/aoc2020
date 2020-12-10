use std::{fs::File};
use std::io::{self, BufRead};
use std::path::Path;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn factorial(num: u64) -> u64 {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}

fn combination(n: u64, r: u64) -> u64 {
    factorial(n) / (factorial(r) * factorial(n - r))
}

fn main() {
    if let Ok(lines) = read_lines("./inputs/input_day10.txt") {
        let mut nums: Vec<u64> = lines.into_iter().map(|e| e.unwrap().parse::<u64>().unwrap()).collect();
        nums.insert(0, 0);
        nums.sort();
        nums.push(nums.last().unwrap() + 3);
        println!("{:?}", nums);

        let mut previous: Option<u64> = None;
        let mut iter_nums = nums.into_iter().peekable();
        let mut can_be_removed = 0;
        let mut arrangments = 1;
        while let Some(num) = iter_nums.next() {
            if let Some(previous) = previous {
                let next = iter_nums.peek();
                match next {
                    None => {}
                    Some(&next) => {
                        println!("{} _{}_ {}", previous, num, next);
                        let diff = next - previous;
                        if diff <= 3 {
                            can_be_removed += 1;
                            println!("\t{} could be removed, consecutive: {}", num, can_be_removed);
                        }
                        else if can_be_removed != 0 {
                            let new_arrangments = combination(can_be_removed, 0)
                                + if can_be_removed >= 1 { combination(can_be_removed, 1) } else { 0 }
                                + if can_be_removed >= 2 { combination(can_be_removed, 2) } else { 0 };
                            arrangments *= new_arrangments;
                            println!("end of remove chain, {} new arrangements for {} total arrangements", new_arrangments, arrangments);
                            can_be_removed = 0;
                        }
                    }
                }
            }
            previous = Some(num);
        }
    }
}
