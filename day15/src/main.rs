use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct Box {
    start: Coordinate,
    end: Coordinate,
}

impl Box {
    fn is_against(&self, c: &Coordinate, dir: &Direction) -> bool {
        let next_move;
        match dir {
            Direction::up => next_move = c.up(),
            Direction::down => next_move = c.down(),
            Direction::left => next_move = c.left(),
            Direction::right => next_move = c.right(),
        }
        return self.start == next_move || self.end == next_move;
    }

    fn up(&self) -> Self {
        Self {
            start: self.start.up(),
            end: self.end.up(),
        }
    }
    fn down(&self) -> Self {
        Self {
            start: self.start.down(),
            end: self.end.down(),
        }
    }
    fn left(&self) -> Self {
        Self {
            start: self.start.left(),
            end: self.end.left(),
        }
    }
    fn right(&self) -> Self {
        Self {
            start: self.start.right(),
            end: self.end.right(),
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    map: Vec<Vec<char>>,
    boxes: Vec<Box>,
    robot: Coordinate,
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
            boxes: Vec::new(),
            robot: Coordinate { x: 0, y: 0 },
        }
    }
    fn add_line(&mut self, line: Vec<char>) {
        self.map.push(line.clone());
    }

    fn print(&self) {
        for i in 0..self.ylen() {
            for j in 0..self.xlen() {
                let mut c: char = self.map[i as usize][j as usize];
                let location = Coordinate::new(j, i);
                for b in self.boxes.iter() {
                    if b.start == location {
                        c = '[';
                    } else if b.end == location {
                        c = ']';
                    };
                }
                if self.robot == location {
                    c = '@';
                }
                print!("{}", c);
            }
            println!();
        }
        println!();
    }

    fn store_boxes_and_robot(&mut self) {
        for i in 0..self.ylen() {
            for j in 0..self.xlen() {
                let c: char = self.map[i as usize][j as usize];
                if c == '[' {
                    let b_start = Coordinate::new(j, i);
                    let b_end = b_start.right();
                    self.boxes.push(Box {
                        start: b_start,
                        end: b_end,
                    });
                    self.map[i as usize][j as usize] = '.';
                }
                if c == ']' {
                    self.map[i as usize][j as usize] = '.';
                }
                if c == '@' {
                    self.robot = Coordinate::new(j, i);
                    self.map[i as usize][j as usize] = '.';
                }
            }
        }
    }

    fn is_blocked(&self, case: &Coordinate, dir: &Direction, c: char) -> bool {
        match dir {
            Direction::up => return self.value(&case.up()).unwrap() == c,
            Direction::down => return self.value(&case.down()).unwrap() == c,
            Direction::left => return self.value(&case.left()).unwrap() == c,
            Direction::right => return self.value(&case.right()).unwrap() == c,
        }
    }
    fn is_inside(&self, point: &Coordinate) -> bool {
        point.x >= 0 && point.y >= 0 && point.x < self.xlen() && point.y < self.ylen()
    }

    fn move_box(&mut self, b: usize, dir: &Direction) -> Option<Vec<usize>> {
        let mut box_can_move = Vec::new();
        if self.is_blocked(&self.boxes[b].start, dir, '#')
            || self.is_blocked(&self.boxes[b].end, dir, '#')
        {
            return None;
        }
        for bx in 0..self.boxes.len() {
            if bx == b {
                continue;
            }
            if self.boxes[bx].is_against(&self.boxes[b].start, dir)
                || self.boxes[bx].is_against(&self.boxes[b].end, dir)
            {
                if let Some(x) = self.move_box(bx,dir) {
                    box_can_move.extend(x);
                } else {
                    return None
                }
            }
        }

        box_can_move.push(b);
        return Some(box_can_move);
    }

    fn move_case(&mut self, case: &Coordinate, dir: &Direction) -> bool {
        if self.is_blocked(&case, &dir, '#') {
            return false;
        }
        if self.is_blocked(&case, &dir, 'O') {
            let mut result_move = false;
            match dir {
                Direction::up => result_move = self.move_case(&case.up(), &dir),
                Direction::down => result_move = self.move_case(&case.down(), &dir),
                Direction::left => result_move = self.move_case(&case.left(), &dir),
                Direction::right => result_move = self.move_case(&case.right(), &dir),
            }
            if !result_move {
                return false;
            }
        }
        match dir {
            Direction::up => self.swap(&case, &case.up()),
            Direction::down => self.swap(&case, &case.down()),
            Direction::left => self.swap(&case, &case.left()),
            Direction::right => self.swap(&case, &case.right()),
        }
        return true;
    }

