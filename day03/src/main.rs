use std::fs;
use std::collections::HashSet;

fn main() {
    let input = "input.txt";
    let contents = fs::read_to_string(input).expect("Error reading input");
    let lines:Vec<&str> = contents.lines().collect();
    let line1 = to_line(lines[0]);
    let line2 = to_line(lines[1]);

    let line1_coords = trace_line(line1);
    let line2_coords = trace_line(line2);
    //println!("{:?}", line1_coords);
    //println!("{:?}", line2_coords);
    let line1_hashset : HashSet<Coord> = line1_coords.iter().cloned().collect();
    let line2_hashset : HashSet<Coord> = line2_coords.iter().cloned().collect();
    let mut closest_distance = std::i32::MAX;
    let mut lowest_steps = std::usize::MAX;
    for coord in line1_hashset.intersection(&line2_hashset) {
        let distance = coord.x.abs() + coord.y.abs();
        if distance < closest_distance {
            closest_distance = distance
        }
        let steps = find_steps_for_coord(&line1_coords, coord) + find_steps_for_coord(&line2_coords, coord) + 2;
        if steps < lowest_steps {
            lowest_steps = steps;
        }
        //println!("Intersection at: {:?}, distance: {:?}, steps: {:?}", coord, distance, steps);
    }
    println!("Part 1: {:?}", closest_distance);
    println!("Part 2: {:?}", lowest_steps);
}

fn find_steps_for_coord(input: &Vec<Coord>, coord: &Coord) -> usize {
    for i in 0..input.len() {
        if input[i] == *coord{
            return i;
        }
    }
    return std::usize::MAX;
}

fn trace_line(moves: Vec<Move>) -> Vec<Coord> {
    let mut collector = Vec::new();
    let mut coord = Coord{x: 0, y: 0};
    for mv in moves {
        //println!("Making move: {:?}", mv);
        for _steps in 0..mv.steps {
            coord = match mv.direction {
                Direction::Left   => Coord{x: coord.x - 1, y: coord.y},
                Direction::Right  => Coord{x: coord.x + 1, y: coord.y},
                Direction::Up     => Coord{x: coord.x, y: coord.y + 1},
                Direction::Down   => Coord{x: coord.x, y: coord.y - 1}
            };
            //println!("{:?}", coord);
            collector.push(coord.clone());
        }
    }
    return collector;
}

fn to_line(line: &str) -> Vec<Move> {
    return line.split(",").map(|item: &str|
        to_move(item)
    ).collect::<Vec<Move>>();
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
struct Coord {
    x: i32,
    y: i32
}

#[derive(Debug)]
#[derive(Clone)]
struct Move {
    direction: Direction,
    steps: u32

}

#[derive(Debug)]
#[derive(Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

fn to_move(input: &str) -> Move {
    let move_length = String::from(input.get(1..).unwrap()).parse::<u32>().unwrap();
    match input.get(0..1).unwrap() {
        "L" => Move{direction: Direction::Left, steps: move_length},
        "R" => Move{direction: Direction::Right, steps: move_length},
        "U" => Move{direction: Direction::Up, steps: move_length},
        "D" => Move{direction: Direction::Down, steps: move_length},
        _ => panic!("No valid move")
    }
}