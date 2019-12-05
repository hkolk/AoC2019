
fn main() {
    let min = 172930;
    let max = 683082;
    let mut part1 = 0;
    let mut part2 = 0;

    for test in min..max {
        let digits = (0..6).rev().map(|range| (test / 10_i32.pow(range)) % 10).collect::<Vec<_>>();
        // six digits => always true
        // in range => always true
        // 2 adjacent digits
        let consecutive = test_consecutive_same(&digits);
        // always accending:
        let acc = test_accending(&digits);
        // only 2 adjecent
        let consecutive_constraint = test_consecutive_constraint(&digits);


        if acc && consecutive {
            println!("{:?}: {:?}, consecutive: {:?}, acc: {:?}, constraint: {:?}", test, digits, consecutive, acc, consecutive_constraint);
            part1 += 1;
            if consecutive_constraint {
                part2 += 1;
            }
        }
    }
    println!("Part1: {:?}", part1);
    println!("Part2: {:?}", part2);
}

fn test_consecutive_same(subject : &Vec<i32>) -> bool {
    return (1..6).fold(false, |acc, index|
        acc || subject[index] == subject[index-1]
    );
}

fn count_item_in(subject: &Vec<i32>, item: i32) -> u32 {
    return (subject).iter().fold(0, |acc, x| if *x == item { acc + 1} else { acc });
}

fn test_consecutive_constraint(subject : &Vec<i32>) -> bool {
    return (1..6).fold(false, |acc, index|
        return acc || (subject[index] == subject[index - 1] && count_item_in(subject, (subject[index])) == 2)
    );
}

fn test_accending(subject : &Vec<i32>) -> bool {
    let mut prev_digit = 0;
    for next_digit in subject {
        if prev_digit > *next_digit {
            return false
        } else {
            prev_digit = *next_digit;
        }
    }
    return true;
}