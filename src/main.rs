mod parse;
mod uncraft;
mod backpack;

use uncraft::Uncrafter;
use backpack::Backpack;
use std::io::stdin;

fn main() {
    let inventory = Backpack::from_path("inventory.csv");
    let mut uc = Uncrafter::from_path("recipes.csv");
    uc.check_recipes(inventory);
    stdin().read_line(&mut "stuff".to_string());
}


