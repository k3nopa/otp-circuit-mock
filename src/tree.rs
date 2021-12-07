/// Holds a complete structure of one OTP decision tree (all nodes are contain).
/// Generate tree based on amount height.
#[derive(Debug)]
pub struct Tree {
    /// Vector is used here as an arena to index or search Node from its id.
    /// Because Node tracks its parent and childrens with ids.
    tree: Vec<Node>,
    height: usize,
}

impl Tree {
    pub fn new(height: usize) -> Self {
        let node = Node::switch(0usize);
        let tree = vec![node];

        Self { tree, height }
    }
    /// Generate all the switches & memory nodes needed from values in Tree struct.
    pub fn generate(&mut self) {
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
        let n = self.tree.len();
        while parent_node < n {
            let new_node = Node::memory(self.tree.len());
            self.tree.push(new_node);

            self.tree[parent_node].children[0] = new_node.node_id as isize;
            parent_node += 1;
        }
    }
    /// Draw the decision tree using graphiz.
    pub fn draw(&self) {
        println!("digraph DecisionTree {{");
        // Add Node's informations.
        for node in self.tree.iter() {
            // Don't add memory's Node information.
            if &node.children[0] == &-1 {
                continue;
            }
            println!("  SW_{}  [label=\"{}\"]", &node.node_id, &node.value);
        }

        // Add Node's connections.
        for node in self.tree.iter() {
            if &node.children[0] == &-1 {
                // Node don't have children, possible memory Node.
                continue;
            } else if &node.children[1] == &-1 {
                // Node have 1 child, possible last layer switch Node.
                println!("  SW_{} -> M_{}", &node.node_id, &node.children[0]);
            } else {
                // Node have 2 children.
                println!(
                    "  SW_{} -> {{SW_{}, SW_{}}}",
                    &node.node_id, &node.children[0], &node.children[1]
                );
            }
        }

        println!("}}");
    }
}
