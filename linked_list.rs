// http://www.reddit.com/r/rust/comments/2jec05/problem_with_implementation_of_linked_list/
// Tested with rust-1.5.0

use std::fmt;

struct Node {
    value: u32,
    link: Option<Box<Node>>,
}

impl Node {
    fn new(value: u32) -> Node {
        Node { value: value, link: None, }
    }

    fn append(&mut self, value: u32) {
        match self.link {
            Some(ref mut node) => node.append(value),
            None => self.link = Some(Box::new(Node::new(value))),
        }
    }

    fn length(&self) -> u32 {
        match self.link {
            Some(ref node) => node.length() + 1,
            None => 1,
        }
    }

    fn insert_after(&mut self, value: u32, after: u32) -> bool {
        if self.value == after {
            self.link = Some(Box::new(Node { value: value, link: self.link.take() }));
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

impl fmt::Display for Node {
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
