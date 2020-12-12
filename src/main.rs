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
    fn to_instruction(&self, v: i32) -> Instruction {
        match self {
            Direction::North => Instruction::North(v),
            Direction::East => Instruction::East(v),
            Direction::South => Instruction::South(v),
            Direction::West => Instruction::West(v),
        }
    }
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
        println!("\tangle of {} degrees is {} steps right", deg, step);
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

    fn take_order(&mut self, instruction: Instruction, relative_to: &Point) {
        match instruction {
            Instruction::North(v) => {
                self.y -= v;
            },
            Instruction::South(v) => {
                self.y += v;
            },
            Instruction::East(v) => {
                self.x += v;
            },
            Instruction::West(v) => {
                self.x -= v;
            },
            Instruction::Left(v) => {
                self.rotate_left(v);
            },
            Instruction::Right(v) => {
                self.rotate_right(v);
            },
            Instruction::Forward(v) => {
                self.x += relative_to.x;
                self.y += relative_to.y;
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
            Instruction::North(v) |
            Instruction::South(v) |
            Instruction::East(v) |
            Instruction::West(v) |
            Instruction::Left(v) |
            Instruction::Right(v) => {
                self.waypoint.take_order(instruction, &self.location);
            },
            Instruction::Forward(v) => {
                let mut i = v;
                while i > 0 {
                    //println!("move a#{}: {:?} / {:?}", i, self.location, self.waypoint);
                    self.location.take_order(instruction, &self.waypoint);
                    //println!("move c#{}: {:?} / {:?}", i, self.location, self.waypoint);
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
