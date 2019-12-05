use std::fs;

fn main() {
    let forealz = false;
    let input = if forealz { "input.txt" } else {"input_test.txt"};
    let contents = fs::read_to_string(input).expect("Error reading input");
    let memory = contents.split(",").map(|item: &str| item.parse::<isize>().unwrap()).collect::<Vec<isize>>();

    //println!("{:?}", memory);
    //println!("{:#?}", memory);
    part1(&memory, forealz);
}

fn part1(memory: &Vec<isize>, forealz: bool) {
    let (final_state, output) = if forealz {
        run(memory, &vec![1])
    } else {
        run(memory,  &vec![1])
    };
    if !forealz {
        println!("memory: {:?}", final_state);
        println!("output: {:?}", output);
    }
    println!("Part 1 answer: {:?}", output.last())
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
                let val1 = if (memory[iptr] / 100) % 10 == 1 { memory[iptr + 1] } else {memory[memory[iptr + 1] as usize]};
                let val2 = if (memory[iptr] / 1000) % 10 == 1 { memory[iptr + 2] } else {memory[memory[iptr + 2] as usize]};

                memory[dest] = val1 + val2;
                iptr += 4;
            }
            2 => {
                let dest = memory[iptr + 3] as usize;
                let val1 = if (memory[iptr] / 100) % 10 == 1 { memory[iptr + 1] } else {memory[memory[iptr + 1] as usize]};
                let val2 = if (memory[iptr] / 1000) % 10 == 1 { memory[iptr + 2] } else {memory[memory[iptr + 2] as usize]};

                memory[dest] = val1 * val2;
                iptr += 4;
            }
            3 => {
                let dest = memory[iptr + 1] as usize;
                memory[dest] = input.pop().unwrap();
                iptr += 2;
            }
            4 => {
                output.push(memory[memory[iptr+1] as usize]);
                iptr += 2;
            }
            99 => {
                break;
            }
            _ => panic!()
        }
    }
    return (memory, output);
}