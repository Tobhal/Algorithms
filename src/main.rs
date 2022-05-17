use crate::unbalanced::tree::b_tree::BTree;
use crate::utils::util::Node;

mod unbalanced;
mod utils;

fn main() {
    let root = BTree {..Default::default()};

    println!("{}", root.root.data)
}
