use rand::Rng;

#[derive(Debug)]
#[derive(PartialEq)]
enum BinaryTree {
    Leaf {
        data: f64,
        left: Box<BinaryTree>,
        right: Box<BinaryTree>,
    },
    Null
}

impl BinaryTree {
    fn new() -> Self {
        BinaryTree::Null
    }

    fn create(data: f64) -> Self {
        BinaryTree::Leaf {
            data,
            left: Box::new(BinaryTree::Null),
            right: Box::new(BinaryTree::Null),
        }
    }

    fn size(&self, tree: &BinaryTree) -> u64 {
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

    fn depth(&self, tree: &BinaryTree) -> u64 {
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

    fn insert(&mut self, new_data: f64) -> bool {
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
                *self = BinaryTree::create(new_data);
                return true;
            }
        }
    }
}

impl <'a> BinaryTree {
    fn search(&self, tree: &'a BinaryTree, key: f64) -> &'a BinaryTree {
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

fn main() {
    let mut binary_search_tree = BinaryTree::new();
    let mut rng = rand::thread_rng();

    for _n in 0..10 {
        binary_search_tree.insert(rng.gen_range(0.0..1000.0));
    }

    binary_search_tree.insert(1001.0);

    println!("binary_search_tree is {:#?}", binary_search_tree);
    println!("binary_search_tree has {} elements", binary_search_tree.size(&binary_search_tree));
    println!("binary_search_tree has a depth of {}", binary_search_tree.depth(&binary_search_tree));
    println!("Find 1001.0 in the tree\nStatus: {:?}", binary_search_tree.search(&binary_search_tree, 1001.0));
    println!("Find 1055.0 in the tree\nStatus: {:?}", binary_search_tree.search(&binary_search_tree, 1055.0));

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create() {
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
