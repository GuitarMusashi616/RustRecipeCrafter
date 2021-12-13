mod parse;
mod uncraft;
mod backpack;

use uncraft::Uncrafter;
use backpack::Backpack;
use std::io::stdin;

fn main() {
    let inventory = Backpack::from([
        ("flint", 10),
        ("iron ingot", 5),
        ("copper gear", 1),
        ("copper ingot", 10),
        ("gold ingot", 3),
        ("redstone", 5),
        ("cobblestone", 64),
        ("wooden planks", 64),
        ("glass", 5),
    ]);
    let mut uc = Uncrafter::new();
    uc.check_recipes(inventory);
    stdin().read_line(&mut "stuff".to_string());
}


