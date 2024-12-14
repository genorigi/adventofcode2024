use std::env;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
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

    fn is_inside(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.xlen() && y < self.ylen()
    }

}

#[derive(Debug,Clone)]
struct Coordinate {
    x: i32,
    y: i32
}
impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }   
}   



#[derive(Debug,Clone)]
struct Trail {
    head: Coordinate,
    tops: HashSet<Coordinate>,
    
}

impl Trail {
    fn new(x: i32, y: i32) -> Self {
        Self {
            head: Coordinate {
                x: x,
                y: y
            },
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
                let trail = Trail::new(line, col);
                trails.push(trail);
            }
        }
    }


    for i in trails {
        dbg!("{}", i);
    };

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
