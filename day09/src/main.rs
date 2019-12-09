use std::fs;
use std::panic::{self, AssertUnwindSafe};
use std::collections::VecDeque;

fn main() {
    let input = "input.txt";
    let contents = fs::read_to_string(input).expect("Error reading input");
    let memory = contents.split(",").map(|item: &str| item.parse::<isize>().unwrap()).collect::<Vec<isize>>();

    //println!("{:?}", memory);
    //println!("{:#?}", memory);
    part1(&memory);
    part2(&memory);
}

fn part1(memory: &Vec<isize>) {
    let mut computer = IntComputer::new(memory, &Vec::new());
    computer.add_input(1);
    computer.run();
    println!("Part1 Output: {:?}", computer.output)
}

fn part2(memory: &Vec<isize>) {
    let mut computer = IntComputer::new(memory, &Vec::new());
    computer.add_input(2);
    computer.run();
    println!("Part2 Output: {:?}", computer.output)
}

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
