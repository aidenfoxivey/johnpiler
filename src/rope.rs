use std::collections::VecDeque;

const LEAF_LENGTH: usize = 64;

#[derive(Debug, Clone)]
pub struct Rope {
    root: RopeNode,
}

impl Rope {
    pub fn new(s: &str) -> Self {
        Rope {
            root: RopeNode::Leaf(s.to_string()),
        }
    }

    fn len(&self) -> usize {
        self.root.len()
    }

    fn build_balanced(s: &str) -> Self {
        if s.is_empty() {
            return Rope::new("");
        }

        if s.len() <= LEAF_LENGTH {
            return Rope::new(s);
        }

        Rope {
            root: Self::build_balanced_node(s),
        }
    }

    fn build_balanced_node(s: &str) -> RopeNode {
        if s.len() <= LEAF_LENGTH {
            return RopeNode::Leaf(s.to_string());
        }

        let char_vec = s.chars().collect::<Vec<char>>();

        let mid = char_vec.len() / 2;
        let left_s = char_vec[0..mid].iter().collect::<String>();
        let right_s = char_vec[mid + 1..char_vec.len()].iter().collect::<String>();
        todo!()
    }

    fn rebalance(&self) -> Rope {
        let s = self.collect();
        Rope::build_balanced(&s)
    }

    fn collect(&self) -> String {
        let mut str = String::with_capacity(self.root.len());
        let mut deque = VecDeque::new();
        deque.push_back(self.root.clone());

        while !deque.is_empty() {
            match deque.pop_back().unwrap() {
                RopeNode::Leaf(s) => str += &s,
                RopeNode::Internal { left, right, .. } => {
                    deque.push_back(*right);
                    deque.push_back(*left);
                }
            }
        }

        str
    }

    fn concat(&self, other: Rope) -> Rope {
        todo!();
    }
}

#[derive(Debug, Clone)]
pub enum RopeNode {
    Leaf(String),
    Internal {
        length: usize,
        // Length refers to the length of the text of the subtree rooted at the left
        // child of the current node.
        left: Box<RopeNode>,
        right: Box<RopeNode>,
        // By definition, a non-leaf RopeNode must have two children. Thus, we do not
        // need to have an Option<T> wrapper.
    },
}

impl RopeNode {
    fn len(&self) -> usize {
        match self {
            RopeNode::Leaf(s) => s.len(),
            RopeNode::Internal { length, right, .. } => *length + right.len(),
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_new_rope() {
        let rope = Rope::new("Hello");
        assert_eq!(rope.len(), 5);
    }

    #[test]
    fn test_collect_leaf() {
        let rope = Rope::new("Hello");
        assert_eq!(rope.collect(), "Hello");
    }

    #[test]
    fn test_collect_internal() {
        let rope = Rope {
            root: RopeNode::Internal {
                length: 12,
                left: Box::new(RopeNode::Leaf("Hello, ".to_string())),
                right: Box::new(RopeNode::Leaf("World".to_string())),
            },
        };
        assert_eq!(rope.collect(), "Hello, World");
    }

    #[test]
    fn test_nested_collect() {
        let rope = Rope {
            root: RopeNode::Internal {
                length: 6,
                left: Box::new(RopeNode::Internal {
                    length: 3,
                    left: Box::new(RopeNode::Leaf("Pat".to_string())),
                    right: Box::new(RopeNode::Leaf("rik".to_string())),
                }),
                right: Box::new(RopeNode::Leaf(" the Barbarian".to_string())),
            },
        };

        let result = "Patrik the Barbarian";
        assert_eq!(rope.collect(), result);
        assert_eq!(rope.len(), result.len());
    }
}
