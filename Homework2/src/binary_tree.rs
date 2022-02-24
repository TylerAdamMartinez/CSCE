#[derive(Debug)]
#[derive(PartialEq)]
/// BinaryTree represented here
/// data: floating point value
/// left & right: points to the next BinaryTree Node
pub enum BinaryTree {
    Leaf {
        data: f64,
        left: Box<BinaryTree>,
        right: Box<BinaryTree>,
    },
    Null
}

impl BinaryTree {
    /// Returns a BinaryTree of Null value
    ///
    /// # Example
    ///
    /// ```
    /// use binary_tree::BinaryTree;
    /// let my_bst = BinaryTree::new();
    /// ```
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

    /// Returns the number of the nodes below the given node as a u64
    ///
    /// # Arguments
    ///
    /// * `tree` - A BinaryTree node
    ///
    /// # Examples
    ///
    /// ```
    /// // If you want to get the number of all nodes in tree 
    /// let number_of_nodes_in_bst = my_bst.size(&my_bst);
    /// // If you want to get the number of nodes in a subtree of parent tree
    /// let number_of_nodes_in_subtree = my_bst.size(&sub_tree_bst_node);
    /// ```
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

    /// Returns the height of the nodes below the given node as a u64
    ///
    /// # Arguments
    ///
    /// * `tree` - A BinaryTree node
    ///
    /// # Examples
    ///
    /// ```
    /// // If you want to get the total height of tree 
    /// let depth_of_bst = my_bst.depth(&my_bst);
    /// // If you want to get the height of a subtree of parent tree
    /// let depth_of_nodes_in_subtree = my_bst.depth(&sub_tree_bst_node);
    /// ```
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

    /// Returns a boolean
    /// If returns true then insert operation was successfull
    /// If returns false then insert operation was unsuccessful
    ///
    /// # Arguments
    ///
    /// * `new_data` - A f64 value to be inserted into the BinaryTree
    ///
    /// # Examples
    ///
    /// ```
    /// use binary_tree::BinaryTree;
    /// let my_bst = binary_tree::BinaryTree::new();
    /// // successfull insertions
    /// my_bst.insert(323.321);
    /// my_bst.insert(12.3432);
    /// // unsuccessful insertions
    /// let status = my_bst.insert(12.3432);
    ///
    /// if status == false {
    ///     println!("Can't insert the same value twice");
    /// }
    /// //TERMINAL OUTPUT: Can't insert the same value twice
    /// ```
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

impl BinaryTree {
    pub fn print(&self, tree: &BinaryTree) {
        println!("{:#?}", tree);
    }
}

impl <'a> BinaryTree {
    /// Returns a reference to BinaryTree Node with the same value as inputed,
    /// or a reference to a BinaryTree::Null if the BinaryTree contain no Nodes with the same value
    ///
    /// # Lifetime of reference
    /// * `&'a BinaryTree` - The returned reference has the same Lifetime as the `tree` BinaryTree
    /// parameter
    ///
    /// # Arguments
    ///
    /// * `tree` - A BinaryTree node
    /// * `key` - The searched for value
    ///
    /// # Examples
    ///
    /// ```
    /// my_bst.insert(1.234);
    /// let searched_for_node = my_bst.search(&my_bst, 1.234);
    /// match *searched_for_node {
    /// binary_tree::BinaryTree::Leaf {
    ///     ref data,
    ///     .. } => println!("The value {}", data),
    ///     binary_tree::BinaryTree::Null => println!("The value is Null")
    /// }
    /// //TERMINAL OUTPUT: The value is 1.234
    /// ```
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
