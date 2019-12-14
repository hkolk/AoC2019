use std::{fs, panic, io};
use std::collections::{VecDeque, HashMap};
use std::panic::AssertUnwindSafe;
use std::io::stdin;
use std::ops::Deref;
use std::fmt::Debug;

fn main() {
    let input = "input.txt";
    let contents = fs::read_to_string(input).expect("Error reading input");
    let memory = contents.split(",").map(|item: &str| item.parse::<isize>().unwrap()).collect::<Vec<isize>>();

    //println!("{:?}", memory);
    //println!("{:#?}", memory);
    part1(&memory);
    part2(&memory, false, false);
}

fn part1(program: &Vec<isize>) {
    let mut computer = IntComputer::new(program, &Vec::new());
    while computer.state != State::Terminated {
        computer.run();
    }
    let (screen, score, blocks) = convert_output(&computer.output);

    //print_screen(&screen);
    println!("Part 1: Number of blocks = {:?}", blocks);

}

fn part2(program: &Vec<isize>, play: bool, display: bool) {
    let mut free_program = program.clone();
    free_program[0] = 2;
    let mut computer = IntComputer::new(&free_program, &Vec::new());
    let mut final_score = 0;
    while computer.state != State::Terminated {
        computer.run();
        let (screen, score, blocks) = convert_output(&computer.output);
        if display {
            print!("{}[2J", 27 as char);
            print_screen(&screen);
            println!("Score: {}, blocks remaining: {}", score, blocks);
        }
        final_score = score;
        let (ballpos_x, _) = position_off(&4, &screen);
        let (paddlepos_x, _) = position_off(&3, &screen);
        let suggestion = (ballpos_x - paddlepos_x).signum();
        //let suggestion = (ballpos_x - paddlepos_x).checked_div((ballpos_x - paddlepos_x).abs()).unwrap_or(0);
        let mut input_number = 0;
        if play {
            let input = rprompt::prompt_reply_stdout(format!("Move (< a, ^ s, > d)? Suggestion = {:?} : ", suggestion).as_str()).unwrap();
            input_number = match input.as_str() {
                "a" => -1,
                "d" => 1,
                "s" => 0,
                _ => suggestion
            };
        } else {
            input_number = suggestion;
        }
        computer.add_input(input_number)
    }
    println!("Part 2: Final score: {:?}", final_score);
}

fn position_off(tiletype: &isize, screen: &HashMap<(isize, isize), isize>) -> (isize, isize) {
    for ((x, y), tile) in screen {
        if tile == tiletype {
            return (x.clone(), y.clone());
        }
    }
    return (0, 0);
}

fn convert_output(output: &Vec<isize>) -> (HashMap<(isize, isize), isize>, usize, usize) {
    let mut screen = HashMap::new();
    let mut score: usize = 0;
    let mut blocks: usize = 0;
    for tile in output.chunks_exact(3) {
        let (x, y, tiletype) = (tile[0], tile[1], tile[2]);
        if x == -1 {
            score = tiletype as usize;
        } else {
            screen.insert((x, y), tiletype);
            if tiletype == 2 {
                blocks += 1;
            }
        }
    }
    return (screen, score, blocks);
}

fn print_screen(screen: &HashMap<(isize, isize), isize>) {
    for y in 0_isize..24 {
        for x in 0_isize..40 {
            let char = match screen.get(&(x, y)).unwrap() {
                0 => " ",
                1 => "%",
                2 => "#",
                3 => "-",
                4 => "o",
                _ => panic!()
            };
            print!("{}", char);
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
