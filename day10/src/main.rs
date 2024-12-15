use std::collections::HashSet;
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
        Self { map: Vec::new() }
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

    fn height(&self, point: &Coordinate) -> i32 {
        //dbg!("{}", self.clone());
        if self.is_inside(&point) {
            
            return self.map[point.x as usize][point.y as usize]
                .to_string()
                .parse::<i32>()
                .unwrap();
        }
        return 0;
    }

    fn find_tops(&self, start: Coordinate, height: i32) -> Vec<Coordinate> {
        if !self.is_inside(&start) {
            return vec![];
        }
        if self.height(&start) != height {
            return vec![];
        }
        if height == 9 {
            return vec![start.clone()];
        }
        let new_height = height + 1;
        return [
            self.find_tops(start.left(), new_height),
            self.find_tops(start.right(), new_height),
            self.find_tops(start.up(), new_height),
            self.find_tops(start.down(), new_height),
        ]
        .concat();
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
}

#[derive(Debug, Clone)]
struct Trail {
    head: Coordinate,
    tops: HashSet<Coordinate>,
}

impl Trail {
    fn new(x: i32, y: i32) -> Self {
        Self {
            head: Coordinate { x: x, y: y },
            tops: HashSet::new(),
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut map: Map = Map::new();
    let mut sum: i32 = 0;
    let mut sum2: i32 = 0;
    // parse using filename
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            map.add_line(line.chars().collect());
        }
    }
    let mut trails: Vec<Trail> = Vec::new();
    for line in 0..map.xlen() {
        for col in 0..map.ylen() {
            if map.map[line as usize][col as usize] == '0' {
                let mut trail = Trail::new(line, col);
                for top in map.find_tops(trail.head.clone(), 0) {
                    trail.tops.insert(top);
                    sum2+=1;
                }
                trails.push(trail);
            }
        }
    }

    for i in trails {
        //dbg!("{}", i);
        sum += i.tops.len() as i32;
    }

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
