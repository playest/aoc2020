use std::{collections::VecDeque, fs::File};
use std::io::{self, BufRead};
use std::path::Path;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



fn main() {
    if let Ok(lines) = read_lines("./inputs/input_day9.txt") {
        let window_size = 5;
        let mut nums = lines.into_iter().map(|e| e.unwrap().parse::<i32>().unwrap());
        //let nums = nums.into_iter();
        let mut queue: VecDeque<i32> = VecDeque::with_capacity(window_size);
        let firsts = nums.by_ref().take(window_size);
        queue.extend(firsts.into_iter());

        println!("Preamble: {:?}", queue);

        for num in nums {
            //println!("{}", num);
            for (i1, n1) in queue.clone().iter().enumerate() {
                for (i2, n2) in queue.clone().iter().enumerate() {
                    println!("{} + {} = {}", n1, n2, n1 + n2);
                }
            }
        }
    }
}
