use std::fs;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;

fn main() {
    let input = "input.txt";
    let contents = fs::read_to_string(input).expect("Error reading input");

    let mut object_map: HashMap<String, Node, RandomState> = HashMap::new();

    for line in contents.lines() {
        let new_node = to_node(line);
        println!("{:?}", new_node);
        object_map.insert(new_node.name.clone(), new_node);
    }
    println!("{:#?}", object_map);
    let mut total_orbit_count = 0;
    for (_name, node) in &object_map {
        let orbit_count = get_orbits(node, &object_map);
        total_orbit_count += orbit_count;
        println!("Object {:?} has {:?} orbits", node.name, orbit_count)
    }
    println!("Total orbits: {:?}", total_orbit_count);
    let my_path_to_com = path_to_com(&"YOU".to_string(), &object_map);
    let santas_path_to_com = path_to_com(&"SAN".to_string(), &object_map);

    println!("Path from YOU to COM: {:?}", my_path_to_com);
    println!("Path from SAN to COM: {:?}", santas_path_to_com);

    let mut my_jumps = 0;
    let mut min_jumps = 0;
    'outer: for my_step in my_path_to_com {
        my_jumps += 1;
        let mut santa_jumps = 0;
        for santa_step in &santas_path_to_com {
            santa_jumps += 1;
            if my_step == santa_step.clone() {
                min_jumps = my_jumps + santa_jumps - 2;
                break 'outer;
            }
        }
    }
    println!("Minimum jumps between YOU and SAN: {:?}", min_jumps)



}
fn path_to_com(source: &String, object_map:&HashMap<String, Node>) -> Vec<String> {
    let mut accu = Vec::new();
    let mut parent = source;
    while parent.as_str() != "COM" {
        parent = &object_map.get(parent.as_str()).unwrap().parent;
        accu.push(parent.clone());
    }
    return accu;
}

fn get_orbits(source: &Node, object_map:&HashMap<String, Node>) -> usize {
    let mut node = source;
    let mut counter = 1;
    while node.parent.as_str() != "COM" {
        counter += 1;
        node = object_map.get(&node.parent).unwrap()
    }
    return counter;
}

#[derive(Debug)]
#[derive(Clone)]
struct Node {
    name: String,
    parent: String,
}

fn to_node(input: &str) -> Node {
    let parts : Vec<&str> = input.split(")").collect();
    return Node{name: parts[1].to_string(), parent: parts[0].to_string()}

}