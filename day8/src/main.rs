use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct City {
    antennas: Vec<Vec<char>>,
    antinodes: Vec<Vec<char>>,
}

impl City {
    fn xlen(&self) -> i32 {
        self.antennas.len() as i32
    }
    fn ylen(&self) -> i32 {
        self.antennas[0].len() as i32
    }
    fn new() -> Self {
        Self {
            antennas: Vec::new(),
            antinodes: Vec::new(),
        }
    }
    fn add_line(&mut self, line: Vec<char>) {
        self.antennas.push(line.clone());
        let mut emptyline: Vec<char> = Vec::new();
        for i in 0..line.len() {
            emptyline.push('.');
        }
        self.antinodes.push(emptyline);
    }
    fn list_antennas(&self) -> Vec<Antenna> {
        let mut antennas: Vec<Antenna> = Vec::new();
        for i in 0..self.xlen() {
            for j in 0..self.xlen() {
                if self.antennas[i as usize][j as usize] != '.' {
                    antennas.push(Antenna {
                        x: i,
                        y: j,
                        frequency: self.antennas[i as usize][j as usize],
                    });
                }
            }
        }
        return antennas;
    }

    /*
        fn reset(&mut self) {
            self.left = self.backup.clone();
            self.up = self.backup.clone();
            self.right = self.backup.clone();
            self.down = self.backup.clone();
            self.path = self.backup.clone();
            self.blockers = self.backup.clone();
            self.initial = self.backup.clone();
        }
    */

    fn print(&self) {
        for i in 0..self.xlen() {
            for j in 0..self.ylen() {
                let mut c: char = self.antinodes[i as usize][j as usize];
                if self.antennas[i as usize][j as usize] != '.' {
                    c = self.antennas[i as usize][j as usize];
                }
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

#[derive(Debug)]
struct Antenna {
    frequency: char,
    x: i32,
    y: i32,
}

impl Antenna {
    fn distance_from(&self, antenna: Antenna) -> (i32, i32) {
        return (self.x - antenna.x, self.y - antenna.y);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut city: City = City::new();
    let mut sum: i32 = 0;
    // parse using filename
    if let Ok(lines) = read_lines(filename) {
        let mut line_nbr = 0;
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            city.add_line(line.chars().collect());
            line_nbr += 1;
        }
    }

    let mut sum2: i32 = 0;

    city.print();
    let mut i = city.list_antennas();
    for i in city.list_antennas() {
        dbg!("{}", i);
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
