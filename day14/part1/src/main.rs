use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Recipe {
    material: String,
    quantity: i64,
}

fn parse_value(val: String) -> Recipe {
    let split: Vec<String> = val
        .split_whitespace()
        .map(|item| item.to_string())
        .collect();

    let material: String = split.last().unwrap().clone();
    let quantity: i64 = split.first().unwrap().clone().parse().unwrap();

    return Recipe {
        material: material,
        quantity: quantity,
    };
}

fn tokenize(filepath: &String) -> HashMap<String, (i64, Vec<Recipe>)> {
    let contents = fs::read_to_string(filepath).expect("Error reading file");
    let lines: Vec<&str> = contents.split('\n').collect();
    let mut recipes: HashMap<String, (i64, Vec<Recipe>)> = HashMap::new();

    for line in lines {
        let split: Vec<&str> = line.split("=>").map(|item| item.trim()).collect();
        let rhs = split.last().unwrap().to_string();
        let key = parse_value(rhs);

        let lhs: Vec<String> = split[0]
            .split(',')
            .map(|item| item.trim().to_string())
            .collect();

        let mut needed_materials: Vec<Recipe> = Vec::new();

        for material in lhs {
            needed_materials.push(parse_value(material));
        }

        recipes.insert(key.material.clone(), (key.quantity, needed_materials));
    }

    return recipes;
}

fn solve(recipes: HashMap<String, (i64, Vec<Recipe>)>) -> i64 {
    let mut supply: HashMap<String, i64> = HashMap::new();
    let mut orders: VecDeque<Recipe> = VecDeque::new();
    let mut ore_needed = 0;
    orders.push_back(Recipe {
        material: "FUEL".to_string(),
        quantity: 1,
    });

    while !orders.is_empty() {
        let order = orders.pop_front().unwrap();
        let is_in_supply = supply.get(&order.material) != None;

        if order.material == "ORE" {
            ore_needed += order.quantity;
        } else if is_in_supply && order.quantity <= *supply.get(&order.material).unwrap() {
            *supply.get_mut(&order.material).unwrap() -= order.quantity;
        } else {
            let amount = if is_in_supply {
                order.quantity - *supply.get(&order.material).unwrap()
            } else {
                order.quantity
            };
            let recipe = recipes.get(&order.material).unwrap();
            let batches = f64::ceil(amount as f64 / recipe.0 as f64) as i64;

            for ingredient in &recipe.1 {
                orders.push_back(Recipe {
                    material: ingredient.material.clone(),
                    quantity: ingredient.quantity * batches,
                });
            }

            let leftover = batches * recipe.0 - amount;
            supply.insert(order.material.clone(), leftover);
        }
    }

    return ore_needed;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Provide filename");
        return;
    }
    let filepath = &args[1];
    let materials = tokenize(filepath);
    let result = solve(materials);
    println!("{}", result);
}
