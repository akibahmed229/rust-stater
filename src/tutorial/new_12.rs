// A smart pointer in Rust is a data structure that acts like a pointer but provides additional functionality 
// such as memory management and ownership tracking. It allows for more controlled and efficient handling of memory resources. 
// Rust's standard library provides smart pointer types like Box, Rc, and Arc, 

#[derive(Debug)] // Make the TreeNode struct printable for debugging
struct TreeNode<T, U> {
    // Fields for representing a binary tree node:
    pub left: Option<Box<TreeNode<T, U>>>, // Option type for left child, allowing for absence of a child
    pub right: Option<Box<TreeNode<T, U>>>, // Option type for right child, similarly
    pub key: T, // Key value of the node
    pub value: U, // Associated value of the node
}

impl<T: std::fmt::Debug, U: std::fmt::Debug> TreeNode<T, U> {
    // Constructor for creating a new TreeNode instance:
    pub fn new(key: T, value: U) -> Self {
        TreeNode {
            left: None,   // Initially, children are absent
            right: None,  //
            key,      
            value,
        }
    }

    // Method to set the left child of a node:
    pub fn left(&mut self, node: TreeNode<T, U>) -> &mut Self {
        self.left = Some(Box::new(node)); // Box the child node for heap allocation
        self   // Return a mutable reference to the current node for method chaining
    }

    // Method to set the right child of a node:
    pub fn right(&mut self, node: TreeNode<T, U>) -> &mut Self {
        self.right = Some(Box::new(node)); // Similarly box the right child
        self   // Return a mutable reference for method chaining
    }

    // Method to traverse the tree in-order (left subtree, root, right subtree):
    pub fn traverse(&self) {
        if let Some(left) = &self.left { // If there's a left child, traverse it recursively
            left.traverse();
        }
        println!("key: {:#?}, value: {:#?}", self.key, self.value); // Print the current node's key and value
        if let Some(right) = &self.right { // If there's a right child, traverse it recursively
            right.traverse();
        }
    }
}

// Example usage of TreeNode:
pub fn my_box() {
    let mut node1 = TreeNode::new(1, 50);
    let mut node2 = TreeNode::new(2, 30);
    let mut node3 = TreeNode::new(3, 70);

    // Build the tree structure using left and right methods:
    node2.left(TreeNode::new(4, 20)).right(TreeNode::new(5, 40));
    node3.left(TreeNode::new(6, 60)).right(TreeNode::new(7, 80));
    node1.left(node2).right(node3);

    // Traverse the tree to print its key-value pairs in-order:
    node1.traverse();
}
