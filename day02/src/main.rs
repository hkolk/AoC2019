use std::fs;

fn main() {
    let forealz = true;
    let input = if forealz { "input.txt" } else {"input_test.txt"};
    let contents = fs::read_to_string(input).expect("Error reading input");
    let memory = contents.split(",").map(|item: &str| item.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    //println!("{:?}", memory);
    //println!("{:#?}", memory);
    part1(&memory, forealz);
    part2(&memory, forealz);
}

fn part1(memory: &Vec<usize>, forealz: bool) {
    let _pos1 = memory[1];
    let _pos2 = memory[2];
    let final_state= if forealz {
        run(memory, 12, 2)
    } else {
        run(memory, _pos1, _pos2)
    };
    if !forealz {
        println!("memory: {:?}", final_state);
    }
    println!("Part 1 answer: {}", final_state[0])
}

fn part2(memory: &Vec<usize>, forealz: bool) {
    for pos1 in 0..100 {
        for pos2 in 0..100 {
            let result = run(memory, pos1, pos2);
            if result[0] == 19690720 {
                if !forealz {
                    println!("memory: {:?}", result);
                }
                println!("Part 2 answer: {}", pos1 * 100 + pos2)
            }

        }
    }
}

fn run(original_memory: &Vec<usize>, pos1: usize, pos2: usize) -> Vec<usize> {
    let mut memory = original_memory.clone();
    memory[1] = pos1;
    memory[2] = pos2;

    let mut iptr = 0;
    loop {
        match memory[iptr] {
            1 => {
                let dest = memory[iptr + 3];
                memory[dest] = memory[memory[iptr + 1]] + memory[memory[iptr + 2]];
                iptr += 4;
            }
            2 => {
                let dest = memory[iptr + 3];
                memory[dest] = memory[memory[iptr + 1]] * memory[memory[iptr + 2]];
                iptr += 4;
            }
            99 => {
                break;
            }
            _ => panic!()
        }
    }
    return memory;
}