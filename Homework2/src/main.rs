use rand::Rng;

#[derive(Debug)]
enum BinaryTree {
    Leaf {
        data: f64,
        left: Box<BinaryTree>,
        right: Box<BinaryTree>,
    },
    Null
}

impl BinaryTree {
    fn new_tree() -> Self {
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

fn main() {
    let mut binary_search_tree = BinaryTree::new_tree();
    let mut rng = rand::thread_rng();

    for _n in 1..10 {
        binary_search_tree.insert(rng.gen_range(0.0..1000.0));
    }

    println!("binary_search_tree is {:#?}", binary_search_tree);
    println!("binary_search_tree has {} elements", binary_search_tree.size(&binary_search_tree));

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create() {
        let mut binary_search_tree = BinaryTree::new_tree();
        binary_search_tree.insert(3.32);
        binary_search_tree.insert(6.43);
        binary_search_tree.insert(5.32);
        binary_search_tree.insert(77.32);
        binary_search_tree.insert(1.23);
        binary_search_tree.insert(455.23);
        println!("{:?}", binary_search_tree)
    }

    #[test]
    fn size() {
        let mut binary_search_tree = BinaryTree::new_tree();
        binary_search_tree.insert(3.32);
        binary_search_tree.insert(6.43);
        binary_search_tree.insert(5.32);
        binary_search_tree.insert(77.32);
        assert_eq!(binary_search_tree.size(&binary_search_tree), 4);
    }
}
