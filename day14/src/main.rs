use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct Map {
    map: Vec<Vec<char>>,
    robots: Vec<Robot>,
}

impl Map {
    fn ylen(&self) -> i32 {
        self.map.len() as i32
    }
    fn xlen(&self) -> i32 {
        self.map[0].len() as i32
    }
    fn new(xsize: i32, ysize: i32) -> Self {
        let mut map: Vec<Vec<char>> = Vec::new();
        for i in 0..ysize {
            let mut line: Vec<char> = Vec::new();
            for j in 0..xsize {
                line.push('.');
            }
            map.push(line);
        }
        Self {
            map: map,
            robots: Vec::new(),
        }
    }
    fn add_line(&mut self, line: String) {
        let r_robot = Regex::new(r"^p=([0-9]+),([0-9]+) v=(-*[0-9]+),(-*[0-9]+)$").unwrap();
        let x = r_robot
            .captures(&line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        let y = r_robot
            .captures(&line)
            .unwrap()
            .get(2)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        let position = Coordinate::new(x, y);
        let speed_x = r_robot
            .captures(&line)
            .unwrap()
            .get(3)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        let speed_y = r_robot
            .captures(&line)
            .unwrap()
            .get(4)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        let movement = Movement::new(speed_x, speed_y);
        let robot = Robot {
            position: position,
            movement: movement,
        };
        self.robots.push(robot);
    }

    fn print(&self) {
        for i in 0..self.ylen() {
            for j in 0..self.xlen() {
                let mut c: char = self.map[i as usize][j as usize];
                for (n, v) in self.robots.iter().enumerate() {
                    if v.position.x == j && v.position.y == i {
                        c = 'R';
                    }
                }
                print!("{}", c);
            }
            println!();
        }
        println!();
    }

    fn move_robots(&mut self, num: i32) {
        let xlen = self.xlen();
        let ylen = self.ylen();
        for i in 0..self.robots.len() {
            self.robots[i].iterate(num, xlen, ylen);
        }
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
    fn is_christmas_tree(&self, list: Vec<Coordinate>) -> bool {
        let mut result = true;
        //let mut result = 0;
        let r_places: Vec<Coordinate> = self.robots.iter().map(|x| x.position.clone()).collect();
        let tree = list;
        for t in 0..tree.len() {
            result = result && tree[t] == r_places[t];
            //result += 1;
        }
        /*if result > 50 {
            return true;
        };*/
        return result;
    }
}

fn christmas_tree(xlen: i32, ylen: i32) -> Vec<Coordinate> {
    let middle = xlen.div_euclid(2);
    let mut result: Vec<Coordinate> = Vec::new();
    result.push(Coordinate { x: middle, y: 0 });
    for i in 1..ylen {
        result.push(Coordinate {
            x: middle - i.div_euclid(2),
            y: i,
        });
        result.push(Coordinate {
            x: middle + i.div_euclid(2),
            y: i,
        });
    }
    let mut result: Vec<Coordinate> = Vec::new();
    result.push(Coordinate { x: middle, y: 0 });
    result.push(Coordinate {
        x: middle,
        y: ylen - 1,
    });
    //result.push(Coordinate { x: middle + 1, y: 1 });
    //result.push(Coordinate { x: middle - 1, y: 1 });
    return result;
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
struct Movement {
    x: i32,
    y: i32,
}
impl Movement {
    fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }
}

#[derive(Debug, Clone, Hash)]
struct Robot {
    position: Coordinate,
    movement: Movement,
}

impl Robot {
    fn iterate(&mut self, iteration: i32, xlen: i32, ylen: i32) {
        let x = self.position.x + self.movement.x * iteration;
        let y = self.position.y + self.movement.y * iteration;
        self.position.x = x.rem_euclid(xlen);
        self.position.y = y.rem_euclid(ylen);
    }
}

#[derive(Debug, Clone, Hash)]
struct Quadrant {
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
}

impl Quadrant {
    fn is_inside(&self, robot: &Robot) -> bool {
        robot.position.x >= self.start_x
            && robot.position.y >= self.start_y
            && robot.position.x < self.end_x
            && robot.position.y < self.end_y
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut map: Map = Map::new(
        args[2].parse::<i32>().unwrap(),
        args[3].parse::<i32>().unwrap(),
    );
    let mut sum: i32 = 0;
    let mut sum2: i32 = 0;
    // parse using filename
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        // let mut lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
        for line in lines.flatten() {
            map.add_line(line);
        }
    }
    // keep map2 for part2
    let mut map2 = map.clone();
    map.move_robots(100);

    let first_quadrant = Quadrant {
        start_x: 0,
        start_y: 0,
        end_x: map.xlen().div_euclid(2),
        end_y: map.ylen().div_euclid(2),
    };
    let second_quadrant = Quadrant {
        start_x: map.xlen().div_euclid(2) + 1,
        start_y: 0,
        end_x: map.xlen(),
        end_y: map.ylen().div_euclid(2),
    };
    let third_quadrant = Quadrant {
        start_x: 0,
        start_y: map.ylen().div_euclid(2) + 1,
        end_x: map.xlen().div_euclid(2),
        end_y: map.ylen(),
    };
    let fourth_quadrant = Quadrant {
        start_x: map.xlen().div_euclid(2) + 1,
        start_y: map.ylen().div_euclid(2) + 1,
        end_x: map.xlen(),
        end_y: map.ylen(),
    };
    let mut s1: i32 = 0;
    let mut s2: i32 = 0;
    let mut s3: i32 = 0;
    let mut s4: i32 = 0;
    for r in map.robots {
        if first_quadrant.is_inside(&r) {
            s1 += 1;
        } else if second_quadrant.is_inside(&r) {
            s2 += 1;
        } else if third_quadrant.is_inside(&r) {
            s3 += 1;
        } else if fourth_quadrant.is_inside(&r) {
            s4 += 1;
        }
    }

    sum = s1 * s2 * s3 * s4;
    let r_places: Vec<Coordinate> = map2.robots.iter().map(|x| x.position.clone()).collect();
    let mut periods: Vec<i32> = Vec::new();
    let mut count: i32 = 0;
    /*for robot in map2.robots.iter() {
        let mut r = robot.clone();
        let start = r.position.clone();
        let mut count: i32 = 0;
        count += 1;
        r.iterate(1, map2.xlen(), map2.ylen());
        while r.position != start {
            count += 1;
            r.iterate(1, map2.xlen(), map2.ylen());
        }
        println!("period {}", count);
        dbg!("r: {]", r);
        periods.push(count);
    }

    let mut full_period: i32 = 1;
    for period in periods {
        full_period = lcm(full_period, period);
    }
    println!("full period: {}", full_period);
    pause();*/
    while true {
        /*   let mut s1: i32 = 0;
           let mut s2: i32 = 0;
           let mut s3: i32 = 0;
           let mut s4: i32 = 0;
           map2.move_robots(1);
           count += 1;
           for r in &map2.robots {
               if first_quadrant.is_inside(&r) {
                   s1 += 1;
               } else if second_quadrant.is_inside(&r) {
                   s2 += 1;
               } else if third_quadrant.is_inside(&r) {
                   s3 += 1;
               } else if fourth_quadrant.is_inside(&r) {
                   s4 += 1;
               }
           }
           if s1 == 0 || s2 == 0 || s3 == 0 || s4 == 0 {
               map2.print();
               println!("{}", count);
               pause();
           }
        }*/
        map2.move_robots(1);
        count += 1;
        //f map2.is_christmas_tree(christmas_tree(map2.xlen(),map2.ylen())) {
        for r in map2.robots.clone() {
            let r_places: Vec<Coordinate> =
                map2.robots.iter().map(|x| x.position.clone()).collect();
            let rdown = r.position.down();
            let rdowndown = rdown.down();
            if  r_places.contains(&rdown.left())
                //&& r_places.contains(&rdown.right())
                //&& r_places.contains(&rdowndown.right().right())
                && r_places.contains(&rdowndown.left().left())
            //    && r_places.contains(&rdowndown.down().right().right().right())
                && r_places.contains(&rdowndown.down().left().left().left())
                && r_places.contains(&rdowndown.down().down().left().left().left().left())
            {
                map2.print();
                println!("count {}", count);
                pause();
            }
        }
        /*if map2.is_christmas_tree(r_places.clone()) {
            //if count.rem_euclid(5201) == 0 {
            map2.print();
            println!("count {}", count);
            pause();
        }*/
        //}
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
