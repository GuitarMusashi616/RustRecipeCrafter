mod parse;
mod uncraft;
mod backpack;

use uncraft::Uncrafter;
use backpack::Backpack;

fn main() {

    let inventory = Backpack::from([
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

    let mut uc = Uncrafter::new();
    uc.execute();
    // uc.uncraft("machine frame");
    // println!("{:?}", uc.recipes_that_exist);
    // println!("{:?}", uc.raw_materials_for_recipe);

    // for each recipe and inventory item uncraft them
}


