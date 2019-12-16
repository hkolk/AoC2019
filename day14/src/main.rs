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
    //println!("{:#?}", reactions.entry("FUEL".to_string()));

    part1(&reactions);
    part2(&reactions);
    /*
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
    }*/

}

fn part1(reactions: &HashMap<String, Reaction>) {
    println!("Part 1: Ore used: {:?}", ore_per_fuel(reactions, 1));
}

fn ore_per_fuel(reactions: &HashMap<String, Reaction>, fuel_size: usize) -> usize{
    let mut inventory = Inventory::new(&reactions);
    inventory.get_item("FUEL", fuel_size);
    return inventory.ore_counter;
    //println!("{:#?}", inventory.stock);
}

fn part2(reactions: &HashMap<String, Reaction>) {
    let mut max_fuel = 1;
    let approx_fuel = 1_000_000_000_000 / ore_per_fuel(reactions, 1_000) * 1_000;
    //println!("approx fuel: {:?}", approx_fuel);
    //println!("actual ore consumed: {:?}", ore_per_fuel(reactions, approx_fuel));


    for fuel in approx_fuel..approx_fuel+ 1_000_000 {
        let mut inventory = Inventory::new(&reactions);
        inventory.get_item("FUEL", fuel);
        if inventory.ore_counter > 1_000_000_000_000 {
            max_fuel = fuel;
            break;
        }
        //println!{"{:?} requires {:?} ore", fuel, inventory.ore_counter};
    }
    println!("Part 2: Fuel generated: {:?}", max_fuel - 1);
}

struct Inventory {
    reactions: HashMap<String, Reaction>,
    stock: HashMap<String, usize>,
    ore_counter: usize
}

fn int_div_ceil(x: usize, y: usize) -> usize {
    return if x % y > 0  { x / y  + 1 } else { x / y };
}

impl Inventory {
    fn new(reactions: &HashMap<String, Reaction>) -> Inventory {
        // populate initial ore and cheat
        return Inventory{
            reactions: reactions.clone(),
            stock: HashMap::new(),
            ore_counter: 0
        };
    }

    fn get_item(&mut self, item_name: &str, item_quantity: usize) -> bool {
        let items_in_stock = self.stock.get(item_name).unwrap_or(&0);
        // 4 items in stock, 3 items requested => update with 1
        // 2 items in stock, 3 items requested => update with 0, react for remainder
        let remaining = *items_in_stock as isize - item_quantity as isize;
        if remaining < 0 {
            // take everything out of the stock
            // and react the remainder
            let additional_stock = self.react(item_name, remaining.abs() as usize);
            self.stock.insert(item_name.to_string(), (remaining + additional_stock as isize) as usize);
            //println!("- Stock [update]: {:?} x {:?} (remaining: {:?}", item_name, item_quantity, remaining);
            return true;
        } else {
            // take it from the stock, update remaining
            //println!("- Stock [take]: {:?} x {:?} (remaining: {:?}", item_name, item_quantity, remaining);
            self.stock.insert(item_name.to_string(), remaining as usize);
            return true
        }
    }
    fn react(&mut self, item_name: &str, item_quantity: usize) -> usize {
        //println!("- Reacting: {:?} x {:?}", item_name, item_quantity);
        if item_name == "ORE" {
            self.ore_counter += item_quantity;
            return item_quantity
        }
        let reaction = self.reactions.get(item_name).unwrap().clone();
        // need 3, per reaction 2: 2 reactions.
        // need 4, per reaction 2: 2 reactions
        let num_reactions = int_div_ceil(item_quantity, reaction.result.quantity);
        for ingredient in reaction.ingredients {
            // get items from stock
            self.get_item(ingredient.name.as_str(), ingredient.quantity * num_reactions);
        }
        // we have taken what we need from stock, so we can react
        return num_reactions * reaction.result.quantity;

    }
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