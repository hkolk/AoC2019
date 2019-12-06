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
    parent: String
}

fn to_node(input: &str) -> Node {
    let parts : Vec<&str> = input.split(")").collect();
    return Node{name: parts[1].to_string(), parent: parts[0].to_string()}

}