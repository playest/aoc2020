use std::{fs::File};
use std::io::{self, BufRead};
use std::{path::Path};


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl Instruction {
    fn from_char(c: char, v: i32) -> Option<Instruction> {
        match c {
            'N' => Some(Instruction::North(v)),
            'S' => Some(Instruction::South(v)),
            'E' => Some(Instruction::East(v)),
            'W' => Some(Instruction::West(v)),
            'L' => Some(Instruction::Left(v)),
            'R' => Some(Instruction::Right(v)),
            'F' => Some(Instruction::Forward(v)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn rotate_right(&self, deg: i32) -> Direction {
        if deg % 90 != 0 {
            panic!("Cannot turn of {} degrees, angle must be a multiple of 90", deg);
        }
        let mut step = (deg / 90) % 4;
        let mut dir = self.clone();
        while step != 0 {
            dir = match dir {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            };
            step -= 1;
        }
        dir

    }

    fn rotate_left(&self, deg: i32) -> Direction {
        self.rotate_right(-1 * deg)
    }

    fn to_instruction(&self, v: i32) -> Instruction {
        match self {
            Direction::North => Instruction::North(v),
            Direction::East => Instruction::East(v),
            Direction::South => Instruction::South(v),
            Direction::West => Instruction::West(v),
        }
    }
}

#[derive(Debug)]
struct Ship {
    facing: Direction,
    x: i32,
    y: i32,
}

impl Ship {
    fn new(starting_direction: Direction, x: i32, y: i32) -> Self {
        Ship {
            facing: starting_direction,
            x,
            y,
        }
    }

    fn take_order(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::North(v) => {
                //self.facing = Direction::North;
                self.y -= v;
            },
            Instruction::South(v) => {
                //self.facing = Direction::South;
                self.y += v;
            },
            Instruction::East(v) => {
                //self.facing = Direction::East;
                self.x += v;
            },
            Instruction::West(v) => {
                //self.facing = Direction::West;
                self.x -= v;
            },
            Instruction::Left(v) => {
                self.facing = self.facing.rotate_left(v);
            },
            Instruction::Right(v) => {
                self.facing = self.facing.rotate_right(v);
            },
            Instruction::Forward(v) => {
                self.take_order(self.facing.to_instruction(v));
            },
        };
    }
}

fn main() {
    if let Ok(lines) = read_lines("./inputs/input_day12.txt") {
        let mut instructions: Vec<Instruction> = Vec::new();
        for line in lines {
            if let Ok(line) = line {
                let order = line.chars().next().unwrap();
                let value = &line[1..];
                instructions.push(Instruction::from_char(order, value.parse::<i32>().unwrap()).unwrap())
            }
        }

        let mut ship = Ship::new(Direction::East, 0, 0);
        println!("ship: {:?}", ship);
        for ins in instructions {
            println!("\tinstruction: {:?}", ins);
            ship.take_order(ins);
            println!("ship: {:?}", ship);
        }
        println!("Manhattan distance: {}", ship.x.abs() + ship.y.abs());
    }
}
