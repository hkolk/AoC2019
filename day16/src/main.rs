use std::fs;
use std::collections::VecDeque;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error reading input");
    let input = contents.chars().map(|c| c.to_digit(10).unwrap() as isize).collect::<Vec<_>>();
    part1(&input);
}
fn part1(input_original: &Vec<isize>) {
    let input = input_original.clone();
    //println!("{:?}", input);

    let mut mutating_input = input.clone();
    for phase in 0..100 {
        // regular algorithm
        let mut output: Vec<isize> = Vec::new();
        for outer_index in 0..mutating_input.len() {
            let mut accu = 0;
            for inner_index in 0..mutating_input.len() {
                let mutator = mutator_for_pos(inner_index, outer_index);
                let full_value = mutator * mutating_input[inner_index] as isize;
                //println!("Index-norm: {:?}-{:?}-{:?}, mutator: {:?}, full_value: {:?}", phase, outer_index, inner_index, mutator, full_value);
                accu += full_value;
            }
            let result = accu.abs() % 10;
            //println!("norm: {:?}-{:?}: {:?} (from {:?})", phase, outer_index, result, accu);
            output.push(result);
        }
        // fast algorithm
        let mut output_fast: Vec<isize> = Vec::new();
        let mut output_predicted: Vec<isize> = Vec::new();
        let mut prevaccu = 0;
        for outer_index in (0..mutating_input.len()).rev() {
            let mut accu = 0;
            for inner_index in ((outer_index)..mutating_input.len()).rev() {
                let mutator = mutator_for_pos(inner_index, outer_index);
                let full_value = mutator * mutating_input[inner_index] as isize;
                //println!("Index-fast: {:?}-{:?}-{:?}, mutator: {:?}, full_value: {:?}", phase, outer_index, inner_index, mutator, full_value);
                accu += full_value;
            }
            let result = accu.abs() % 10;
            let current = (mutating_input[outer_index] * mutator_for_pos(outer_index, outer_index));
            prevaccu += current;
            //println!("fast: {:?}-{:?}: {:?} (from {:?}, predicted: {:?}) from {:?} + {:?}, prevaccu: {:?}, full: {:?}", phase, outer_index, result, accu, (output_fast.iter().sum::<isize>() + current), output_fast.iter().sum::<isize>(), current, prevaccu, output_fast.iter());
            //println!("fast: {:?}-{:?}: {:?} (from {:?}, predicted: {:?})", phase, outer_index, result, accu, prevaccu);
            output_fast.push(result);
            output_predicted.push(prevaccu % 10);

        }
        //println!("Phase {:?}, output: {:?}", phase, output);
        mutating_input = output.clone();
        println!("Phase norm      {:2}: {:?}", phase, output.into_iter().map(|d| d.to_string()).collect::<Vec<String>>().join(""));
        //println!("Phase fast      {:2}: {:?}", phase, output_fast.into_iter().rev().map(|d| d.to_string()).collect::<Vec<String>>().join(""));
        //println!("Phase predicted {:2}: {:?}", phase, output_predicted.into_iter().rev().map(|d| d.to_string()).collect::<Vec<String>>().join(""));

    }
    println!("Part 1: {:?}", mutating_input.into_iter().take(8).map(|d| d.to_string()).collect::<Vec<String>>().join(""));
}

