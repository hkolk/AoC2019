use std::fs;
use std::f64;

fn main() {
    let input = "input.txt";
    let contents = fs::read_to_string(input).expect("Error reading input");
    let lines = contents.lines().map(|line| {
        line.chars().map(|loc| loc == '#').collect()
    }).collect::<Vec<Vec<_>>>();

    let mut asteroids = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, position) in line.iter().enumerate() {
            if *position {
                asteroids.push(Coord{ y: y as isize, x: x as isize });
            }
        }
    }

    let mut best_asteroid_count = 0;
    let mut best_asteroid = &Coord {y: -1, x: -1};

    for asteroid in &asteroids {
        let mut visible_asteroids = -1; // I will find myself as wel...
        for other_asteroid in &asteroids {
            let path = asteroid.path(&other_asteroid);
            let mut colission = false;
            for path_coord in path {
                if lines[path_coord.y as usize][path_coord.x as usize] {
                    // colission!
                    colission = true;
                }
            }
            if !colission {
                visible_asteroids += 1;
            }
        }
        //println!("Asteroid at {:?} can see {:?} other asteroids!", asteroid, visible_asteroids);
        if visible_asteroids > best_asteroid_count {
            best_asteroid = asteroid;
            best_asteroid_count = visible_asteroids;
        }
    }
    println!("[Part1] !! Best asteroid at {:?} can see {:?} other asteroids!", best_asteroid, best_asteroid_count);

    let station = best_asteroid.clone();
    let mut visible = visible_asteroids(&lines, &asteroids, &station);
    //println!("Double check: {:?}", visible.len());

    visible.sort_by(|a, b | station.angle(a).partial_cmp(&station.angle(b)).unwrap());
    //for roid in &visible {
        //println!("[Station: {:?}] Roid {:?} has angle {:?}", station, roid, station.angle(roid))
    //}
    // cheating because I have 284 roids visible
    println!("[Part2] Roid 200 = {:?}, code: {:?}", &visible[199], &visible[199].x * 100 + &visible[199].y);
}

fn angle(x: isize, y: isize) -> f64 {
    let x: f64 = x as f64;
    let y: f64 = y as f64;
    let atan2 = x.atan2(y);
    return (atan2 - f64::consts::PI).abs();
}

fn visible_asteroids(map: &Vec<Vec<bool>>, all_asteroids: &Vec<Coord>, from :&Coord) -> Vec<Coord> {
    let mut visible_asteroids: Vec<Coord> = Vec::new();
    for other_asteroid in all_asteroids {
        if other_asteroid == from {
            // let's not nuke myself
            continue;
        }
        let path = from.path(&other_asteroid);
        let mut colission = false;
        for path_coord in path {
            if map[path_coord.y as usize][path_coord.x as usize] {
                // colission!
                colission = true;
            }
        }
        if !colission {
            visible_asteroids.push(other_asteroid.clone());
        }
    }
    return visible_asteroids;
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Coord {
    x: isize,
    y: isize
}

impl Coord {
    fn path(&self, other: &Coord) -> Vec<Coord> {
        let mut path = Vec::new();
        let dist_x = other.x - self.x;
        let dist_y = other.y - self.y;
        let gcd = gcd(dist_x, dist_y) as isize;
        if gcd == 0 {
            return path;
        }
        let reduced_x = dist_x / gcd;
        let reduced_y = dist_y / gcd;

        //println!("{:?} {:?} {:?} {:?} {:?}", dist_x, dist_y, gcd, reduced_x, reduced_y);
        for step in 1..gcd {
            //println!("{:?}", step);
            path.push(Coord{
                x:self.x + (reduced_x * step),
                y:self.y + (reduced_y * step),
            })
        }
        return path;
    }

    fn angle(&self, other: &Coord) -> f64 {
        return angle(
            other.x - self.x,
            other.y - self.y
        );
    }
}

fn gcd(x: isize, y: isize) -> usize {
    let mut x = x.abs() as usize;
    let mut y = y.abs() as usize;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}