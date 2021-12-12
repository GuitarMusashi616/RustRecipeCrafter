use std::collections::{HashSet, HashMap};
use crate::backpack::Backpack;
use crate::parse::{csv_to_manual_struct, Pattern};


pub struct Uncrafter {
    recipes_map: HashMap<String, Pattern>,
    recipes_that_exist: HashSet<String>,
    raw_materials_for_recipe: HashMap<String, Backpack>,
}

impl Uncrafter {
    pub fn new() -> Uncrafter {
        // create the recipe exist set and init the hash map
        let recipes_vec = csv_to_manual_struct();

        let recipes_that_exist: HashSet<String> = recipes_vec
            .iter()
            .map(|x|x.name.to_string())
            .collect();

        let recipes_map: HashMap<String, Pattern> = recipes_vec.into_iter().map(|pattern|((&pattern.name).to_string(), pattern)).collect();
        println!("{:?}", recipes_map);

        let raw_materials_for_recipe: HashMap<String, Backpack> = HashMap::new();

        Uncrafter {
            recipes_map,
            recipes_that_exist,
            raw_materials_for_recipe,
        }
    }

    pub fn uncraft(&self, recipe_name: &str) -> Backpack {
        let recipe = self.recipes_map.get(recipe_name).expect("recipe_name not found");

        let mut raw_materials = Backpack::new();

        for ingredient in recipe.ingredients.iter().filter_map(|x| x.as_ref()) {
            // println!("{}", ingredient);
            if self.recipes_that_exist.contains(ingredient) {
                // uncraft it an add the result
                raw_materials += self.uncraft(ingredient);
                continue
            }
            // otherwise add the ingredient to the backpack
            raw_materials.add(ingredient);
        }
        raw_materials
    }

    pub fn execute(&mut self) {
        for recipe_name in self.recipes_map.keys() {
            self.raw_materials_for_recipe.insert(recipe_name.to_string(),self.uncraft(recipe_name));
        }
        for (recipe_name, backpack) in self.raw_materials_for_recipe.iter(){
            println!("{:?}", recipe_name);
            println!("{:?}", backpack);
            println!("\n");
        }


    }
}