use std::fs;
use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    let input = "input.txt";
    let contents = fs::read_to_string(input).expect("Error reading input");
    let mut reactions = HashMap::new();
    for line in contents.lines() {
        let recipe = Reaction::new(line);
        let name = recipe.result.name.clone();
        reactions.insert(name, recipe);

    }
    println!("{:#?}", reactions.entry("FUEL".to_string()));

    let mut required = HashMap::new();
    required.insert("FUEL".to_string(), 1_usize);

    loop {
        let new_ingredients = find_ingredients(&required, &reactions);
        println!("{:?}", new_ingredients);
        if new_ingredients.len() == 1 && new_ingredients.contains_key("ORE") {
            break;
        } else {
            required = new_ingredients;
        }
    }
}

fn find_ingredients(shoppinglist: &HashMap<String, usize>,  reactions: &HashMap<String, Reaction>) -> HashMap<String, usize> {
    let mut result = HashMap::new();
    for (item_name, item_quantity) in shoppinglist.iter() {
        if item_name == "ORE" {
            continue;
        }
        let reaction = reactions.get(item_name.as_str()).unwrap();
        // how many?
        let mut num_reactions = (item_quantity / reaction.result.quantity);
        if item_quantity % reaction.result.quantity != 0 {
            num_reactions += 1;
        }
        for ingredient in &reaction.ingredients {
            let entry = result.get(ingredient.name.as_str());
            let new_quantity = match entry {
                Some(existing_quantity) => existing_quantity + (ingredient.quantity * num_reactions),
                None => ingredient.quantity * num_reactions
            };
            result.insert(ingredient.name.clone(), new_quantity);
        }
    }
    return result;
}

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Hash, Clone)]
struct Reaction {
    result: Reagent,
    ingredients: Vec<Reagent>
}

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Hash, Clone)]
struct Reagent {
    name: String,
    quantity: usize
}

impl Reaction {
    fn new(line: &str) -> Reaction {
        let parts = line.split(" => ").collect::<Vec<_>>();
        let result = Reagent::new(parts[1]);
        let ingredients = parts[0].split(", ").map(|item| Reagent::new(item)).collect::<Vec<_>>();
        return Reaction{result, ingredients}
    }
}

impl Reagent {
    fn new(input: &str) -> Reagent {
        let parts = input.split_whitespace().collect::<Vec<_>>();
        return Reagent{name: parts[1].to_string(), quantity: parts[0].parse::<usize>().unwrap()}
    }
}