#[derive(Debug)]
#[derive(PartialEq)]
pub enum BinaryTree {
    Leaf {
        data: f64,
        left: Box<BinaryTree>,
        right: Box<BinaryTree>,
    },
    Null
}

impl BinaryTree {
    pub fn new() -> Self {
        BinaryTree::Null
    }

    fn plant(data: f64) -> Self {
        BinaryTree::Leaf {
            data,
            left: Box::new(BinaryTree::Null),
            right: Box::new(BinaryTree::Null),
        }
    }

    pub fn size(&self, tree: &BinaryTree) -> u64 {
        match tree {
            BinaryTree::Leaf {
                ref left,
                ref right,
                ..
            } => {
                return 1 + self.size(left) + self.size(right);
            }
            BinaryTree::Null => {
                return 0;
            }
        }
    }

    pub fn depth(&self, tree: &BinaryTree) -> u64 {
        let left_depth: u64;
        let right_depth: u64;

        match tree {
            BinaryTree::Leaf {
                ref left,
                ref right,
                ..
            } => {
                left_depth = self.depth(left);
                right_depth = self.depth(right);

                if left_depth > right_depth {
                    return left_depth + 1;
                } else {
                    return right_depth + 1;
                }
            }
            BinaryTree::Null => {
                return 0;
            }
        }
    }

    pub fn insert(&mut self, new_data: f64) -> bool {
        match self {
            BinaryTree::Leaf {
                ref data,
                ref mut left,
                ref mut right,
            } => match data {
                i if i > &new_data => {
                    left.insert(new_data);
                    return true;
                }
                i if i < &new_data => {
                    right.insert(new_data);
                    return true;
                }
                _ => { 
                    return false;
                }
            },
            BinaryTree::Null => {
                *self = BinaryTree::plant(new_data);
                return true;
            }
        }
    }
}

impl <'a> BinaryTree {
    pub fn search(&self, tree: &'a BinaryTree, key: f64) -> &'a BinaryTree {
        match tree {
            BinaryTree::Leaf {
                ref data,
                ref left,
                ref right,
                ..
            } => {
                if data == &key {
                    return tree;
                } else if data > &key {
                    return self.search(left, key);
                } else {
                    return self.search(right, key);
                }
            }
            BinaryTree::Null => {
                return &BinaryTree::Null;
            }
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn plant() {
        let mut binary_search_tree = BinaryTree::new();
        binary_search_tree.insert(5.32);
        binary_search_tree.insert(77.32);
        binary_search_tree.insert(1.23);
        binary_search_tree.insert(455.23);
        println!("{:?}", binary_search_tree)
    }

    #[test]
    fn size() {
        let mut binary_search_tree = BinaryTree::new();
        binary_search_tree.insert(3.32);
        binary_search_tree.insert(6.43);
        binary_search_tree.insert(5.32);
        binary_search_tree.insert(77.32);
        assert_eq!(binary_search_tree.size(&binary_search_tree), 4);
    }

    #[test]
    fn search() {
        let mut binary_search_tree = BinaryTree::new();
        binary_search_tree.insert(3.32);
        binary_search_tree.insert(6.43);
        binary_search_tree.insert(5.32);
        binary_search_tree.insert(77.32);
        binary_search_tree.insert(1.23);
        assert_eq!(
            *binary_search_tree.search(&binary_search_tree, 1.23), 
            BinaryTree::Leaf {
                data: 1.23,
                left: Box::new(BinaryTree::Null),
                right: Box::new(BinaryTree::Null),
            });
    }

    #[test]
    fn depth() {
        let mut binary_search_tree = BinaryTree::new();
        binary_search_tree.insert(409.571);
        binary_search_tree.insert(290.002);
        binary_search_tree.insert(3.80219);
        binary_search_tree.insert(772.576);
        binary_search_tree.insert(526.063);
        binary_search_tree.insert(508.058);
        binary_search_tree.insert(742.160);
        binary_search_tree.insert(722.376);
        binary_search_tree.insert(749.221);
        assert_eq!(binary_search_tree.depth(&binary_search_tree), 5);
    }
}