    fn move_robot(&mut self, robot: Coordinate, dir: &Direction) -> Coordinate {
        if self.move_case(&robot, &dir) {
            match dir {
                Direction::up => return robot.up(),
                Direction::down => return robot.down(),
                Direction::left => return robot.left(),
                Direction::right => return robot.right(),
            }
        }
        return robot.clone();
    }

    fn move_self_robot(&mut self, dir: &Direction) {
        for b in 0..self.boxes.len() {
            if self.boxes[b].is_against(&self.robot, dir) {
                if let Some(list) = self.move_box(b, dir) {
                    for b in list {
                        match dir {
                            Direction::up => self.boxes[b] = self.boxes[b].up(),
                            Direction::down => self.boxes[b] = self.boxes[b].down(),
                            Direction::left => self.boxes[b] = self.boxes[b].left(),
                            Direction::right => self.boxes[b] = self.boxes[b].right(),
                        }
                    }
                } else {
                    return;
                }
                break;
            }
        }

        if !self.is_blocked(&self.robot, dir, '#') {
            match dir {
                Direction::up => self.robot = self.robot.up(),
                Direction::down => self.robot = self.robot.down(),
                Direction::left => self.robot = self.robot.left(),
                Direction::right => self.robot = self.robot.right(),
            }
        }
    }

    fn swap(&mut self, case: &Coordinate, new_case: &Coordinate) {
        let c: char = self.value(&case).unwrap();
        self.map[case.y as usize][case.x as usize] = self.value(&new_case).unwrap();
        self.map[new_case.y as usize][new_case.x as usize] = c;
    }

    fn value(&self, point: &Coordinate) -> Option<char> {
        if !self.is_inside(&point) {
            return None;
        }
        return Some(self.map[point.y as usize][point.x as usize]);
    }

    fn calculate_score(&self) -> i32 {
        let mut result: i32 = 0;
        for i in 0..self.ylen() {
            for j in 0..self.xlen() {
                if self.map[j as usize][i as usize] == 'O' {
                    result += j * 100;
                    result += i;
                }
            }
        }
        return result;
    }
    fn calculate_boxes_score(&self) ->i32 {
        let mut result: i32 = 0;
        for b in self.boxes.iter() {
            result += b.start.y * 100;
            result += b.start.x;
        }
        return result;
    }
    fn calculate_new_map(&self) -> Self {
        let mut result: Map = Map::new();
        for i in 0..self.ylen() {
            let mut line: Vec<char> = Vec::new();
            for j in 0..self.xlen() {
                if self.map[i as usize][j as usize] == '.' {
                    line.push('.');
                    line.push('.');
                } else if self.map[i as usize][j as usize] == 'O' {
                    line.push('[');
                    line.push(']');
                } else if self.map[i as usize][j as usize] == '@' {
                    line.push('@');
                    line.push('.');
                } else if self.map[i as usize][j as usize] == '#' {
                    line.push('#');
                    line.push('#');
                }
            }
            result.map.push(line);
        }
        return result;
    }
}

#[derive(Debug, Clone, Hash)]
enum Direction {
    up,
    down,
    left,
    right,
}

impl Direction {
    fn print(&self) {
        match self {
            Direction::up => print!("{}", "^"),
            Direction::down => print!("{}", "v"),
            Direction::left => print!("{}", "<"),
            Direction::right => print!("{}", ">"),
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
}

fn transform(c: char) -> Direction {
    match c {
        '>' => return Direction::right,
        '^' => return Direction::up,
        '<' => return Direction::left,
        'v' => return Direction::down,
        _ => return Direction::right,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut map: Map = Map::new();
    let mut directions: Vec<Direction> = Vec::new();
    let mut direction_turn = false;
    let mut sum: i32 = 0;
    let mut sum2: i32 = 0;
    let mut robot: Coordinate = Coordinate::new(0, 0);
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
                directions.extend::<Vec<Direction>>(line.chars().map(|x| transform(x)).collect());
            }
        }
    }
    for i in 0..map.ylen() {
        for j in 0..map.xlen() {
            let ici = Coordinate { x: j, y: i };
            if map.value(&ici).unwrap() == '@' {
                robot = Coordinate { x: j, y: i };
            }
        }
    }

    let mut map2 = map.calculate_new_map();

    for dir in directions.clone() {
        robot = map.move_robot(robot, &dir);
    }

    sum = map.calculate_score();

    // part2
    // store boxes
    map2.store_boxes_and_robot();

    for dir in directions {
        map2.move_self_robot(&dir);
    }
    map2.print();
    sum2 = map2.calculate_boxes_score();
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
