use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct Map {
    map: Vec<Vec<char>>,
    crop_map: Vec<Vec<i32>>,
    crop_list: HashMap<i32, Crop>,
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
            crop_map: Vec::new(),
            crop_list: HashMap::new(),
        }
    }
    fn add_line(&mut self, line: Vec<char>) {
        self.map.push(line.clone());
        let mut null_line: Vec<i32> = Vec::new();
        for i in 0..line.len() {
            null_line.push(0);
        }
        self.crop_map.push(null_line);
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
    fn print_crops(&self) {
        for i in 0..self.xlen() {
            for j in 0..self.ylen() {
                let c: i32 = self.crop_map[i as usize][j as usize];
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

    fn fence_nbr(&self, point: &Coordinate) -> i32 {
        let mut nbr: i32 = 0;
        let c = self.value(point).unwrap();
        for p in point.neighbours() {
            if !self.is_inside(&p) {
                nbr += 1;
            } else if let Some(e) = self.value(&p) {
                if e != c {
                    nbr += 1;
                }
            }
        }
        return nbr;
    }

    fn mark_crop(&mut self, parcel: Coordinate, crop_index: i32) {
        let plant = self.value(&parcel).unwrap();
        if self.is_inside(&parcel) {
            if self.crop_map[parcel.x as usize][parcel.y as usize] == crop_index {
                return;
            }
            self.crop_map[parcel.x as usize][parcel.y as usize] = crop_index;
            if let Some(crop) = self.crop_list.get_mut(&crop_index) {
                crop.parcels.push(parcel.clone());
            } else {
                self.crop_list.insert(
                    crop_index,
                    Crop {
                        parcels: vec![parcel.clone()],
                    },
                );
            }
        }
        let mut parcel_list: Vec<Coordinate> = Vec::new();
        parcel_list.push(parcel.left());
        parcel_list.push(parcel.up());
        parcel_list.push(parcel.right());
        parcel_list.push(parcel.down());
        for p in parcel_list {
            if let Some(c) = self.value(&p) {
                if c == plant {
                    self.mark_crop(p, crop_index);
                }
            }
        }
    }

    fn calculate_crops(&mut self) {
        for x in 0..self.xlen() {
            for y in 0..self.ylen() {
                let crop_index = self.crop_map[x as usize][y as usize];
                if crop_index == 0 {
                    let new_crop_index = self.crop_list.len() as i32 + 1;
                    let parcel = Coordinate::new(x as i32, y as i32);
                    self.mark_crop(parcel, new_crop_index);
                }
            }
        }
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

#[derive(Debug, Clone, Hash)]
struct Crop {
    parcels: Vec<Coordinate>,
}

impl Crop {
    fn area(&self) -> i32 {
        return self.parcels.len() as i32;
    }

    fn fence_nbr(&self, map: &Map) -> i32 {
        let mut nbr: i32 = 0;
        for parcel in &self.parcels {
            nbr += map.fence_nbr(&parcel);
        }
        return nbr;
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

    map.print();
    map.calculate_crops();
    map.print_crops();

    for (k, i) in &mut map.crop_list.iter() {
        let area = i.area();
        let fences = i.fence_nbr(&map);
        sum += area * fences;
        println!("crop {} is area {} and fence {}", k, area, fences);
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
