use std::fmt;
use std::str;

#[derive(Debug)]
pub struct Tree {
    root: Link,
}

#[derive(Debug, Clone)]
struct Node {
    value: i32,
    left: Link,
    right: Link,
}

type Link = Option<Box<Node>>;

impl Tree {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn max(&self) -> i32 {
        match &self.root {
            None => {
                eprintln!("Tree is empty!");
                return -1;
            }
            Some(node) => Node::max(*node.to_owned()),
        }
    }

    pub fn add(&mut self, value: i32) {
        match self.root.take() {
            None => {
                let new_node = Box::new(Node {
                    value,
                    left: None,
                    right: None,
                });
                self.root = Some(new_node);
            }
            Some(mut node) => {
                node.add_to(value);
                self.root = Some(node);
            }
        }
    }

    pub fn depth(&self) -> i32 {
        match &self.root {
            None => 0,
            Some(node) => Node::depth_of(*node.to_owned()),
        }
    }
}

impl Default for Tree {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let btree_depth = self.depth() as usize;
        let mut btree_vec: Vec<Vec<u8>> = Vec::with_capacity(btree_depth);
        let max_digits = (self.max().checked_ilog10().unwrap_or(0) + 1) as usize;
        let len_of_layer = vec![32; 2_usize.pow(btree_depth as u32) * max_digits];
        for _i in 0..btree_depth {
            btree_vec.push(len_of_layer.clone());
        }
        match &self.root {
            None => eprintln!("Empty tree"),
            Some(node) => {
                Node::print(*node.to_owned(), &mut btree_vec, 0, btree_depth, 1);
            }
        }

        for str in btree_vec {
            write!(f, "{}\n", str::from_utf8(&str).unwrap())?;
        }
        write!(f, "")
    }
}

impl Node {
    fn print(
        node: Node,
        strs: &mut Vec<Vec<u8>>,
        depth: usize,
        btree_depth: usize,
        cur_wide: usize,
    ) {
        let cur_pos = 2_usize.pow(btree_depth as u32) / 2_usize.pow(depth as u32) * cur_wide;
        let value_to_copy = node.value.to_string();
        let max_digits = value_to_copy.as_bytes().len();
        strs[depth][cur_pos..cur_pos + max_digits].copy_from_slice(value_to_copy.as_bytes());

        match node.left {
            None => {}
            Some(node) => {
                Node::print(*node, strs, depth + 1, btree_depth, cur_wide * 2 - 1);
            }
        }

        match node.right {
            None => {}
            Some(node) => {
                Node::print(*node, strs, depth + 1, btree_depth, cur_wide * 2 + 1);
            }
        }
    }

    fn max(node: Node) -> i32 {
        match node.right {
            None => node.value,
            Some(node) => Node::max(*node),
        }
    }

    fn depth_of(node: Node) -> i32 {
        let ldepth = match node.left {
            None => 0,
            Some(node) => Node::depth_of(*node),
        };
        let rdepth = match node.right {
            None => 0,
            Some(node) => Node::depth_of(*node),
        };
        if ldepth > rdepth {
            ldepth + 1
        } else {
            rdepth + 1
        }
    }

    fn add_to(&mut self, value: i32) {
        if value < self.value {
            match self.left.take() {
                None => {
                    let new_node = Box::new(Node {
                        value,
                        left: None,
                        right: None,
                    });
                    self.left = Some(new_node);
                }
                Some(node) => {
                    self.left = Some(node);
                    self.left.as_mut().unwrap().add_to(value);
                }
            }
        } else {
            match self.right.take() {
                None => {
                    let new_node = Box::new(Node {
                        value,
                        left: None,
                        right: None,
                    });
                    self.right = Some(new_node);
                }
                Some(node) => {
                    self.right = Some(node);
                    self.right.as_mut().unwrap().add_to(value);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tree;
    #[test]
    fn basics() {
        let mut btree = Tree::new();
        println!("{:?}", btree);
        btree.add(5);
        println!("{:?}", btree);
        btree.add(1);
        println!("{:?}", btree);
        btree.add(6);
        println!("{:?}", btree);

        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn depth() {
        let mut btree = Tree::new();
        btree.add(9);
        btree.add(8);
        btree.add(7);
        btree.add(11);
        btree.add(12);
        btree.add(13);
        let btree_depth = btree.depth();

        assert_eq!(btree_depth, 4);
    }
}
