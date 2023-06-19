use binary_tree::*;

fn main() {
    let mut btree = Tree::new();
    btree.add(10);
    btree.add(15);
    btree.add(5);
    btree.add(13);
    btree.add(17);
    btree.add(18);
    btree.add(16);
    btree.add(12);
    btree.add(14);
    btree.add(11);
    btree.add(3);
    btree.add(7);
    btree.add(2);
    btree.add(4);
    btree.add(1);
    btree.add(6);
    btree.add(8);
    btree.add(9);
    println!("{}", btree);
    btree.remove(15);
    for _i in 0..10000000 {
        btree.add(11);
        btree.remove(11);
    }
    println!("{}", btree);
}
