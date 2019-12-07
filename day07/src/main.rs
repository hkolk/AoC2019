use permutohedron::heap_recursive;
use std::fs;

fn main() {
    let input = "input.txt";
    let contents = fs::read_to_string(input).expect("Error reading input");
    let memory = contents.split(",").map(|item: &str| item.parse::<isize>().unwrap()).collect::<Vec<isize>>();

    //println!("{:?}", memory);
    //println!("{:#?}", memory);
    part1(&memory);
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
            let input = vec![phases[i], prev_output];
            let (final_state, output) = run(memory,  &input);
            prev_output = *output.last().unwrap();
        }
        if(prev_output > max_output) {
            max_output = prev_output;
            max_phases = phases.clone();
        }
        println!("Phases {:?} generated {:?}", phases, prev_output);
    }
    println!("Max Phases {:?} generated {:?}", max_phases, max_output);

    /*
    let (final_state, output) = run(memory,  &vec![5]);
    println!("memory: {:?}", final_state);
    println!("output: {:?}", output);
    println!("Part 1 answer: {:?}", output.last().unwrap())
    */
}


fn run(original_memory: &Vec<isize>, original_input: &Vec<isize>) -> (Vec<isize>, Vec<isize>) {
    let mut memory = original_memory.clone();
    let mut output :Vec<isize> = Vec::new();
    let mut input = original_input.clone();
    input.reverse();

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
                let dest = memory[iptr + 1] as usize;
                memory[dest] = input.pop().unwrap();
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