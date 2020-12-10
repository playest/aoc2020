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
        let nums: Vec<u64> = lines.into_iter().map(|e| e.unwrap().parse::<u64>().unwrap()).collect();
        let mut nums_iter = nums.iter();
        let mut queue: VecDeque<u64> = VecDeque::with_capacity(window_size);
        let firsts = nums_iter.by_ref().take(window_size);
        queue.extend(firsts.into_iter());

        println!("Preamble: {:?}", queue);

        for &num in nums_iter {
            //println!("{}", num);
            let mut sol_n1 = None;
            let mut sol_n2 = None;
            let mut solution = None;
            let it1 = queue.iter();
            
            'search: for (i1, n1) in it1.enumerate() {
                let it2 = queue.iter();
                for (i2, n2) in it2.enumerate() {
                    let sum = n1 + n2;
                    if i1 != i2 && sum == num {
                        solution = Some(sum);
                        sol_n1 = Some(n1);
                        sol_n2 = Some(n2);
                        break 'search;
                    }
                }
            }

            if let Some(sum) = solution {
                //println!("{} is valid because = {} + {} = {} in {:?}", num, sol_n1.unwrap(), sol_n2.unwrap(), sum, queue);
            }
            else {
                println!("{} is invalid because no sum found in {:?}", num, queue);

                // try to find encryption weakness

                let mut nums = nums.iter();
                let mut sum_set: VecDeque<u64> =  VecDeque::new();
                let mut sum: u64 = 0;
                loop {
                    if let Some(&n) = nums.next() {                    
                        sum += n;
                        sum_set.push_back(n);
                        println!("Current sum is {} compared to {} in {:?}", sum, num, sum_set);
                        
                        while sum > num {
                            sum -= sum_set.pop_front().unwrap();
                        }
                        
                        if sum == num {
                            let mut sorted: Vec<u64> = Vec::new();
                            sorted.extend(sum_set.clone().into_iter());
                            sorted.sort();
                            let weakness = sorted.first().unwrap() + sorted.last().unwrap();
                            println!("Found weakness {} in {:?}", weakness, sum_set);
                            break;
                        }
                    }
                    else {
                        println!("Could not find weakness");
                        break;
                    }
                }

                return;
            }

            queue.push_back(num);
            queue.pop_front();
        }
    }
}
