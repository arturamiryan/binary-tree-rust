use binary_tree::*;

fn main() {
    let mut btree = Tree::new();
    btree.add(5);
    btree.add(3);
    btree.add(2);
    btree.add(4);
    btree.add(7);
    btree.add(8);
    btree.add(9);
    btree.add(6);
    btree.add(1);
    btree.add(100);
    println!("{}", btree);
}
