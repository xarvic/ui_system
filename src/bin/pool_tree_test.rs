#![feature(type_name_of_val)]
use ui_system::pool_tree::PoolTree;
use std::fmt::{Display, Formatter, Error};

struct Droping(&'static str);

impl Display for Droping{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(self.0)
    }
}

impl Drop for Droping {
    fn drop(&mut self) {
        println!("Droping value {}!", self.0);
    }
}


fn main() {
    let mut tree = PoolTree::new(Droping("A"));
    println!("{}", tree);
    let mut node = tree.root_mut();

    node.add_child(Droping("B"));
    node.add_child(Droping("C"));
    node.add_child(Droping("D"));

    if let Some(mut node) = node.to_child(0) {
        println!("In child");
        node.add_child(Droping("E"));
    } else {
        println!("No childs!")
    }

    println!("{}", tree);


}