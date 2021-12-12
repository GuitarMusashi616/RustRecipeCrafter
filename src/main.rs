mod parse;
mod uncraft;
mod backpack;

use std::collections::HashMap;
use uncraft::Uncrafter;

fn main() {

    let inventory = HashMap::from([
        ("flint", 10),
        ("iron ingot", 5),
        ("copper ingot", 10),
        ("gold ingot", 3),
        ("redstone", 5),
        ("cobblestone", 64),
        ("wooden planks", 64),
        ("glass", 5),
    ]);
    println!("{:?}", inventory);

    let tuples = [("one", 1), ("two", 2)];
    let test: HashMap<_,_> = tuples.into_iter().collect();
    println!("{:?}", test);

    let mut uc = Uncrafter::new();
    uc.execute();
    // uc.uncraft("machine frame");
    // println!("{:?}", uc.recipes_that_exist);
    // println!("{:?}", uc.raw_materials_for_recipe);

    // for each recipe and inventory item uncraft them
}


