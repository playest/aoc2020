use std::{fmt::Display, fs::File, fmt};
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

enum TerrainType {
    Open,
    Tree,
}

impl TerrainType {
    fn from_char(c: char) -> Self {
        match c {
            '.' => TerrainType::Open,
            '#' => TerrainType::Tree,
            _ => panic!("Unknown character for TerrainType: {}", c),
        }
    }
}

impl Display for TerrainType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TerrainType::Open => write!(f, "."),
            TerrainType::Tree => write!(f, "#"),
        }
    }
}

struct Terrain {
    kind: TerrainType,
    marked: bool,
}

impl Terrain {
    fn new(kind: TerrainType) -> Self {
        Self { kind, marked: false }
    }

    fn mark(&mut self) {
        self.marked = true;
    }
}

impl Display for Terrain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut symbols = String::new();
        match self.kind {
            TerrainType::Open => symbols.push_str(" ."),
            TerrainType::Tree => symbols.push_str(" #"),
        }
        match self.marked {
            true => symbols.push_str("x "),
            false => symbols.push_str("  "),
        }
        write!(f, "{}", symbols)
    }
}

struct Map {
    lines: Vec<Vec<Terrain>>,
}

impl Map {
    fn new() -> Self {
        Self {
            lines: Vec::new()
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<&Terrain> {
        let height = self.lines.len();
        if y >= height {
            None
        }
        else {
            let width = self.lines.first().unwrap().len();
            Some(&self.lines[y][x % width])
        }
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Terrain> {
        let height = self.lines.len();
        if y >= height {
            None
        }
        else {
            let width = self.lines.first().unwrap().len();
            let l = &mut self.lines[y];
            //println!("x: {}, y: {}, width: {}, height: {}", x, y, width, height);
            Some(&mut l[x % width])
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut y = 0;
        for line in &self.lines {
            write!(f, "{:>2}: ", y)?;
            for square in line {
                write!(f, "{}", square)?;
            }
            write!(f, "\n")?;
            y += 1;
        }
        Result::Ok(())
    }
}

trait Growing {
    fn add_terrain(&mut self, tt: TerrainType);
    fn new_line(&mut self);
}

impl Growing for Map {
    
    fn add_terrain(&mut self, tt: TerrainType) {
        let last = self.lines.last_mut();
        let last = match last {
            None => {
                self.lines.push(Vec::new());
                self.lines.last_mut().unwrap()
            },
            Some(v) => v
        };

        last.push(Terrain::new(tt));
    }

    fn new_line(&mut self) {
        self.lines.push(Vec::new());
    }
}

struct MapBrowser<'a> {
    map: &'a mut Map,
    x: i32,
    y: i32,
}

impl<'a> MapBrowser<'a> {
    fn new(map: &'a mut Map, x: i32, y: i32) -> Self {
        Self { map, x, y }
    }

    fn walk(&mut self, x_offset: i32, y_offset: i32) -> Option<&Terrain> {
        self.x += x_offset;
        self.y += y_offset;
        self.map.get(self.x as usize, self.y as usize)
    }

    fn walk_mut(&mut self, x_offset: i32, y_offset: i32) -> Option<&mut Terrain> {
        self.x += x_offset;
        self.y += y_offset;
        self.map.get_mut(self.x as usize, self.y as usize)
    }
}

fn main() {
    if let Ok(lines) = read_lines("./inputs/input_day3.txt") {
        let mut map = Map::new();
        for line in lines {
            if let Ok(line) = line {
                map.new_line();
                for c in line.chars() {
                    map.add_terrain(TerrainType::from_char(c));
                }
            }
        }
        println!("{}", map);


        let mut tree_counts: Vec<u64> = Vec::new();
        for (x_offset, y_offset) in vec!((1, 1), (3, 1), (5, 1), (7, 1), (1, 2)) {
            let mut map_browser = MapBrowser::new(&mut map, 0, 0);
            //let mut maybe_square = map_browser.walk_mut(1, 3);
            let mut again = true;
            let mut tree_count = 0;
            let mut open_count = 0;
            while again {
                let mut maybe_square = map_browser.walk_mut(x_offset, y_offset);
                match maybe_square {
                    None => again = false,
                    Some(square) => {
                        square.mark();
                        //println!("{}", square);
                        match square.kind {
                            TerrainType::Tree => tree_count += 1,
                            TerrainType::Open => open_count += 1,
                        }
                    },
                }
            }
            
            //println!("{}", map);
            println!("tree_count: {}, open_count: {}", tree_count, open_count);
            tree_counts.push(tree_count);
        }

        let mut product: u64 = 1;
        for tc in tree_counts {
            product *= tc;
        }

        println!("product: {}", product);
    }
}
