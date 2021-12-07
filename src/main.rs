#[warn(dead_code)]

/// Holds multiple layers of decision trees.
struct DecisionTree {
    layers: Vec<Tree>,
}

/// Holds a complete structure of one OTP decision tree (all nodes are contain).
/// Generate tree based on amount height.
#[derive(Debug)]
struct Tree {
    /// Vector is used here as an arena to index or search Node from its id.
    /// Because Node tracks its parent and childrens with ids.
    tree: Vec<Node>,
    height: usize,
}

#[derive(Debug, Clone, Copy)]
struct Node {
    node_id: usize,
    decision_input: Option<u8>,
    decision_output: Option<u8>,
    children: [isize; 2],
    value: u32,
}

impl Tree {
    fn new(height: usize) -> Self {
        let node = Node::switch(0usize);
        let tree = vec![node];

        Self { tree, height }
    }
    /// Generate all the switches & memory nodes needed from values in Tree struct.
    fn generate(&mut self) {
        // Need to make sure that root node was initialized.
        assert!(self.tree.len() == 1usize, "Root Node don't exist");

        // Create all switch Nodes.
        let mut parent_node = 0;
        for layer in 1..self.height {
            let mut counter = 2isize.pow(layer as u32);
            while counter > 0 {
                let new_node = Node::switch(self.tree.len());
                self.tree.push(new_node);

                if self.tree[parent_node].children[0] == -1 {
                    self.tree[parent_node].children[0] =
                        self.tree[self.tree.len() - 1].node_id as isize;
                } else {
                    self.tree[parent_node].children[1] =
                        self.tree[self.tree.len() - 1].node_id as isize;
                    parent_node += 1;
                }

                counter -= 1;
            }
        }

        // Create all memory Nodes.
        let mut parent_node = 2usize.pow((self.height - 1) as u32) - 1;
        println!("{}", parent_node);
        let n = self.tree.len();
        while parent_node < n {
            let new_node = Node::memory(self.tree.len());
            self.tree.push(new_node);

            self.tree[parent_node].children[0] = new_node.node_id as isize;
            parent_node += 1;
        }
    }
}

impl Node {
    fn switch(node_id: usize) -> Self {
        Self {
            node_id,
            decision_input: None,
            decision_output: None,
            children: [-1, -1],
            value: 0,
        }
    }
    fn memory(node_id: usize) -> Self {
        Self {
            node_id,
            decision_input: None,
            decision_output: None,
            children: [-1, -1],
            value: 1,
        }
    }
}

fn main() {
    let mut tree = Tree::new(3);

    tree.generate();
    println!("{:#?}", tree.tree.len());

    // TODO: Retrieve key signal.
    // TODO: Generate nodes using weibull distribution.

    // TODO: Need to implement global allocator in order to use Vec type for STM32.
    // REF : https://github.com/rust-embedded/alloc-cortex-m
    // REF : https://github.com/yvt/rlsf
    // TODO: Port to STM32 or Raspberry Pi.
    // REF : https://github.com/golemparts/rppal
}
