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
}

fn part1(memory: &Vec<isize>) {
    let mut computer = IntComputer::new(memory, &Vec::new());
    computer.run();
    println!("Output: {:?}", computer.output)

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
        loop {
            match self.memory[self.iptr] % 100 {
                1 => {
                    let dest = self.memory[self.iptr + 3] as usize;
                    self.memory[dest] = get_val1(&self.memory, self.iptr) + get_val2(&self.memory, self.iptr);
                    self.iptr += 4;
                }
                2 => {
                    let dest = self.memory[self.iptr + 3] as usize;
                    let result = panic::catch_unwind(AssertUnwindSafe(|| {
                        self.memory[dest] = get_val1(&self.memory, self.iptr) * get_val2(&self.memory, self.iptr);
                    }));
                    if result.is_err() {
                        panic!("Attempted multiply of {:?} and {:?}", get_val1(&self.memory, self.iptr), get_val2(&self.memory, self.iptr))
                    }
                    self.iptr += 4;
                }
                3 => {
                    if self.input.len() == 0 {
                        // halting state
                        self.state = State::Halted;
                        break;
                    }
                    let dest = self.memory[self.iptr + 1] as usize;
                    self.memory[dest] = self.input.pop_front().unwrap();
                    self.iptr += 2;
                }
                4 => {
                    self.output.push(get_val1(&self.memory, self.iptr));
                    self.iptr += 2;
                }
                5 => {
                    self.iptr = if get_val1(&self.memory, self.iptr) > 0 { get_val2(&self.memory, self.iptr) as usize } else { self.iptr + 3 };

                }
                6 => {
                    self.iptr = if get_val1(&self.memory, self.iptr) == 0 { get_val2(&self.memory, self.iptr) as usize } else { self.iptr + 3 };
                }
                7 => {
                    let dest = self.memory[self.iptr + 3] as usize;
                    self.memory[dest] = if get_val1(&self.memory, self.iptr) < get_val2(&self.memory, self.iptr) { 1 } else { 0 };
                    self.iptr += 4;
                }
                8 => {
                    let dest = self.memory[self.iptr + 3] as usize;
                    self.memory[dest] = if get_val1(&self.memory, self.iptr) == get_val2(&self.memory, self.iptr) { 1 } else { 0 };
                    self.iptr += 4;
                }
                9 => {
                    self.relative_base += get_val1(&self.memory, self.iptr);
                    self.iptr += 2;

                }
                99 => {
                    self.state = State::Terminated;
                    break;
                }
                _ => panic!("All f'd up: {:?}", self)
            }
        }
        return self.clone();
    }
}

fn get_val1(memory: &Vec<isize>, iptr: usize) -> isize {
    return get_parameter(&memory, iptr+1, get_parameter_mode(memory[iptr], 1));
}

fn get_val2(memory: &Vec<isize>, iptr: usize) -> isize {
    return get_parameter(&memory, iptr+2, get_parameter_mode(memory[iptr], 2));
}

fn get_parameter(memory: &Vec<isize>, loc: usize, mode: Mode) -> isize {
    return match mode {
        Mode::Position => memory[memory[loc] as usize],
        Mode::Immediate => memory[loc],
        Mode::Relative => panic!("Unimplemented!")
    }
}

fn get_parameter_mode(opcode: isize, param: u32) -> Mode {
    return match opcode / 10_isize.pow(param + 1) % 10 {
        0 => Mode::Position,
        1 => Mode::Immediate,
        2 => Mode::Relative,
        _ => panic!()
    };
}