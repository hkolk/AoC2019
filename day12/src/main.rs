use itertools::Itertools;
use std::hash::Hash;

fn main() {
/*
    let mut moons = vec![
        Moon::new( 0, -1,  0,  2),
        Moon::new( 1,  2, -10, -7),
        Moon::new( 2,  4, -8,  8),
        Moon::new( 3,  3,  5,  -1)
    ];
    */

    let mut moons = vec![
        Moon::new(0, -3, 15, -11),
        Moon::new(1, 3, 13, -19),
        Moon::new(2, -13, 18, -2),
        Moon::new(3, 6, 0, -1)
    ];
    //*/

    let original_moons = moons.clone();
    let (orig_x, orig_y, orig_z) = get_dimensions(&original_moons);

    println!("Step 0!");
    for moon in &moons {
        println!("  {:?}", moon);
    }

    let (mut x_step, mut y_step, mut z_step) = (0, 0, 0);

    for step in 1..=1000000000000000000_usize {
        //println!("Step {}!", step);
        let ro_moons = moons.clone();
        for moon in moons.iter_mut() {
            for other in ro_moons.iter() {
                if moon.id != other.id {
                    (*moon).apply_gravity(other);
                }
            }
        }

        for moon in moons.iter_mut() {
            (*moon).apply_velocity();
        }
        for moon in &moons {
            //println!("  {:?}", moon);
        }
        if step == 1000 {
            let total_energy = moons.iter().fold(0, |acc, moon| acc + moon.energy());
            println!("Part1: {:?}", total_energy);
        }
        let (dim_x, dim_y, dim_z) = get_dimensions(&moons);
        if x_step == 0 && dim_x == orig_x {
            x_step = step;
            println!("Found x_step: {:?}", x_step);
        }
        if y_step == 0 && dim_y == orig_y {
            y_step = step;
            println!("Found y_step: {:?}", y_step);
        }
        if z_step == 0 && dim_z == orig_z {
            z_step = step;
            println!("Found z_step: {:?}", z_step);
        }
        if x_step > 0 && y_step > 0 && z_step > 0 {
            println!("Part2: {:?}", lcm(lcm(x_step, y_step), z_step));
            break;
        }
        if moons.eq(&original_moons) {
            println!("Part2: {:?}", step);
            break;
        }
    }
}

fn get_dimensions(moons: &Vec<Moon>) -> (Vec<isize>, Vec<isize>, Vec<isize>) {
    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut z = Vec::new();
    for moon in moons {
        x.push(moon.location.x);
        x.push(moon.velocity.x);
        y.push(moon.location.y);
        y.push(moon.velocity.y);
        z.push(moon.location.z);
        z.push(moon.velocity.z);
    }
    return (x, y, z);
}
fn gcd(x: usize, y: usize) -> usize {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Moon {
    id: isize,
    location: Coord,
    velocity: Coord
}

impl Moon {
    fn new(id: isize, x: isize, y: isize, z: isize) -> Moon {
        return Moon {
            id,
            location: Coord{x, y, z},
            velocity: Coord{x:0, y:0, z:0}
        }
    }
    fn apply_gravity(&mut self, other: &Moon) {
        self.velocity.x += calculate_gravity(self.location.x, other.location.x);
        self.velocity.y += calculate_gravity(self.location.y, other.location.y);
        self.velocity.z += calculate_gravity(self.location.z, other.location.z);

    }

    fn apply_velocity(&mut self) {
        self.location.x += self.velocity.x;
        self.location.y += self.velocity.y;
        self.location.z += self.velocity.z;
    }

    fn energy(&self) -> isize {
        return self.location.sum_abs() * self.velocity.sum_abs();
    }

    fn hash_x(&self) -> isize {
        return self.location.x * 100000000 + self.velocity.x;
    }
    fn hash_y(&self) -> isize {
        return self.location.y * 100000000 + self.velocity.y;
    }
    fn hash_z(&self) -> isize {
        return self.location.z * 100000000 + self.velocity.z;
    }
}
fn calculate_gravity(this: isize,  other: isize) -> isize {
    if this == other {
        return 0
    } else if this < other {
        return 1
    } else {
        return -1
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Coord {
    x: isize,
    y: isize,
    z: isize
}

impl Coord {
    fn sum_abs(&self) -> isize {
        return self.x.abs() + self.y.abs() + self.z.abs();
    }
}