use std::collections::HashMap;
use crate::backpack::Backpack;
use crate::parse::{csv_to_manual_struct, Pattern};


pub struct Uncrafter {
    recipes_map: HashMap<String, Pattern>,
    raw_materials_for_recipe: HashMap<String, Backpack>,
}

impl Uncrafter {
    pub fn new(recipes_vec: Vec<Pattern>) -> Self {
        // create the recipe exist set and init the hash map
        let recipes_map: HashMap<String, Pattern> = recipes_vec.into_iter().map(|pattern|((&pattern.name).to_string(), pattern)).collect();

        let raw_materials_for_recipe: HashMap<String, Backpack> = HashMap::new();

        Self {
            recipes_map,
            raw_materials_for_recipe,
        }
    }

    pub fn from_path(path: &str) -> Self {
        let recipes_vec = csv_to_manual_struct(path);
        return Self::new(recipes_vec);
    }

    pub fn uncraft(&self, recipe_name: &str, count: u32) -> Backpack {
        let recipe = self.recipes_map.get(recipe_name).expect("recipe_name not found");

        let mut raw_materials = Backpack::new();

        for ingredient in recipe.ingredients.iter().filter_map(|x| x.as_ref()) {
            // println!("{}", ingredient);
            if self.recipes_map.contains_key(ingredient) {
                // uncraft it an add the result
                raw_materials += self.uncraft(ingredient, count);
                continue
            }
            // otherwise add the ingredient to the backpack
            raw_materials.add(ingredient, count);
        }
        raw_materials
    }

    pub fn initialize_recipes(&mut self) {
        // calculates the raw ingredients required for each recipe
        for recipe_name in self.recipes_map.keys() {
            self.raw_materials_for_recipe.insert(recipe_name.to_string(),self.uncraft(recipe_name, 1));
        }
        println!("Recipe Contents: ");
        for (recipe_name, backpack) in self.raw_materials_for_recipe.iter(){
            println!("{:?}", recipe_name);
            println!("{:?}\n", backpack);
        }
    }

    pub fn uncraft_items_in(&self, inventory: Backpack) -> Backpack{
        // return backpack of uncrafted inventory
        let mut raw_inventory = Backpack::new();
        for (item_name, count) in &inventory.contents {
            if self.recipes_map.contains_key(item_name) {
                raw_inventory += self.uncraft(item_name, *count);
                continue
            }
            raw_inventory.add(item_name, *count);
        }
        raw_inventory
    }

    pub fn check_recipes(&mut self, inventory: Backpack) {
        // given an inventory of items, returns which recipes can be crafted
        self.initialize_recipes();
        let uncrafted_inventory = self.uncraft_items_in(inventory);
        println!("Craftable Recipes: ");
        for (recipe_name, ingredients) in &self.raw_materials_for_recipe {
            let num = uncrafted_inventory.contains_all_x_times(ingredients);
            if num > 0 {
                println!("{}x {}", num, recipe_name);
            }
        }
        println!();
    }
}