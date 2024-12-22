use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct Map {
    map: Vec<Vec<char>>,
}

impl Map {
    fn xlen(&self) -> i32 {
        self.map.len() as i32
    }
    fn ylen(&self) -> i32 {
        self.map[0].len() as i32
    }
    fn new() -> Self {
        Self {
            map: Vec::new(),
        }
    }
    fn add_line(&mut self, line: Vec<char>) {
        self.map.push(line.clone());
    }

    fn print(&self) {
        for i in 0..self.xlen() {
            for j in 0..self.ylen() {
                let c: char = self.map[i as usize][j as usize];
                print!("{}", c);
            }
            println!();
        }
        println!();
    }

    fn is_inside(&self, point: &Coordinate) -> bool {
        point.x >= 0 && point.y >= 0 && point.x < self.xlen() && point.y < self.ylen()
    }

    fn value(&self, point: &Coordinate) -> Option<char> {
        if !self.is_inside(&point) {
            return None;
        }
        return Some(self.map[point.x as usize][point.y as usize]);
    }

}

#[derive(Debug, Clone, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}
impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Coordinate {}

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }
    fn right(&self) -> Coordinate {
        return Coordinate {
            x: self.x,
            y: self.y + 1,
        };
    }
    fn left(&self) -> Coordinate {
        return Coordinate {
            x: self.x,
            y: self.y - 1,
        };
    }
    fn down(&self) -> Coordinate {
        return Coordinate {
            x: self.x + 1,
            y: self.y,
        };
    }
    fn up(&self) -> Coordinate {
        return Coordinate {
            x: self.x - 1,
            y: self.y,
        };
    }
    fn neighbours(&self) -> Vec<Coordinate> {
        vec![self.up(), self.down(), self.right(), self.left()]
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut map: Map = Map::new();
    let mut direction: Vec<char>= Vec::new();
    let mut direction_turn = false;
    let mut sum: i32 = 0;
    let mut sum2: i32 = 0;
    let mut robot: Coordinate = Coordinate::new(0,0);
    // parse using filename
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            if !direction_turn {
                if line.len() == 0 {
                    direction_turn = true;
                } else {
                    map.add_line(line.chars().collect());
                }
            } else {
                direction.extend::<Vec<char>>(line.chars().collect());
            }
        }
    }
    for i in 0..map.ylen() {
        for j in 0..map.xlen() {
            let ici = Coordinate{x: i, y: j };
            if map.value(&ici).unwrap() == '@' {
                robot = Coordinate{ x: i, y: j};
            }
        }
    }

    map.print();
    dbg!("robot: {}",robot);

    println!("Sum is :{}", sum);
    println!("Sum2 is :{}", sum2);
}

//number of X
fn number_of_x(matrix: Vec<Vec<char>>, x: char) -> i32 {
    let mut sum: i32 = 0;
    for line in matrix.iter() {
        for col in line.iter() {
            if *col == x {
                sum += 1
            }
        }
    }
    return sum;
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn pause() {
    dbg!("Pausing! Press enter to continue...");

    let mut buffer = String::new();

    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
}
