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

    fn new_leaf(data: f64) -> Self {
        BinaryTree::Leaf {
            data,
            left: Box::new(BinaryTree::Null),
            right: Box::new(BinaryTree::Null),
        }
    }
}

impl BinaryTree {
    fn add_leaf(&mut self, new_data: f64) {
        match self {
            BinaryTree::Leaf {
                ref data,
                ref mut left,
                ref mut right,
            } => match data {
                i if i > &new_data => {
                    left.add_leaf(new_data);
                    println!("added {} to the left", new_data);
                }
                i if i < &new_data => {
                    right.add_leaf(new_data);
                    println!("added {} to the right", new_data);
                }
                _ => {}
            },
            BinaryTree::Null => {
                *self = BinaryTree::new_leaf(new_data);
            }
        }
    }
}

fn main() {
    let mut binary_search_tree = BinaryTree::new_tree();
    binary_search_tree.add_leaf(3.32);
    binary_search_tree.add_leaf(6.43);
    binary_search_tree.add_leaf(5.32);
    binary_search_tree.add_leaf(77.32);
    binary_search_tree.add_leaf(1.23);
    binary_search_tree.add_leaf(455.23);
    binary_search_tree.add_leaf(6.43);
    binary_search_tree.add_leaf(3213.2);
    binary_search_tree.add_leaf(0.21231);
}
