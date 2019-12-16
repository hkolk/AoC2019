use std::fs;
use std::collections::VecDeque;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error reading input");
    let input = contents.chars().map(|c| c.to_digit(10).unwrap() as isize).collect::<Vec<_>>();
    let base_pattern = vec![0, 1, 0, -1];
    //println!("{:?}", input);

    let mut mutating_input = input.clone();
    for phase in 0..100 {
        let mut output: Vec<isize> = Vec::new();
        for outer_index in 0..mutating_input.len() {
            let applied_pattern = base_pattern.clone().into_iter().flat_map(|digit| vec![digit; outer_index + 1]).collect::<Vec<_>>();
            //println!("{:?}", applied_pattern);
            let mut accu = 0;
            for inner_index in 0..mutating_input.len() {
                let mutator = applied_pattern[(inner_index + 1) % applied_pattern.len()];
                let full_value = mutator * mutating_input[inner_index] as isize;
                //println!("Index: {:?}, mutator: {:?}, full_value: {:?}", inner_index, mutator, full_value);
                accu += full_value;
            }
            let result = accu.abs() % 10;
            //println!("Result for outer_index {:?}: {:?} (from {:?})", outer_index, result, accu);
            output.push(result);
        }
        //println!("Phase {:?}, output: {:?}", phase, output);
        mutating_input = output.clone();
    }
    println!("Part 1: {:?}", mutating_input.into_iter().take(8).map(|d| d.to_string()).collect::<Vec<String>>().join(""));
}