fn part2(input_original: &Vec<isize>) {
    let input = input_original.clone();
    //println!("{:?}", input);

    let mut mutating_input = input.clone();
    for phase in 0..100 {
        // regular algorithm
        let mut output: Vec<isize> = Vec::new();
        for outer_index in 0..mutating_input.len() {
            let mut accu = 0;
            for inner_index in 0..mutating_input.len() {
                let mutator = mutator_for_pos(inner_index, outer_index);
                let full_value = mutator * mutating_input[inner_index] as isize;
                //println!("Index-norm: {:?}-{:?}-{:?}, mutator: {:?}, full_value: {:?}", phase, outer_index, inner_index, mutator, full_value);
                accu += full_value;
            }
            let result = accu.abs() % 10;
            //println!("norm: {:?}-{:?}: {:?} (from {:?})", phase, outer_index, result, accu);
            output.push(result);
        }
        // fast algorithm
        let mut output_fast: Vec<isize> = Vec::new();
        let mut output_predicted: Vec<isize> = Vec::new();
        let mut prevaccu = 0;
        for outer_index in (0..mutating_input.len()).rev() {
            let mut accu = 0;
            for inner_index in ((outer_index)..mutating_input.len()).rev() {
                let mutator = mutator_for_pos(inner_index, outer_index);
                let full_value = mutator * mutating_input[inner_index] as isize;
                //println!("Index-fast: {:?}-{:?}-{:?}, mutator: {:?}, full_value: {:?}", phase, outer_index, inner_index, mutator, full_value);
                accu += full_value;
            }
            let result = accu.abs() % 10;
            let current = (mutating_input[outer_index] * mutator_for_pos(outer_index, outer_index));
            prevaccu += current;
            //println!("fast: {:?}-{:?}: {:?} (from {:?}, predicted: {:?}) from {:?} + {:?}, prevaccu: {:?}, full: {:?}", phase, outer_index, result, accu, (output_fast.iter().sum::<isize>() + current), output_fast.iter().sum::<isize>(), current, prevaccu, output_fast.iter());
            //println!("fast: {:?}-{:?}: {:?} (from {:?}, predicted: {:?})", phase, outer_index, result, accu, prevaccu);
            output_fast.push(result);
            output_predicted.push(prevaccu % 10);

        }
        //println!("Phase {:?}, output: {:?}", phase, output);
        mutating_input = output.clone();
        println!("Phase norm      {:2}: {:?}", phase, output.into_iter().map(|d| d.to_string()).collect::<Vec<String>>().join(""));
        //println!("Phase fast      {:2}: {:?}", phase, output_fast.into_iter().rev().map(|d| d.to_string()).collect::<Vec<String>>().join(""));
        //println!("Phase predicted {:2}: {:?}", phase, output_predicted.into_iter().rev().map(|d| d.to_string()).collect::<Vec<String>>().join(""));

    }
    println!("Part 2: {:?}", mutating_input.into_iter().take(8).map(|d| d.to_string()).collect::<Vec<String>>().join(""));
}

fn mutator_for_pos(pos: usize, numeration: usize) -> isize {
    let base_pattern = vec![0, 1, 0, -1];
    let index = ((pos + 1) / (numeration + 1)) % 4;
    return base_pattern[index];
}

#[test]
fn test_mutator_for_pos() {
    assert_eq!(mutator_for_pos(0, 0), 1);
    assert_eq!(mutator_for_pos(1, 0), 0);
    assert_eq!(mutator_for_pos(2, 0), -1);
    assert_eq!(mutator_for_pos(3, 0), 0);
    assert_eq!(mutator_for_pos(4, 0), 1);
    assert_eq!(mutator_for_pos(5, 0), 0);

    assert_eq!(mutator_for_pos(0, 1), 0);
    assert_eq!(mutator_for_pos(1, 1), 1);
    assert_eq!(mutator_for_pos(2, 1), 1);
    assert_eq!(mutator_for_pos(3, 1), 0);
    assert_eq!(mutator_for_pos(4, 1), 0);
    assert_eq!(mutator_for_pos(5, 1), -1);

    assert_eq!(mutator_for_pos(0, 2), 0);
    assert_eq!(mutator_for_pos(1, 2), 0);
    assert_eq!(mutator_for_pos(2, 2), 1);
    assert_eq!(mutator_for_pos(3, 2), 1);
    assert_eq!(mutator_for_pos(4, 2), 1);
    assert_eq!(mutator_for_pos(5, 2), 0);

}