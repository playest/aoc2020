use std::{fs::File};
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("./inputs/input_day5.txt") {
        let mut seat_ids: Vec<i32> = Vec::new();
        for line in lines {
            if let Ok(line) = line {
                println!("line: {}", line);
                let mut row_min = 0;
                let mut row_max = 128;
                let mut col_min = 0;
                let mut col_max = 8;

                for char in line.chars() {
                    if char == 'F' {
                        row_max -= (row_max - row_min) / 2;
                    }
                    else if char == 'B' {
                        row_min += (row_max - row_min) / 2;
                    }
                    else if char == 'L' {
                        col_max -= (col_max - col_min) / 2;
                    }
                    else if char == 'R' {
                        col_min += (col_max - col_min) / 2;
                    }
                    else {
                        println!("Unknown char: {}", char);
                    }
                    //println!("{} -> row: [{}, {}[, col: [{}, {}[", char, row_min, row_max, col_min, col_max);
                }
                //println!("");

                if row_max - row_min == 1 && col_max - col_min == 1 {
                    let row = row_min;
                    let col = col_min;
                    let seat_id = row * 8 + col;
                    seat_ids.push(seat_id);
                    println!("=> row {}, col {}, id {}", row, col, seat_id);
                }
                else {
                    println!("Not enough char to deduce col and row");
                }
                
            }
        }
        seat_ids.sort();
        println!("highest seat id: {:?}", seat_ids.last());

        let mut previous = seat_ids.first().unwrap();
        for sid in &seat_ids[1..] {
            if sid - previous != 1 {
                println!("seat {} was skipped", sid - 1);
            }
            previous = sid;
        }
    }
}
