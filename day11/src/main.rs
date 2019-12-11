use std::fs;
use std::panic::{self, AssertUnwindSafe};
use std::collections::{VecDeque, HashMap};
use crate::Direction::{Right, Left};

fn main() {
    let input = "input.txt";
    let contents = fs::read_to_string(input).expect("Error reading input");
    let memory = contents.split(",").map(|item: &str| item.parse::<isize>().unwrap()).collect::<Vec<isize>>();

    //println!("{:?}", memory);
    //println!("{:#?}", memory);
    part1(&memory);
    part2(&memory);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Coord {
    x: isize,
    y: isize
}

impl Coord {
    fn do_move(&self, direction: Direction) -> Coord {
        return match direction {
            Direction::Up =>    Coord{y: self.y - 1, x: self.x},
            Direction::Right => Coord{y: self.y, x: self.x + 1},
            Direction::Down =>  Coord{y: self.y + 1, x: self.x},
            Direction::Left =>  Coord{y: self.y, x: self.x - 1},
        }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn turn(&self, clockwise: bool) -> Direction {
        return match &self {
            Direction::Up => if clockwise { Direction::Right } else { Direction::Left },
            Direction::Right => if clockwise { Direction::Down } else { Direction::Up },
            Direction::Down => if clockwise { Direction::Left } else { Direction::Right },
            Direction::Left => if clockwise { Direction::Up } else { Direction::Down },
        }
    }
}


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Color {
    White,
    Black
}

impl Color {
    fn paint(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White
        }
    }

    fn to_int(&self) -> isize {
        return match self {
            Color::White => 1,
            Color::Black => 0
        }
    }
    fn from_int(code: isize) -> Color {
        return match code {
            1 => Color::White,
            0 => Color::Black,
            _ => panic!()
        }
    }
}


fn part1(memory: &Vec<isize>) {
    let mut panels: HashMap<Coord, Color> = HashMap::new();
    let mut computer = IntComputer::new(memory, &Vec::new());

    let mut location = Coord{x:0, y:0};
    panels.entry(location).or_insert(Color::Black);
    let mut direction = Direction::Up;

    loop {
        let current_color = panels.entry(location).or_insert(Color::Black);
        computer.input.push_back( current_color.to_int());
        let mut state = computer.run();
        let clockwise = state.output.pop().unwrap() == 1;
        let color = state.output.pop().unwrap();

        let new_color = Color::from_int(color);
        let entry = panels.entry(location).or_insert(new_color);
        *entry = new_color.clone();
        //println!("Painted {:?} to {:?}", location, new_color);

        direction = direction.turn(clockwise);
        location = location.do_move(direction);
        //println!("Moved {:?} to new location {:?}", direction, location);

        if state.state == State::Terminated {
            break;
        }
    }
    println!("Part1: panels painted: {:?}", panels.len());

/*
    let mut computer = IntComputer::new(memory, &Vec::new());
    computer.add_input(1);
    computer.run();
    println!("Part1 Output: {:?}", computer.output)
*/
}

fn part2(memory: &Vec<isize>) {
    let mut panels: HashMap<Coord, Color> = HashMap::new();
    let mut computer = IntComputer::new(memory, &Vec::new());

    let mut location = Coord{x:0, y:0};
    panels.entry(location).or_insert(Color::White);
    let mut direction = Direction::Up;

    loop {
        let current_color = panels.entry(location).or_insert(Color::Black);
        computer.input.push_back( current_color.to_int());
        let mut state = computer.run();
        let clockwise = state.output.pop().unwrap() == 1;
        let color = state.output.pop().unwrap();

        let new_color = Color::from_int(color);
        let entry = panels.entry(location).or_insert(new_color);
        *entry = new_color.clone();
        //println!("Painted {:?} to {:?}", location, new_color);

        direction = direction.turn(clockwise);
        location = location.do_move(direction);
        //println!("Moved {:?} to new location {:?}", direction, location);

        if state.state == State::Terminated {
            break;
        }
    }
    println!("Part2: panels painted: {:?}", panels.len());

    let (mut min_x, mut max_x, mut min_y, mut max_y) = (0, 0, 0, 0);
    for (coord, _) in &panels {
        min_x = min_x.min(coord.x);
        min_y = min_y.min(coord.y);
        max_x = max_x.max(coord.x);
        max_y = max_y.max(coord.y);
    }
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let color = panels.get(&Coord{x, y}).unwrap_or(&Color::White);
            if  color == &Color::White {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
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
    phase: isize,
    relative_base: isize
}

impl IntComputer {
    fn new(initial_memory: &Vec<isize>, initial_input: &Vec<isize>) -> IntComputer {
        return IntComputer {
            memory: initial_memory.clone(),
            iptr: 0,
            input: VecDeque::from(initial_input.clone()),
            output: Vec::new(),
            state: State::Running,
            phase: initial_input.first().unwrap_or(&-1).clone(),
            relative_base: 0
        };
    }

    fn add_input(&mut self, input: isize) {
        self.input.push_back(input);
    }

    fn run(&mut self) -> IntComputer {
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
        return self.clone();
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
