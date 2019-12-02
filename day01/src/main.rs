use std::fs;


fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error reading input");

    let mut part1_total = 0;
    let mut part2_total = 0;
    for line in contents.lines() {
        let module_size : i32 = line.parse().unwrap();
        if module_size <= 0 {
            continue;
        }

        let mut module_fuel = 0;
        let mut additional:i32 = (module_size / 3) - 2;

        part1_total += additional;
        while additional > 0 {
            module_fuel += additional;
            additional = (additional / 3) - 2;
        }
        part2_total += module_fuel;
    }
    println!("Part1: {}", part1_total);
    println!("Part2: {}", part2_total);

}
