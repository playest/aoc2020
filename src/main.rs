use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("./inputs/input1.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut expenses: Vec<i32> = Vec::new();
        for line in lines {
            if let Ok(expense) = line {
                let expense = expense.parse::<i32>();
                if let Ok(expense) = expense {
                    println!("{}", expense);
                    expenses.push(expense);
                }
            }
        }

        for e1 in &expenses {
            for e2 in &expenses {
                for e3 in &expenses {
                    let sum = e1 + e2 + e3;
                    if sum == 2020 {
                        println!("{} + {} + {} = {} => {}", e1, e2, e3, sum, e1*e2*e3);
                    }
                }
            }
        }
    }
}
