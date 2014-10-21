// http://www.reddit.com/r/rust/comments/2jec05/problem_with_implementation_of_linked_list/
// Tested with rust-0.12.0

use std::fmt;

struct Node {
    value: uint,
    link: Option<Box<Node>>,
}

impl Node {
    fn new(value: uint) -> Node {
        Node { value: value, link: None, }
    }

    fn append(&mut self, value: uint) {
        match self.link {
            Some(ref mut node) => node.append(value),
            None => self.link = Some(box Node::new(value)),
        }
    }

    fn length(&self) -> uint {
        match self.link {
            Some(ref node) => node.length() + 1,
            None => 1,
        }
    }

    fn insert_after(&mut self, value: uint, after: uint) -> bool {
        if self.value == after {
            self.link = Some(box Node { value: value, link: self.link.take() });
            true
        }
        else {
            match self.link {
                Some(ref mut node) => node.insert_after(value, after),
                None => false,
            }
        }
    }
}

impl fmt::Show for Node {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.link {
			Some(ref node) => write!(f, "{} -> {}", self.value, node),
			None => write!(f, "{}.", self.value)
		}
	}
}
fn main() {
	let mut root = Node::new(0);
	root.append(1);
	root.insert_after(2, 1);
	println!("Node length: {}", root.length());
	println!("Node: {}", root);
}
