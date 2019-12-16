use std::{fs, panic, io, fmt};
use std::collections::{VecDeque, HashMap};
use std::panic::AssertUnwindSafe;
use std::io::stdin;
use std::ops::Deref;
use std::fmt::Debug;
use itertools::Itertools;
use crate::MoveResult::MovedAndFound;
use pathfinding::grid::Grid;
use pathfinding::directed::astar::astar;
use pathfinding::utils::absdiff;

fn main() {
    let input = "input.txt";
    let contents = fs::read_to_string(input).expect("Error reading input");
    let memory = contents.split(",").map(|item: &str| item.parse::<isize>().unwrap()).collect::<Vec<isize>>();

    //println!("{:?}", memory);
    //println!("{:#?}", memory);
    //part1(&memory);
    //part2(&memory, false, false);

    let mut droid = Droid::new(&memory);
    let mut direction = Direction::South;
    let mut breaker = 0;
    loop {
        let result = droid.smart_move();
        //println!("Moving result: Droid location: {:?}, heading: {:?}", droid.loc, droid.direction);

        // not in the mood for loop detection...
        breaker += 1;
        if breaker > 3_000 {
            break;
        }
    }

    println!("======= Full Map ========");
    print_map(&droid.map);
    let (path, moves) = find_path(&droid.map, &Coord{x:0, y:0}, &droid.hatch);
    let mut walked_map = droid.map.clone();
    for tile in path {
        walked_map.insert(tile, MapTile::Path);
    }
    println!("======= Walked Map ========");
    print_map(&walked_map);
    println!("Part 1: {:?} moves", moves);


    let mut oxygen_map = droid.map.clone();
    oxygen_map.insert(droid.hatch, MapTile::Oxygen);
    let mut open_tiles = oxygen_map.values().filter(|item| *item == &MapTile::Open).count();

    let mut counter = 0;
    while open_tiles > 0 {
        //println!("Open tiles: {:?}", open_tiles);
        let ro_oxygenmap = oxygen_map.clone();
        let oxygen_tiles = ro_oxygenmap.iter().filter(|(key, value)| *value == &MapTile::Oxygen).map(|(key, coord)| key).collect::<Vec<_>>();
        for oxygen_tile in oxygen_tiles {
            for neighbour in oxygen_tile.neighbours() {
                if oxygen_map.get(&neighbour).unwrap_or(&MapTile::Wall) == &MapTile::Open {
                    oxygen_map.insert(neighbour, MapTile::Oxygen);
                }
            }

        }
        counter += 1;
        open_tiles = oxygen_map.values().filter(|item| *item == &MapTile::Open).count();
    }
    print!("Part2: {:?} minutes", counter);

}

fn print_map(map: &HashMap<Coord, MapTile>) {
    let (min_x, max_x) = map.keys().map(|coord|coord.x).minmax().into_option().unwrap();
    let (min_y, max_y) = map.keys().map(|coord|coord.y).minmax().into_option().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!("{}", map.get(&Coord{x, y}).unwrap_or(&MapTile::Unexplored).to_char());
        }
        println!();
    }
}



fn find_path(map: &HashMap<Coord, MapTile>, from: &Coord, to: &Coord) -> (Vec<Coord>, u32) {
    let result = astar(
        from,
        |p| p.successors(map),
        |p| p.distance(to),
        |p| p == to
    ).unwrap();
    return result;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4
}

impl Direction {
    fn left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North

        }
    }
    fn right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::West => Direction::North,
            Direction::South => Direction::West,
            Direction::East => Direction::South

        }
    }
    fn reverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::East => Direction::West

        }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum MoveResult {
    Blocked,
    Moved,
    MovedAndFound
}

