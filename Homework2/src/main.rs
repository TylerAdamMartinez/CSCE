
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

fn main() {
    let binary_search_tree = BinaryTree::new_tree();
    println!("Hello, world!");
}
