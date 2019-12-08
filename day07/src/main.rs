use permutohedron::heap_recursive;
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

fn part2(memory: &Vec<isize>) {
    //let data = [3, 1, 4, 2, 0];
    let mut data = [5, 6, 7, 8, 9];
    let mut permutations = Vec::new();
    heap_recursive(&mut data, |permutation| {
        permutations.push(permutation.to_vec())
    });

    let mut max_output = 0;
    let mut max_phases = Vec::new();

    for phases in permutations {
        let mut computers = VecDeque::from(phases.to_vec().into_iter().map(|phase| IntComputer::new(memory, &vec![phase])).collect::<Vec<_>>());
        let mut prev_value = 0;
        let mut running = computers.len();
        while computers.len() > 0 {
            let mut computer = computers.pop_front().unwrap();
            computer.add_input(prev_value);
            //println!("beginning next one at {:?}", computer);
            let mut new_computer = computer.run();
            match new_computer.state {
                State::Halted => {
                    //println!("Halted");
                    computers.push_back(computer);
                    prev_value = new_computer.output.pop().unwrap();
                }
                State::Terminated => {
                    //println!("Terminated");
                    prev_value = computer.output.pop().unwrap();
                    running -= 1;
                    if running == 0 {
                        break;
                    }
                }
                _ => {
                    panic!();
                }
            }
        }
        //println!("Final value: {:?}", prev_value);
        if prev_value > max_output {
            max_output = prev_value;
            max_phases = phases.clone();
        }
    }
    println!("Part 2: Max Phases {:?} generated {:?}", max_phases, max_output);


    /*
    let (final_state, output) = run(memory,  &vec![5]);
    println!("memory: {:?}", final_state);
    println!("output: {:?}", output);
    println!("Part 1 answer: {:?}", output.last().unwrap())
    */
}

fn part1(memory: &Vec<isize>) {
    let mut data = [0, 1, 2, 3, 4];
    let mut permutations = Vec::new();
    heap_recursive(&mut data, |permutation| {
        permutations.push(permutation.to_vec())
    });

    let mut max_output = 0;
    let mut max_phases = Vec::new();

    for phases in permutations {
        let mut prev_output = 0;
        for i in 0..5 {
            let input = VecDeque::from(vec![phases[i], prev_output]);
            let (_final_state, output) = run(memory,  &input);
            prev_output = *output.last().unwrap();
        }
        if prev_output > max_output {
            max_output = prev_output;
            max_phases = phases.clone();
        }
        //println!("Phases {:?} generated {:?}", phases, prev_output);
    }
    println!("Part 1: Max Phases {:?} generated {:?}", max_phases, max_output);

    /*
    let (final_state, output) = run(memory,  &vec![5]);
    println!("memory: {:?}", final_state);
    println!("output: {:?}", output);
    println!("Part 1 answer: {:?}", output.last().unwrap())
    */
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum State {
    Running,
    Halted,
    Terminated
}

#[derive(Clone, Debug)]
struct IntComputer {
    memory: Vec<isize>,
    iptr: usize,
    input: VecDeque<isize>,
    output: Vec<isize>,
    state: State,
    phase: isize
}

impl IntComputer {
    fn new(initial_memory: &Vec<isize>, initial_input: &Vec<isize>) -> IntComputer {
        return IntComputer {
            memory: initial_memory.clone(),
            iptr: 0,
            input: VecDeque::from(initial_input.clone()),
            output: Vec::new(),
            state: State::Running,
            phase: initial_input.first().unwrap_or(&-1).clone()
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
                    // halting state
                    //println!("[{:?}] Halting because I just pushed data, which was {:?}", self.phase, self.output);
                    //println!("{:?}", self);
                    self.state = State::Halted;
                    break;
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


fn run(original_memory: &Vec<isize>, original_input: &VecDeque<isize>) -> (Vec<isize>, Vec<isize>) {
    let mut memory = original_memory.clone();
    let mut output :Vec<isize> = Vec::new();
    let mut input = original_input.clone();

    let mut iptr: usize = 0;
    loop {
        match memory[iptr] % 100 {
            1 => {
                let dest = memory[iptr + 3] as usize;
                memory[dest] = get_val1(&memory, iptr) + get_val2(&memory, iptr);
                iptr += 4;
            }
            2 => {
                let dest = memory[iptr + 3] as usize;
                memory[dest] = get_val1(&memory, iptr) * get_val2(&memory, iptr);
                iptr += 4;
            }
            3 => {
                if input.len() == 0 {
                    // halting state
                }
                let dest = memory[iptr + 1] as usize;
                memory[dest] = input.pop_front().unwrap();
                iptr += 2;
            }
            4 => {
                output.push(get_val1(&memory, iptr));
                iptr += 2;
            }
            5 => {
                iptr = if get_val1(&memory, iptr) > 0 { get_val2(&memory, iptr) as usize } else { iptr + 3 };

            }
            6 => {
                iptr = if get_val1(&memory, iptr) == 0 { get_val2(&memory, iptr) as usize } else { iptr + 3 };
            }
            7 => {
                let dest = memory[iptr + 3] as usize;
                memory[dest] = if get_val1(&memory, iptr) < get_val2(&memory, iptr) { 1 } else { 0 };
                iptr += 4;
            }
            8 => {
                let dest = memory[iptr + 3] as usize;
                memory[dest] = if get_val1(&memory, iptr) == get_val2(&memory, iptr) { 1 } else { 0 };
                iptr += 4;
            }
            99 => {
                break;
            }
            _ => panic!()
        }
    }
    return (memory, output);
}

fn get_val1(memory: &Vec<isize>, iptr: usize) -> isize {
    return get_parameter(&memory, iptr+1, is_immediate(memory[iptr], 1));
}

fn get_val2(memory: &Vec<isize>, iptr: usize) -> isize {
    return get_parameter(&memory, iptr+2, is_immediate(memory[iptr], 2));
}

fn get_parameter(memory: &Vec<isize>, loc: usize, immediate: bool) -> isize {
    return if immediate { memory[loc] } else {memory[memory[loc] as usize]};
}

fn is_immediate(opcode: isize, param: u32) -> bool {
    return opcode / 10_isize.pow(param + 1) % 10 == 1

}