use std::fmt;
use std::str;

#[derive(Debug)]
pub struct Tree {
    root: Link,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Node {
    value: i32,
    left: Link,
    right: Link,
}

type Link = Option<Box<Node>>;

impl Tree {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn max(&self) -> Option<i32> {
        match &self.root {
            None => None,
            Some(node) => Some(Node::max(*node.to_owned())),
        }
    }

    pub fn is_empty(&self) -> bool {
        match &self.root {
            None => true,
            Some(_node) => false,
        }
    }

    pub fn find(&self, value: i32) -> Option<Node> {
        match &self.root {
            None => None,
            Some(node) => Node::find(&*node, value),
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

    pub fn remove(&mut self, value: i32) {
        match self.root.take() {
            None => (),
            Some(mut node) => {
                node.remove(value);
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
        if self.is_empty() {
            eprintln!("Error: tree is empty");
            return Err(fmt::Error);
        }
        let btree_depth = self.depth() as usize;
        let mut btree_vec: Vec<Vec<u8>> = Vec::with_capacity(btree_depth);
        let max_digits = (self.max().unwrap().checked_ilog10().unwrap_or(0) + 2) as usize;
        let len_of_layer = vec![32; 2_usize.pow(btree_depth as u32) * max_digits];
        for _i in 0..btree_depth {
            btree_vec.push(len_of_layer.clone());
        }
        match &self.root {
            None => {
                return Err(fmt::Error);
            }
            Some(node) => {
                Node::print(*node.to_owned(), &mut btree_vec, 0, btree_depth, 1);
            }
        }

        for str in btree_vec {
            writeln!(f, "{}", str::from_utf8(&str).unwrap())?;
        }
        Ok(())
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

    fn find(node: &Node, value: i32) -> Option<Node> {
        if node.value == value {
            Some(node.to_owned())
        } else if node.value < value {
            match &node.right {
                None => None,
                Some(node) => Node::find(&*node, value),
            }
        } else {
            match &node.left {
                None => None,
                Some(node) => Node::find(&*node, value),
            }
        }
    }

    fn get_min(node: &Node) -> Node {
        match &node.left {
            None => node.to_owned(),
            Some(nnode) => Node::get_min(&nnode),
        }
    }

    fn remove(&mut self, value: i32) -> bool {
        if value > self.value {
            match self.right.take() {
                None => return false,
                Some(rnode) => {
                    self.right = Some(rnode);
                    let result = self.right.as_mut().unwrap().remove(value);
                    if result {
                        if self.right.as_ref().unwrap().left == None
                            && self.right.as_ref().unwrap().right == None
                        {
                            self.right = None;
                        } else if self.right.as_ref().unwrap().left == None {
                            self.right = self.right.to_owned().unwrap().right;
                        } else if self.right.as_ref().unwrap().right == None {
                            self.right = self.right.to_owned().unwrap().left;
                        } else {
                            if self.right.as_ref().unwrap().right.as_ref().unwrap().left == None {
                                self.right.as_mut().unwrap().value =
                                    self.right.to_owned().unwrap().right.unwrap().value;
                                self.right.as_mut().unwrap().right =
                                    self.right.to_owned().unwrap().right.unwrap().right;
                            } else {
                                let min = Node::get_min(
                                    self.right.as_mut().unwrap().right.as_mut().unwrap(),
                                );
                                self.right.as_mut().unwrap().remove(min.value);
                                self.right.as_mut().unwrap().value = min.value;
                            }
                        }
                    }
                    false
                }
            }
        } else if value < self.value {
            match self.left.take() {
                None => return false,
                Some(lnode) => {
                    self.left = Some(lnode);
                    let result = self.left.as_mut().unwrap().remove(value);
                    if result {
                        if self.left.as_ref().unwrap().left == None
                            && self.left.as_ref().unwrap().right == None
                        {
                            self.left = None;
                        } else if self.left.as_ref().unwrap().left == None {
                            self.left = self.left.to_owned().unwrap().right;
                        } else if self.left.as_ref().unwrap().right == None {
                            self.left = self.left.to_owned().unwrap().left;
                        } else {
                            if self.left.as_ref().unwrap().right.as_ref().unwrap().left == None {
                                self.left.as_mut().unwrap().value =
                                    self.left.to_owned().unwrap().right.unwrap().value;
                                self.left.as_mut().unwrap().right =
                                    self.left.to_owned().unwrap().right.unwrap().right;
                            } else {
                                let min = Node::get_min(
                                    self.left.as_mut().unwrap().right.as_mut().unwrap(),
                                );
                                self.left.as_mut().unwrap().remove(min.value);
                                self.left.as_mut().unwrap().value = min.value;
                            }
                        }
                    }
                    false
                }
            }
        } else {
            true
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
        } else if value > self.value {
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
