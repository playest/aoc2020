use std::{fs::File};
use std::io::{self, BufRead};
use std::{fmt::Display, path::Path};
use std::convert::TryFrom;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Position {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl Position {
    fn from_char(c: char) -> Position {
        match c {
            'L' => Position::EmptySeat,
            '#' => Position::OccupiedSeat,
            '.' => Position::Floor,
            _ => panic!("unknown char {}", c),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Position::EmptySeat => 'L',
            Position::OccupiedSeat => '#',
            Position::Floor => '.',
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, PartialEq)]
struct Room {
    positions: Vec<Vec<Position>>
}

impl Room {
    fn new(positions: Vec<Vec<Position>>) -> Self {
        Room { positions }
    }

    fn get(&self, x: i32, y: i32) -> Option<Position> {
        let x = usize::try_from(x).ok()?;
        let y = usize::try_from(y).ok()?;
        let line = self.positions.get(y)?;
        line.get(x).copied()
    }

    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut Position> {
        let x = usize::try_from(x).ok()?;
        let y = usize::try_from(y).ok()?;
        let line = self.positions.get_mut(y)?;
        line.get_mut(x)
    }

    fn adjacent1(&self, x: i32, y: i32) -> Vec<Position> {
        let mut adj: Vec<Position> = Vec::with_capacity(8);
        self.get(x - 1, y - 1).and_then(|e| { adj.push(e); Some(e) });
        self.get(x    , y - 1).and_then(|e| { adj.push(e); Some(e) });
        self.get(x + 1, y - 1).and_then(|e| { adj.push(e); Some(e) });
        self.get(x - 1, y    ).and_then(|e| { adj.push(e); Some(e) });
        self.get(x + 1, y    ).and_then(|e| { adj.push(e); Some(e) });
        self.get(x - 1, y + 1).and_then(|e| { adj.push(e); Some(e) });
        self.get(x    , y + 1).and_then(|e| { adj.push(e); Some(e) });
        self.get(x + 1, y + 1).and_then(|e| { adj.push(e); Some(e) });
        adj
    }

    fn adjacent(&self, x: i32, y: i32) -> Vec<Position> {
        let mut adj: Vec<Position> = Vec::with_capacity(8);
        let offsets = vec![
            (- 1, - 1),
            ( 0 , - 1),
            ( 1 , - 1),
            (- 1,  0 ),
            ( 1 ,  0 ),
            (- 1,  1 ),
            ( 0 ,  1 ),
            ( 1 ,  1 ),
        ];
        for (dx, dy) in offsets {
            println!("Look towards {}, {}", dx, dy);
            let mut x = x + dx;
            let mut y = y + dy;
            loop {
                let pos = self.get(x, y);
                println!("\tLook at {}, {} = {:?}", x, y, pos);
                match pos {
                    None => {
                        break; // we reached the border of the room
                    },
                    Some(Position::OccupiedSeat) => {
                        adj.push(Position::OccupiedSeat);
                        break;
                    },
                    Some(Position::EmptySeat) => {
                        adj.push(Position::OccupiedSeat);
                        break;
                    },
                    _ => {}
                }
                x += dx;
                y += dy;
            }
        }
        adj
    }

    fn step(&self) -> Self {
        let mut next_step = self.clone();
        for (y, line) in self.positions.iter().enumerate() {
            for (x, &pos) in line.iter().enumerate() {
                let adj_occupied_seats = self.adjacent(x as i32, y as i32).iter().filter(|&&e| e == Position::OccupiedSeat).count();
                //let adj_free_seats = self.adjacent(x as i32, y as i32).iter().filter(|&&e| e != Position::OccupiedSeat).count();
                //println!("{}, {} -> adj_occupied_seats: {}, adj_free_seats: {}", x, y, adj_occupied_seats, adj_free_seats);

                if pos == Position::EmptySeat && adj_occupied_seats == 0 {
                    //println!("Free seat!");
                    *next_step.get_mut(x as i32, y as i32).unwrap() = Position::OccupiedSeat;
                }
                else if pos == Position::OccupiedSeat && adj_occupied_seats >= 5 {
                    //println!("Leave seat!");
                    *next_step.get_mut(x as i32, y as i32).unwrap() = Position::EmptySeat;
                }
            }
        }
        next_step
    }
}

impl Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.positions {
            for pos in line {
                write!(f, " {} ", pos)?
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}

fn main() {
    if let Ok(lines) = read_lines("./inputs/input_day11.txt") {
        let items = lines.into_iter()
            .map(|e| e.unwrap().chars().into_iter()
                .map(|c| Position::from_char(c)).collect::<Vec<_>>()
        ).collect::<Vec<Vec<_>>>();

        let mut round = 0;
        let mut prev_room = Room::new(items);

        // tests
        let p = prev_room.get(3, 3);
        println!("origin: {:?}", p);
        let p = prev_room.adjacent(3, 3);
        println!("adj: {}, {:?}", p.len(), p);
        
        return;
        // run
        let mut new_room: Room;
        println!("=> round {}:\n{}", round, prev_room);
        loop {
            round += 1;
            new_room = prev_room.step();
            println!("=> round {}:\n{}", round, new_room);
            if new_room == prev_room {
                break;
            }
            prev_room = new_room;
            if round > 100 {
                break;
            }
        }

        println!("Stabilized after {} rounds:\n{}", round, prev_room);
        let occupied: usize = prev_room.positions
            .iter()
            .map(|l| l.iter()
                .filter(|&&p| p == Position::OccupiedSeat)
                .count()
        ).sum();
        println!("Occupied seats: {}", occupied);

    }
}
