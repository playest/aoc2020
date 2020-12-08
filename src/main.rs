use std::{fs::File};
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::HashSet;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("./inputs/input_day6.txt") {
        let mut groups: Vec<HashSet<char>> = Vec::new();
        let mut group: HashSet<char> = HashSet::new();
        let mut first = true;
        for line in lines {
            if let Ok(line) = line {
                //println!("line: {}", line);
                if line.is_empty() {
                    println!("group set: {:?}", group);
                    groups.push(group);
                    group = HashSet::new();
                    first = true;
                    continue;
                }
                
                let mut person: HashSet<char> = HashSet::new();
                for char in line.chars() {
                    person.insert(char);
                }
                println!("\tperson {:?}", person);
                if first {
                    group = person;
                }
                else {
                    group = group.intersection(&person).cloned().collect();
                }
                first = false;
            }
        }
        println!("group set: {:?}", group);
        groups.push(group);


        let mut sum = 0;
        for group in groups {
            sum += group.len();
        }

        println!("sum: {}", sum);
    }
}
