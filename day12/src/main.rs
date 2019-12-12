use itertools::Itertools;

fn main() {
    /*
    let mut moons = vec![
        Moon::new( 0, -1,  0,  2),
        Moon::new( 1,  2, -10, -7),
        Moon::new( 2,  4, -8,  8),
        Moon::new( 3,  3,  5,  -1)
    ];
    */
    /*
<x=  -3, y= 15, z= -11>
<x=   3, y= 13, z= -19>
<x= -13, y= 18, z=  -2>
<x=   6, y=  0, z=  -1>
    */
    let mut moons = vec![
        Moon::new( 0,  -3,  15,  -11),
        Moon::new( 1,   3,  13,  -19),
        Moon::new( 2, -13,  18,   -2),
        Moon::new( 3,   6,   0,   -1)
    ];

    println!("Step 0!");
    for moon in &moons {
        println!("  {:?}", moon);
    }

    for step in 1..=1000 {
        println!("Step {}!", step);
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
    }

    let total_energy = moons.iter().fold(0, |acc, moon| acc + moon.energy());
    println!("Part1: {:?}", total_energy);
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