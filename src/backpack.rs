// can add or remove [string] from backpack
// will keep count of how many of that [string] it has
use std::collections::HashMap;
use std::ops::AddAssign;

#[derive(Debug)]
pub struct Backpack {
    pub contents: HashMap<String, u32>,
}

impl Backpack {
    pub fn new() -> Self {
        let contents = HashMap::new();
        Backpack {
            contents,
        }
    }

    pub fn add(&mut self, item: &str, amount: u32) {
        if !self.contents.contains_key(item) {
            self.contents.insert(item.to_string(), 0);
        }
        let count = self.contents.get_mut(item).unwrap();
        *count += amount;
    }

    pub fn contains(&self, item: &str, amount: u32) -> bool {
        match self.contents.get(item) {
            Some(count) => *count >= amount,
            None => false,
        }
    }

    pub fn contains_all(&self, backpack: &Backpack) -> bool {
        for (item, amount) in backpack.contents.iter() {
            if !self.contains(item, *amount) {
                return false
            }
        }
        true
    }

    pub fn contains_all_x_times(&self, backpack: &Backpack) -> u32 {
        backpack.contents
            .iter()
            .map(|(item, amount_needed_for_one)| {
                match self.contents.get(item) {
                    Some(amount_owned) => amount_owned / amount_needed_for_one,
                    None => 0,
                }
            })
            .min()
            .unwrap_or(0)
    }
}

impl AddAssign for Backpack {
    fn add_assign(&mut self, rhs: Self) {
        rhs.contents.iter().for_each(|(item,amount)|self.add(item, *amount))
    }
}

impl<const N: usize> From<[(&str, u32); N]> for Backpack {
    fn from(arr: [(&str, u32); N]) -> Self {
        let mut backpack = Backpack::new();
        arr.iter().for_each(|(item_name, amount)|backpack.add(item_name, *amount));
        backpack
    }
}


#[test]
fn backpack_test() {
    let mut bp = Backpack::new();
    bp.add("flint");
    bp.add("glass");
    bp.add("flint");
    bp.add("iron ingot");
    println!("{:?}", bp.contents);
    assert_eq!(HashMap::from([("flint".to_string(),2), ("glass".to_string(),1), ("iron ingot".to_string(),1)]), bp.contents)
}

#[test]
fn add_backpacks_together() {
    let mut bp = Backpack::new();
    bp.add_multiple("iron ingot", 3);
    bp.add_multiple("gold ingot", 3);
    bp.add_multiple("copper ingot", 3);
    bp.add_multiple("redstone", 3);

    let mut op = Backpack::new();
    op.add_multiple("iron ingot", 7);
    op.add_multiple("gold ingot", 2);
    op.add_multiple("glass", 5);
    op.add_multiple("redstone", 8);

    bp += op;
    println!("{:?}", bp.contents);
}