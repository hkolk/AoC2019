use std::fs;

fn main() {
    let forealz = false;
    let input = if forealz { "input.txt" } else {"input_test.txt"};
    let contents = fs::read_to_string(input).expect("Error reading input");
    let memory = contents.split(",").map(|item: &str| item.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    //println!("{:?}", memory);
    //println!("{:#?}", memory);
    part1(&memory, forealz);
}

fn part1(memory: &Vec<usize>, forealz: bool) {
    let (final_state, output) = if forealz {
        run(memory, &vec![1])
    } else {
        run(memory,  &vec![1])
    };
    if !forealz {
        println!("memory: {:?}", final_state);
        println!("output: {:?}", output);
    }
    println!("Part 1 answer: {:?}", output)
}


fn run(original_memory: &Vec<usize>, original_input: &Vec<usize>) -> (Vec<usize>, Vec<usize>) {
    let mut memory = original_memory.clone();
    let mut output :Vec<usize> = Vec::new();
    let mut input = original_input.clone();
    input.reverse();

    let mut iptr = 0;
    loop {
        match memory[iptr] % 100 {
            1 => {
                let dest = memory[iptr + 3];
                let val1 = if (memory[iptr] / 100) % 10 == 1 { memory[iptr + 1] } else {memory[memory[iptr + 1]]};
                let val2 = if (memory[iptr] / 1000) % 10 == 1 { memory[iptr + 2] } else {memory[memory[iptr + 2]]};

                memory[dest] = val1 + val2;
                iptr += 4;
            }
            2 => {
                let dest = memory[iptr + 3];
                let val1 = if (memory[iptr] / 100) % 10 == 1 { memory[iptr + 1] } else {memory[memory[iptr + 1]]};
                let val2 = if (memory[iptr] / 1000) % 10 == 1 { memory[iptr + 2] } else {memory[memory[iptr + 2]]};

                memory[dest] = val1 * val2;
                iptr += 4;
            }
            3 => {
                let dest = memory[iptr + 1];
                memory[dest] = input.pop().unwrap();
                iptr += 2;
            }
            4 => {
                output.push(memory[memory[iptr+1]]);
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