impl MoveResult {
    fn from(result: isize) -> MoveResult {
        match result {
            0 => MoveResult::Blocked,
            1 => MoveResult::Moved,
            2 => MoveResult::MovedAndFound,
            _ => panic!()
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Coord {
    x: isize,
    y: isize
}

impl Coord {
    fn next(&self, direction: Direction) -> Coord {
        return match direction {
            Direction::North =>    Coord{y: self.y - 1, x: self.x},
            Direction::East => Coord{y: self.y, x: self.x + 1},
            Direction::South =>  Coord{y: self.y + 1, x: self.x},
            Direction::West =>  Coord{y: self.y, x: self.x - 1},
        }
    }
    fn neighbours(&self) -> Vec<Coord> {
        return vec!(
            //Coord{x: self.x - 1, y: self.y - 1}, // can't diagonal
            Coord{x: self.x - 1, y: self.y},
            //Coord{x: self.x - 1, y: self.y + 1}, // can't diagonal
            Coord{x: self.x, y: self.y - 1},
            //Coord(x: self.x, y: self.y), skip ourselves
            Coord{x: self.x, y: self.y + 1},
            //Coord{x: self.x + 1, y: self.y - 1}, // can't diagonal
            Coord{x: self.x + 1, y: self.y},
            //Coord{x: self.x + 1, y: self.y + 1}, // can't diagonal
        )
    }

    fn distance(&self, other: &Coord) -> u32 {
        (absdiff(self.x, other.x) + absdiff(self.y, other.y)) as u32
    }

    fn successors(&self, map: &HashMap<Coord, MapTile>) -> Vec<(Coord, u32)> {
        self.neighbours().into_iter().filter(|i| map.get(i).unwrap_or(&MapTile::Wall) != &MapTile::Wall).map(|i| (i, 1) ).collect()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum MapTile {
    Open,
    Wall,
    Hatch,
    Unexplored,
    Path,
    Oxygen
}

impl MapTile {
    fn to_char(&self) -> &str {
        return match self {
            MapTile::Open => " ",
            MapTile::Wall => "#",
            MapTile::Hatch => "X",
            MapTile::Unexplored => "*",
            MapTile::Path => "|",
            MapTile::Oxygen => "O"
        }
    }
}

struct Droid {
    computer: IntComputer,
    loc: Coord,
    direction: Direction,
    map: HashMap<Coord, MapTile>,
    hatch: Coord,
}

impl Droid {
    fn new(program: &Vec<isize>) -> Droid {
        return Droid {
            computer: IntComputer::new(program),
            loc: Coord{x: 0, y: 0},
            direction: Direction::North,
            map: HashMap::new(),
            hatch: Coord{x: 0, y: 0},

        }
    }

    fn smart_move(&mut self) {
        // try to go left, return if successfull
        if self.do_move(self.direction.left()) != MoveResult::Blocked {
            self.direction = self.direction.left();
            return;
        }
        if self.do_move(self.direction) != MoveResult::Blocked {
            return;
        }
        if self.do_move(self.direction.right()) != MoveResult::Blocked {
            self.direction = self.direction.right();
            return;
        }
        if self.do_move(self.direction.reverse()) != MoveResult::Blocked {
            self.direction = self.direction.reverse();
            return;
        }
        panic!("How did I end up here?")
    }

    fn do_move(&mut self, direction: Direction) -> MoveResult {
        self.computer.input.push_back(direction.clone() as isize);
        self.computer.run();
        let result = MoveResult::from(self.computer.output.pop().unwrap());
        match result {
            MoveResult::Moved => self.map.insert(self.loc.next(direction), MapTile::Open),
            MoveResult::Blocked => self.map.insert(self.loc.next(direction), MapTile::Wall),
            MoveResult::MovedAndFound => self.map.insert(self.loc.next(direction), MapTile::Hatch)
        };
        if result ==  MoveResult::Moved{
            self.loc = self.loc.next(direction);
        }
        if result == MovedAndFound {
            self.loc = self.loc.next(direction);
            self.hatch = self.loc.clone();
        }

        return result;
    }
}
// intcomputer from here

#[derive(Copy, Clone, Debug, PartialEq)]
enum State {
    Running,
    Halted,
    Terminated
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Mode {
    Position,
    Immediate,
    Relative
}

#[derive(Clone, Debug)]
struct IntComputer {
    memory: Vec<isize>,
    iptr: usize,
    input: VecDeque<isize>,
    output: Vec<isize>,
    state: State,
    relative_base: isize
}

impl IntComputer {
    fn new(initial_memory: &Vec<isize>) -> IntComputer {
        return IntComputer {
            memory: initial_memory.clone(),
            iptr: 0,
            input: VecDeque::new(),
            output: Vec::new(),
            state: State::Running,
            relative_base: 0
        };
    }

    fn add_input(&mut self, input: isize) {
        self.input.push_back(input);
    }

    fn run(&mut self) {
        self.state = State::Running;
        let mut breaker = 0;
        loop {
            match self.memory[self.iptr] % 100 {
                1 => {
                    let dest = self.get_dest(3);
                    self.set_value(dest, self.get_val1() + self.get_val2() );
                    //self.memory[dest] = self.get_val1() + self.get_val2();
                    self.iptr += 4;
                }
                2 => {
                    let dest = self.get_dest(3);
                    //let dest = self.get_val3() as usize;
                    let result = panic::catch_unwind(AssertUnwindSafe(|| {
                        self.set_value(dest, self.get_val1() * self.get_val2());
                    }));
                    if result.is_err() {
                        panic!("Attempted multiply of {:?} and {:?}", self.get_val1(), self.get_val2())
                    }
                    self.iptr += 4;
                }
                3 => {
                    if self.input.len() == 0 {
                        // halting state
                        self.state = State::Halted;
                        break;
                    }
                    let dest = self.get_dest(1);
                    let value = self.input.pop_front().unwrap();
                    self.set_value(dest, value);
                    self.iptr += 2;
                }
                4 => {
                    self.output.push(self.get_val1());
                    self.iptr += 2;
                }
                5 => {
                    self.iptr = if self.get_val1() > 0 { self.get_val2() as usize } else { self.iptr + 3 };

                }
                6 => {
                    self.iptr = if self.get_val1() == 0 { self.get_val2() as usize } else { self.iptr + 3 };
                }
                7 => {
                    let dest = self.get_dest(3);
                    self.set_value(dest, if self.get_val1() < self.get_val2() { 1 } else { 0 });
                    self.iptr += 4;
                }
                8 => {
                    let dest = self.get_dest(3);
                    self.set_value(dest, if self.get_val1() == self.get_val2() { 1 } else { 0 });
                    self.iptr += 4;
                }
                9 => {
                    self.relative_base += self.get_val1();
                    self.iptr += 2;

                }
                99 => {
                    self.state = State::Terminated;
                    break;
                }
                _ => panic!("All f'd up: {:?}", self)
            }
            //println!("{:?}", self);
            breaker += 1;
            if breaker > 100 {
                //break;
            }
        }
        return;// self.clone();
    }

    fn set_value(&mut self, loc: usize, value: isize) {
        if loc >= self.memory.len() {
            self.memory.resize(loc + 1, 0);
        }
        self.memory[loc] = value;
    }

    fn get_val1(&self) -> isize {
        return self.get_parameter(self.iptr+1, self.get_parameter_mode(self.memory[self.iptr], 1));
    }

    fn get_val2(&self) -> isize {
        return self.get_parameter(self.iptr+2, self.get_parameter_mode(self.memory[self.iptr], 2));
    }

    fn get_dest(&self, distance: u32) -> usize {
        return self.get_memory_loc(self.iptr+ distance as usize, self.get_parameter_mode(self.memory[self.iptr], distance))
    }

    fn get_memory_loc(&self, loc: usize, mode: Mode) -> usize {
        return match mode {
            Mode::Position => self.memory[loc] as usize,
            //Mode::Position => self.memory[self.memory[loc] as usize],
            Mode::Immediate => loc,
            Mode::Relative => (self.relative_base + self.memory[loc]) as usize
            //Mode::Relative => self.relative_base + self.memory[loc]
        }
    }

    fn get_parameter(&self, loc: usize, mode: Mode) -> isize {
        let default:isize = 0;
        return match mode {
            Mode::Position => *self.memory.get(self.memory[loc] as usize).unwrap_or(&default),
            //Mode::Position => self.memory[self.memory[loc] as usize],
            Mode::Immediate => self.memory[loc],
            Mode::Relative => *self.memory.get((self.relative_base + self.memory[loc]) as usize).unwrap_or(&default)
            //Mode::Relative => self.relative_base + self.memory[loc]
        }
    }

    fn get_parameter_mode(&self, opcode: isize, param: u32) -> Mode {
        return match opcode / 10_isize.pow(param + 1) % 10 {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => panic!()
        };
    }
}