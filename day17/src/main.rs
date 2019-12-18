use std::{fs, panic, io, fmt};
use std::collections::{VecDeque, HashMap};
use std::panic::AssertUnwindSafe;
use std::io::stdin;
use std::ops::Deref;
use std::fmt::Debug;
use itertools::Itertools;
use pathfinding::grid::Grid;
use pathfinding::directed::astar::astar;
use pathfinding::utils::absdiff;
use std::iter::Map;

fn main() {
    let input = "input.txt";
    let contents = fs::read_to_string(input).expect("Error reading input");
    let memory = contents.split(",").map(|item: &str| item.parse::<isize>().unwrap()).collect::<Vec<isize>>();

    //println!("{:?}", memory);
    //println!("{:#?}", memory);
    //part1(&memory);
    //part2(&memory, false, false);
    let mut computer = IntComputer::new(&memory);
    computer.run();
    let output = computer.output.clone();
    for tile in &output {
        print!("{}", (*tile as u8) as char);
    }
    computer.output.clear();

    let map = to_map(&output);
    let map_ro = map.clone();
    let mut intersection_sum = 0;
    'outer: for (coord, tile) in map.iter() {
        if tile == &MapTile::Scaffold {
            for neighbour in coord.neighbours() {
                if map_ro.get(&neighbour).unwrap_or(&MapTile::Space) == &MapTile::Space {
                    continue 'outer;
                }
            }
            println!("Found intersection at coord: {:?}", coord);
            intersection_sum += coord.x * coord.y;
        }
    }
    println!("Part1: {:?}", intersection_sum)
}

fn to_map(input: &Vec<isize>) -> HashMap<Coord, MapTile> {
    let mut map: HashMap<Coord, MapTile> = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    for tile in input {
        if *tile == 10 {
            x = 0;
            y += 1;
        } else {
            map.insert(Coord{x, y}, MapTile::from_isize(tile));
            x += 1;
        }
    }
    return map;
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
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum MapTile {
    Robot,
    Scaffold,
    Space
}

impl MapTile {
    fn to_char(&self) -> &str {
        return match self {
            MapTile::Robot => "X",
            MapTile::Scaffold => "#",
            MapTile::Space => ".",
        }
    }
    fn from_isize(input: &isize) -> MapTile {
        return match input {
            35 => MapTile::Scaffold,
            46 => MapTile::Space,
            _ => MapTile::Robot
        }
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