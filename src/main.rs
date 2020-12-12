use std::{fs::File};
use std::io::{self, BufRead};
use std::{path::Path};


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Clone, Copy)]
enum AbsoluteInstruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
}

#[derive(Debug)]
enum RelativeInstruction {
    Forward(i32),
}

#[derive(Debug)]
enum Instruction {
    Abs(AbsoluteInstruction),
    Rel(RelativeInstruction),
}

impl Instruction {
    fn from_char(c: char, v: i32) -> Result<Instruction, String> {
        match c {
            'N' => Ok(Instruction::Abs(AbsoluteInstruction::North(v))),
            'S' => Ok(Instruction::Abs(AbsoluteInstruction::South(v))),
            'E' => Ok(Instruction::Abs(AbsoluteInstruction::East(v))),
            'W' => Ok(Instruction::Abs(AbsoluteInstruction::West(v))),
            'L' => Ok(Instruction::Abs(AbsoluteInstruction::Left(v))),
            'R' => Ok(Instruction::Abs(AbsoluteInstruction::Right(v))),
            'F' => Ok(Instruction::Rel(RelativeInstruction::Forward(v))),
            _ => Err(format!("{} is not a valid char for an Instruction", c).to_string()),
        }
    }
}

trait OrderTaker<T> {
    fn take_order(&mut self, instruction: T);
}

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
        }
    }

    fn rotate_right(&mut self, deg: i32) {
        if deg % 90 != 0 {
            panic!("Cannot turn of {} degrees, angle must be a multiple of 90", deg);
        }
        let mut step = (deg / 90) % 4;
        //println!("\tangle of {} degrees is {} steps right", deg, step);
        if step < 0 {
            step += 4;
        }
        let (x, y) = (self.x, self.y);
        let (new_x, new_y) = match step {
            0 => ( x,  y),
            1 => (-y,  x),
            2 => (-x, -y),
            3 => ( y, -x),
            _ => panic!("Should not get a step outside [0, 3] for rotate: {}° -> {}", deg, step),
        };
        self.x = new_x;
        self.y = new_y;
    }

    fn rotate_left(&mut self, deg: i32) {
        self.rotate_right(-1 * deg);
    }

    fn forward(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    fn take_order(&mut self, instruction: AbsoluteInstruction) {
        match instruction {
            AbsoluteInstruction::North(v) => {
                self.y -= v;
            },
            AbsoluteInstruction::South(v) => {
                self.y += v;
            },
            AbsoluteInstruction::East(v) => {
                self.x += v;
            },
            AbsoluteInstruction::West(v) => {
                self.x -= v;
            },
            AbsoluteInstruction::Left(v) => {
                self.rotate_left(v);
            },
            AbsoluteInstruction::Right(v) => {
                self.rotate_right(v);
            },
        };
    }
}

#[derive(Debug)]
struct Ship {
    location: Point,
    waypoint: Point,
}

impl Ship {
    fn new(x: i32, y: i32, waypoint_x: i32, waypoint_y: i32) -> Self {
        Self {
            location: Point::new(x, y),
            waypoint: Point::new(waypoint_x, waypoint_y),
        }
    }

    fn take_order(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Abs(abs_ins) => {
                self.waypoint.take_order(abs_ins);
            },
            Instruction::Rel(RelativeInstruction::Forward(v)) => {
                let mut i = v;
                while i > 0 {
                    //println!("move a#{}: {:?} / {:?}", i, self.location, self.waypoint);
                    //self.location.take_order(instruction, &self.waypoint);
                    //println!("move c#{}: {:?} / {:?}", i, self.location, self.waypoint);
                    self.location.forward(self.waypoint.x, self.waypoint.y);
                    i -= 1;
                }
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

        let ship_x = 0;
        let ship_y = 0;
        let mut ship = Ship::new(ship_x, ship_y, ship_x + 10, ship_y - 1);
        println!("ship: {:?}", ship);
        for ins in instructions {
            println!("\tinstruction: {:?}", ins);
            ship.take_order(ins);
            println!("ship: {:?}", ship);
        }
        println!("Manhattan distance: {}", ship.location.x.abs() + ship.location.y.abs());
    }
}
