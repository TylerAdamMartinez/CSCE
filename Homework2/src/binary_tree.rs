use std::mem;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
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

    /// Returns the number of the nodes in binary search tree as a u64
    ///
    /// # Example
    /// ```
    /// use binary_tree::BinaryTree;
    /// let my_bst = BinaryTree::new();
    /// let number_of_nodes = my_bst.size();
    /// println!("Number of nodes in tree: {}", number_of_nodes);
    /// //TERMINAL OUTPUT: Number of nodes in tree: 0
    /// ```
    pub fn size(&self) -> u64 {
        match self {
            BinaryTree::Leaf {
                ref left,
                ref right,
                ..
            } => {
                return 1 + self.size_of_subtree(left) + self.size_of_subtree(right);
            }
            BinaryTree::Null => {
                return 0;
            }
        }
    }

    fn size_of_subtree(&self, tree: &BinaryTree) -> u64 {
        match tree {
            BinaryTree::Leaf {
                ref left,
                ref right,
                ..
            } => {
                return 1 + self.size_of_subtree(left) + self.size_of_subtree(right);
            }
            BinaryTree::Null => {
                return 0;
            }
        }
    }

    /// Returns the height of binary search tree as a u64
    ///
    /// # Example
    ///
    /// ```
    /// use binary_tree::BinaryTree;
    /// let my_bst = BinaryTree::new();
    /// let depth_of_bst = my_bst.depth();
    /// println!("Height of binary search tree: {}", depth_of_bst);
    /// //TERMINAL OUTPUT: Height of binary search tree: 0
    /// ```
    pub fn depth(&self) -> u64 {
        let left_depth: u64;
        let right_depth: u64;

        match self {
            BinaryTree::Leaf {
                ref left,
                ref right,
                ..
            } => {
                left_depth = self.depth_of_subtree(left);
                right_depth = self.depth_of_subtree(right);

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

    fn depth_of_subtree(&self, tree: &BinaryTree) -> u64 {
        let left_depth: u64;
        let right_depth: u64;

        match tree {
            BinaryTree::Leaf {
                ref left,
                ref right,
                ..
            } => {
                left_depth = self.depth_of_subtree(left);
                right_depth = self.depth_of_subtree(right);

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
    /// # Argument
    ///
    /// * `new_data` - A f64 value to be inserted into the BinaryTree
    ///
    /// # Example
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
    #[allow(dead_code)]
    pub fn print(&self) {
        println!("{:#?}", self);
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
    /// # Argument
    ///
    /// * `key` - The searched for value
    ///
    /// # Example
    ///
    /// ```
    /// use binary_tree::BinaryTree;
    /// let my_bst = binary_tree::BinaryTree::new();
    /// my_bst.insert(1.234);
    /// let searched_for_node = my_bst.search(1.234);
    /// match *searched_for_node {
    /// binary_tree::BinaryTree::Leaf {
    ///     ref data,
    ///     .. } => println!("The value {}", data),
    ///     binary_tree::BinaryTree::Null => println!("The value is Null")
    /// }
    /// //TERMINAL OUTPUT: The value is 1.234
    /// ```
    pub fn search(&self, key: f64) -> &BinaryTree {
        match self {
            BinaryTree::Leaf {
                ref data,
                ref left,
                ref right,
                ..
            } => {
                if data == &key {
                    return self;
                } else if data > &key {
                    return self.search_subtree(left, key);
                } else {
                    return self.search_subtree(right, key);
                }
            }
            BinaryTree::Null => {
                return &BinaryTree::Null;
            }
        }
    }

    fn search_subtree(&self, tree: &'a BinaryTree, key: f64) -> &'a BinaryTree {
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
                    return self.search_subtree(left, key);
                } else {
                    return self.search_subtree(right, key);
                }
            }
            BinaryTree::Null => {
                return &BinaryTree::Null;
            }
        }
    }
}

impl BinaryTree {
    /// Returns Null
    ///
    /// # Argument
    ///
    /// * `key` - The value to be deleted
    ///
    /// # Example
    ///
    /// ```
    /// use binary_tree::BinaryTree;
    /// let my_bst = binary_tree::BinaryTree::new();
    /// my_bst.insert(1.234);
    /// my_bst.remove(1.234);
    /// ```
    pub fn remove(&mut self, key: f64) {
        loop {
            match *self {
                BinaryTree::Leaf {
                    ref data,
                    ref mut left,
                    ref mut right
                } => {
                    if data == &key {
                        mem::drop(self);
                        break;
                    } else if data > &key {
                        match **left {
                            BinaryTree::Leaf {
                                ref left,
                                .. 
                            } => *self = *left.clone(),
                            BinaryTree::Null => break,
                        }
                    } else if data < &key {
                        match **right {
                            BinaryTree::Leaf {
                                ref right,
                                .. 
                            } => *self  = *right.clone(),
                            BinaryTree::Null => break,
                        }
                    } else {
                        break;
                    }
                },
                BinaryTree::Null => break,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let mut binary_search_tree = BinaryTree::new();
        binary_search_tree.insert(5.32);
        binary_search_tree.insert(77.32);
        binary_search_tree.insert(1.23);
        binary_search_tree.insert(455.23);
        println!("{:?}", binary_search_tree)
    }

    #[test]
    fn print() {
        let mut binary_search_tree = BinaryTree::new();
        binary_search_tree.insert(3.32);
        binary_search_tree.insert(6.43);
        binary_search_tree.insert(5.32);
        binary_search_tree.insert(77.32);
        binary_search_tree.print()
    }

    #[test]
    fn size() {
        let mut binary_search_tree = BinaryTree::new();
        binary_search_tree.insert(3.32);
        binary_search_tree.insert(6.43);
        binary_search_tree.insert(5.32);
        binary_search_tree.insert(77.32);
        assert_eq!(binary_search_tree.size(), 4);
    }

    #[test]
    fn found_search() {
        let mut binary_search_tree = BinaryTree::new();
        binary_search_tree.insert(3.32);
        binary_search_tree.insert(6.43);
        binary_search_tree.insert(5.32);
        binary_search_tree.insert(77.32);
        binary_search_tree.insert(1.23);
        assert_eq!(
            *binary_search_tree.search(1.23), 
            BinaryTree::Leaf {
                data: 1.23,
                left: Box::new(BinaryTree::Null),
                right: Box::new(BinaryTree::Null),
            });
    }

    #[test]
    fn unfound_search() {
        let mut binary_search_tree = BinaryTree::new();
        binary_search_tree.insert(3.32);
        binary_search_tree.insert(6.43);
        binary_search_tree.insert(5.32);
        binary_search_tree.insert(77.32);
        binary_search_tree.insert(1.23);
        assert_eq!(
            *binary_search_tree.search(9.99), 
            BinaryTree::Null);
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
        assert_eq!(binary_search_tree.depth(), 5);
    }

    #[test]
    fn found_remove_first() {
        let mut binary_search_tree = BinaryTree::new();
        binary_search_tree.insert(508.058);
        binary_search_tree.insert(742.160);
        binary_search_tree.insert(722.376);
        binary_search_tree.insert(749.221);
        binary_search_tree.remove(508.058)
    }

    #[test]
    fn found_remove_last() {
        let mut binary_search_tree = BinaryTree::new();
        binary_search_tree.insert(508.058);
        binary_search_tree.insert(742.160);
        binary_search_tree.insert(722.376);
        binary_search_tree.insert(749.221);
        binary_search_tree.remove(749.221)
    }

    #[test]
    fn unfound_remove() {
        let mut binary_search_tree = BinaryTree::new();
        binary_search_tree.insert(508.058);
        binary_search_tree.insert(742.160);
        binary_search_tree.insert(722.376);
        binary_search_tree.insert(749.221);
        binary_search_tree.remove(999.999)
    }
}
