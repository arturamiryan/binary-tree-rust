use std::mem;

pub struct Tree {
    root: Link,
}

struct Node {
    value: i32,
    right: Link,
    left: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

impl Tree {
    pub fn add(&mut self, value: i32) {
        match mem::replace(&mut self.root, Link::Empty) {
            Link::Empty => {
                let root = Box::new(Node{
                    value: value,
                    right: Link::Empty,
                    left: Link::Empty,
                });
                self.root = Link::More(root);
            },
            Link::More(node) => {
                add_to(node, value);
            }
        }
    }
}

fn add_to(mut node: Box<Node>, value: i32) {
    if value < node.value {
        match mem::replace(&mut node.left, Link::Empty) {
            Link::Empty => {
                let new_node = Box::new(Node{
                    value: value,
                    right: Link::Empty,
                    left: Link::Empty,
                });
                node.left = Link::More(new_node);
            },
            Link::More(node) => add_to(node, value),
        }
    } else {
        match mem::replace(&mut node.right, Link::Empty) {
            Link::Empty => {
                let new_node = Box::new(Node{
                    value: value,
                    right: Link::Empty,
                    left: Link::Empty,
                });
                node.right = Link::More(new_node);
            },
            Link::More(node) => add_to(node, value),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
