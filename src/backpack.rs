// can add or remove [string] from backpack
// will keep count of how many of that [string] it has
use std::collections::HashMap;
use std::ops::AddAssign;

#[derive(Debug)]
pub struct Backpack {
    contents: HashMap<String, u32>,
}

impl Backpack {
    pub fn new() -> Self {
        let contents = HashMap::new();
        Backpack {
            contents,
        }
    }

    pub fn add_multiple(&mut self, item: &str, amount: u32) {
        if !self.contents.contains_key(item) {
            self.contents.insert(item.to_string(), 0);
        }
        let count = self.contents.get_mut(item).unwrap();
        *count += amount;
    }

    pub fn add(&mut self, item: &str) {
        self.add_multiple(item, 1);
    }
}

impl AddAssign for Backpack {
    fn add_assign(&mut self, rhs: Self) {
        rhs.contents.iter().for_each(|(item,amount)|self.add_multiple(item, *amount))
    }
}

impl<const N: usize> From<[(&str, u32); N]> for Backpack {
    fn from(arr: [(&str, u32); N]) -> Self {
        let mut backpack = Backpack::new();
        arr.iter().for_each(|(item_name, amount)|backpack.add_multiple(item_name, *amount));
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