use std::{fs::File};
use std::io::{self, BufRead};
use std::path::Path;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("./inputs/input_day11.txt") {
        let mut items: Vec<_> = lines.into_iter().map(|e| e.unwrap().parse::<u64>().unwrap()).collect();
        println!("{:?}", items);

        let mut iter_items = items.into_iter();
        while let Some(item) = iter_items.next() {
            println!("{}", item);
        }
    }
}
