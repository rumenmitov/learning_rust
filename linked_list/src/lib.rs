pub mod linked_list {
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

#[derive(PartialEq, Debug)] 
pub struct Node {
    pub val :i32,
    pub next :Option<Rc<RefCell<Node>>>,
}

#[derive(PartialEq, Debug)] 
pub struct NodeIter {
    pub node :Option<Rc<RefCell<Node>>>
}

impl Node {
    pub fn new() -> Self {
        Node {
            val:   0,
            next: None,
        }
    }

    pub fn append(&mut self, nval :i32) {
        let cursor = self;

        if let Some(ncurs) = &cursor.next {
            ncurs.as_ref().borrow_mut().append(nval);
        } else {
            cursor.next = Some(Rc::new(RefCell::new(Node::from(nval))));
        }
    }

    pub fn insert(&mut self, nval :i32, index :u32) {
        if index == 0 {
            let mut node = Node::from(self.val);
            self.val = nval;

            node.next = self.next.take();
            self.next = Some(Rc::new(RefCell::new(node)));
        } else {
            if let Some(next) = &self.next {
                next.as_ref().borrow_mut().insert(nval, index - 1);
            } else {
                let node = Node::from(nval);
                self.next = Some(Rc::new(RefCell::new(node)));
            }
        }
    }
}


impl Iterator for NodeIter {
    type Item = Rc<RefCell<Node>>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if let Some(node) = self.node.take() {
            if let Some(next) = &node.borrow().next {
                self.node = Some(next.clone());
            } else {
                self.node = None;
            }

            return Some(node);
        }

        None
    }
}

impl IntoIterator for Node {
    type Item = Rc<RefCell<Node>>;
    type IntoIter = NodeIter;

    fn into_iter(self) -> Self::IntoIter {
        NodeIter {
            node: Some(Rc::new(RefCell::from(self)))
        }
    }
}

impl From<i32> for Node {
    fn from(value: i32) -> Self {
        Node {
            val:  value,
            next: None,
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

}
