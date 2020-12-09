use std::{fs::File};
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Clone)]
enum Operation {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

#[derive(Debug, Clone)]
struct Executer {
    accumulator: i32,
    current_operation_index: usize,
    exec: Vec<i8>,
}

impl Executer {
    fn new() -> Self {
        Executer {
            accumulator: 0,
            current_operation_index: 0,
            exec: Vec::new(),
        }
    }

    fn step(&mut self, program: &Vec<Operation>) -> (bool, bool) {
        if self.current_operation_index >= self.exec.len() {
            self.exec.resize(self.current_operation_index + 1, 0);
        }

        let executed = self.exec.get_mut(self.current_operation_index).unwrap();
        let already_executed = *executed > 0;
        *executed = 1;

        let op = program.get(self.current_operation_index);
        //println!("op: {:?}", op);
        //println!("executer: {:?}", self);
        match op {
            Some(Operation::Nop(_)) => {},
            Some(Operation::Acc(v)) => self.accumulator += v,
            Some(Operation::Jmp(v)) => self.current_operation_index = (self.current_operation_index as i32 + v - 1) as usize,
            None => return (true, already_executed),
        }

        self.current_operation_index += 1;
        (false, already_executed)
    }

    fn would_terminate(program: &Vec<Operation>) -> bool {
        let mut cloned_executor = Executer::new();
        loop {
            let (finished, looping) = cloned_executor.step(program);
            if finished {
                return true;
            }
            if looping {
                return false;
            }
        }
    }
}

fn main() {
    if let Ok(lines) = read_lines("./inputs/input_day8.txt") {
        let mut program: Vec<Operation> = Vec::new();
        program.push(Operation::Nop(1));
        for line in lines {
            if let Ok(line) = line {
                let mut parts = line.split_ascii_whitespace();
                let op_string = parts.next();
                let op = match op_string {
                    Some("nop") => Operation::Nop(parts.next().unwrap().parse::<i32>().unwrap()),
                    Some("acc") => Operation::Acc(parts.next().unwrap().parse::<i32>().unwrap()),
                    Some("jmp") => Operation::Jmp(parts.next().unwrap().parse::<i32>().unwrap()),
                    _ => panic!("wrong instruction {}", line),
                };
                program.push(op);
            }
        }

        // trying to fix program
        let mut program2: Vec<Operation>;
        for (i, _) in program.iter().enumerate() {
            program2 = program.clone();
            let op = program2.get_mut(i).unwrap();
            match op {
                Operation::Nop(v) => {
                    *op = Operation::Jmp(*v);
                }
                Operation::Jmp(v) => {
                    *op = Operation::Nop(*v);
                }
                _ => {}
            }

            let finishes = Executer::would_terminate(&program2);
            if finishes {
                program = program2;
                break;
            }
        }

        // exec
        let mut executer = Executer::new();
        let mut previous_accumulator = executer.accumulator;
        loop {
            let (finished, looping) = executer.step(&program);
            if finished {
                println!("End of program. Accumulator: {}", executer.accumulator);
                break;
            }
            if looping {
                println!("Loop detected, stopping. Accumulator was: {}", previous_accumulator);
                break;
            }
            previous_accumulator = executer.accumulator;
        }
    }
}
