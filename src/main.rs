#[warn(dead_code)]

struct Node {
    node_id: usize,
    decision_input: u8,
    decision_output: u8,
    parent: Option<usize>,
    children: Option<usize>,
    /// Switch Node's degrade counter.
    /// Key Memory's key value.
    value: u32,
}

/// Holds the connection info between nodes.
struct Conn {
    from: Node,
    to: Node,
}

/// Holds a complete structure of a OTP decision tree.
/// Generate tree based on amount height.
struct Tree {
    tree: Vec<Node>,
    height: u16,
}

/// Holds multiple layers of decision trees.
struct DecisionTree {
    layers: Vec<Tree>,
}

impl Node {
    /// Add new root switch node.
    fn new() -> Self {
        todo!()
    }
    /// Add new child switch node.
    fn switch(&self) -> Self {
        // XXX: Create a child node, conn & connect parent with new child node.
        // Root Node has id of 0
        todo!()
    }
    /// Add new child memory node.
    fn memory(&self) -> Self {
        // XXX: Create a child node, conn & connect parent with new child node.
        // Root Node has id of 0
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
    // TODO: Create Tree.
    // TODO: Create all nodes needed from given tree info.
    // TODO: Retrieve key signal.
    // TODO: Generate nodes using weibull distribution.
    // TODO: Port to STM32.
}
