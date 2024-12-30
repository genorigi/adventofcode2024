use self::Direction::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::slice::Iter;

#[derive(Debug, Clone)]
struct Deerpath {
    p: Coordinate,
    history: Vec<Coordinate>,
    dir: Direction,
    value: i32,
}

impl Deerpath {
    fn next(&self) -> Self {
        Self {
            p: self.p.next(&self.dir),
            dir: self.dir.clone(),
            value: self.value + 1,
            history: [self.history.clone(), vec![self.p.clone()]].concat(),
        }
    }

    fn turn_right(&self) -> Self {
        match self.dir {
            Up => Self {
                p: self.p.clone(),
                dir: Right,
                value: self.value + 1000,
                history: self.history.clone(),
            },
            Down => Self {
                p: self.p.clone(),
                dir: Left,
                value: self.value + 1000,
                history: self.history.clone(),
            },
            Right => Self {
                p: self.p.clone(),
                dir: Down,
                value: self.value + 1000,
                history: self.history.clone(),
            },
            Left => Self {
                p: self.p.clone(),
                dir: Up,
                value: self.value + 1000,
                history: self.history.clone(),
            },
        }
    }
    fn turn_left(&self) -> Self {
        match self.dir {
            Up => Self {
                p: self.p.clone(),
                dir: Left,
                value: self.value + 1000,
                history: self.history.clone(),
            },
            Down => Self {
                p: self.p.clone(),
                dir: Right,
                value: self.value + 1000,
                history: self.history.clone(),
            },
            Right => Self {
                p: self.p.clone(),
                dir: Up,
                value: self.value + 1000,
                history: self.history.clone(),
            },
            Left => Self {
                p: self.p.clone(),
                dir: Down,
                value: self.value + 1000,
                history: self.history.clone(),
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    map: Vec<Vec<char>>,
    paths: Vec<Deerpath>,
    best_values: Vec<Vec<HashMap<Direction, i32>>>,
    start: Coordinate,
    end: Coordinate,
    max_value: i32,
    winners: Vec<Deerpath>,
    seats: Vec<Coordinate>,
}

impl Map {
    fn ylen(&self) -> i32 {
        self.map.len() as i32
    }
    fn xlen(&self) -> i32 {
        self.map[0].len() as i32
    }
    fn new() -> Self {
        Self {
            map: Vec::new(),
            max_value: i32::MAX,
            paths: Vec::new(),
            start: Coordinate::new(0, 0),
            end: Coordinate::new(0, 0),
            best_values: Vec::new(),
            winners: Vec::new(),
            seats: Vec::new(),
        }
    }
    fn add_line(&mut self, line: Vec<char>) {
        self.map.push(line.clone());
        let mut new_line: Vec<HashMap<Direction, i32>> = Vec::new();
        let mut max_new_line: HashMap<Direction, i32> = HashMap::new();
        max_new_line.insert(Up, i32::MAX);
        max_new_line.insert(Down, i32::MAX);
        max_new_line.insert(Left, i32::MAX);
        max_new_line.insert(Right, i32::MAX);
        for _i in 0..line.len() {
            new_line.push(max_new_line.clone());
        }
        self.best_values.push(new_line.clone());
    }

    fn print(&self) {
        for y in 0..self.ylen() {
            for x in 0..self.xlen() {
                let c: char = self.map[y as usize][x as usize];
                print!("{}", c);
            }
            println!();
        }
        println!();
        pause();
    }

    fn is_blocked(&self, case: &Coordinate, dir: &Direction, c: char) -> bool {
        match dir {
            Direction::Up => return self.value(&case.up()).unwrap() == c,
            Direction::Down => return self.value(&case.down()).unwrap() == c,
            Direction::Left => return self.value(&case.left()).unwrap() == c,
            Direction::Right => return self.value(&case.right()).unwrap() == c,
        }
    }

    fn is_inside(&self, point: &Coordinate) -> bool {
        point.x >= 0 && point.y >= 0 && point.x < self.xlen() && point.y < self.ylen()
    }

    fn value(&self, point: &Coordinate) -> Option<char> {
        if !self.is_inside(&point) {
            return None;
        }
        return Some(self.map[point.y as usize][point.x as usize]);
    }

    fn max_value(&self, point: &Coordinate, dir: &Direction) -> Option<i32> {
        if !self.is_inside(&point) {
            return None;
        }
        return Some(
            self.best_values[point.y as usize][point.x as usize]
                .get(dir)
                .unwrap()
                .clone(),
        );
    }
    fn update_max_value(&mut self, point: &Coordinate, dir: &Direction, value: i32) {
        self.best_values[point.y as usize][point.x as usize].insert(dir.clone(), value);
    }

    fn find_path(&mut self, deer: Deerpath, value: i32) {
        let mut deer = deer.clone();
        let mut max_path_value = self.max_value(&deer.p, &deer.dir).unwrap();
        // while we are not at the end
        //if deer.value > max_path_value {
        //    println!("stopping here I'm at {} and best value is {}", deer.value, max_path_value);
        //    self.print(&deer);
        //}
        while deer.p != self.end
            && deer.value < self.max_value
            && !deer.history.contains(&deer.p)
            && deer.value < value
            && deer.value <= max_path_value
        {
            //self.print(&deer);
            self.update_max_value(&deer.p, &deer.dir, deer.value);
            let mut deers: Vec<Deerpath> = vec![deer.clone()];
            // find how many possible directions there are
            deers.push(deer.turn_right());
            deers.push(deer.turn_left());
            let mut indexes: Vec<usize> = Vec::new();
            for i in 0..deers.len() {
                if self.is_blocked(&deers[i].p, &deers[i].dir, '#') {
                    // we skip this path
                    continue;
                }
                indexes.push(i);
            }
            if indexes.len() > 1 {
                // we keep the first one, store the others
                deer = deers[indexes[0]].clone();
                indexes.remove(0);
                for i in indexes {
                    self.paths.push(deers[i].clone());
                }
            } else if indexes.len() == 1 {
                // if only one, we keep
                deer = deers[indexes[0]].clone();
            } else {
                // this is a dead end
                return;
            }
            deer = deer.next();
            max_path_value = self.max_value(&deer.p, &deer.dir).unwrap();
        }
        if deer.p == self.end {
            self.max_value = deer.value;
            self.winners.push(deer.clone());
            //println!("found {}", deer.value);
            //self.print(&deer);
        }
    }
    fn iterate(&mut self, value: i32) {
        for y in 0..self.ylen() {
            for x in 0..self.xlen() {
                if self.map[y as usize][x as usize] == 'S' {
                    self.start = Coordinate { x: x, y: y };
                    //             map.values[y as usize][x as usize].insert(Right, 0);
                    self.paths.push(Deerpath {
                        p: self.start.clone(),
                        dir: Right,
                        value: 0,
                        history: Vec::new(),
                    });
                    self.paths.push(Deerpath {
                        p: self.start.clone(),
                        dir: Up,
                        value: 1000,
                        history: Vec::new(),
                    });
                    self.paths.push(Deerpath {
                        p: self.start.clone(),
                        dir: Down,
                        value: 1000,
                        history: Vec::new(),
                    });
                } else if self.map[y as usize][x as usize] == 'E' {
                    self.end = Coordinate { x: x, y: y };
                }
            }
        }
        let mut len = self.paths.len();
        while len > 0 {
            let deer = self.paths.remove(0);
            len = self.paths.len();
            //println!("we have {} deers", self.paths.len());
            self.find_path(deer, value);
        }
        // we get all the deers, and build a list of paths
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn print(&self) {
        match self {
            Direction::Up => print!("{}", "^"),
            Direction::Down => print!("{}", "v"),
            Direction::Left => print!("{}", "<"),
            Direction::Right => print!("{}", ">"),
        }
    }
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [Up, Down, Right, Left];
        DIRECTIONS.iter()
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
            x: self.x + 1,
            y: self.y,
        };
    }
    fn left(&self) -> Coordinate {
        return Coordinate {
            x: self.x - 1,
            y: self.y,
        };
    }
    fn down(&self) -> Coordinate {
        return Coordinate {
            x: self.x,
            y: self.y + 1,
        };
    }
    fn up(&self) -> Coordinate {
        return Coordinate {
            x: self.x,
            y: self.y - 1,
        };
    }
    fn neighbours(&self) -> Vec<Coordinate> {
        vec![self.up(), self.down(), self.right(), self.left()]
    }
    fn next(&self, dir: &Direction) -> Self {
        match dir {
            Direction::Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

fn transform(c: char) -> Direction {
    match c {
        '>' => return Direction::Right,
        '^' => return Direction::Up,
        '<' => return Direction::Left,
        'v' => return Direction::Down,
        _ => return Direction::Right,
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

    let mut iterate = i32::MAX;
    while map.max_value == i32::MAX {
        println!("I'm at {}", iterate);
        map.iterate(iterate);
    }

    sum = map.max_value;


    // part2
    let list_deer: Vec<Deerpath> = map.winners.clone();
    let mut list_seats: Vec<Coordinate> = Vec::new();
    for d in list_deer {
        if d.value == map.max_value {
            list_seats.extend(d.history);
        }
    }
    for c in list_seats {
        map.map[c.y as usize][c.x as usize] = 'O';
    }
    map.print();
    sum2 = number_of_x(map.map, 'O') + 1;
    println!("Sum is :{}", sum);
    println!("Sum2 is :{}", sum2);
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
