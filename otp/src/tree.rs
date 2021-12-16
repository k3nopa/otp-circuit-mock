#[warn(unused_assignments)]
#[warn(dead_code)]
use crate::node::Node;

/// Holds multiple complete structure of OTP decision trees.
pub struct DecisionTree {
    pub trees: Vec<Tree>,
}
/// Holds a complete structure of one OTP decision tree (all nodes are contain).
/// Generate tree based on amount height.
#[derive(Debug)]
pub struct Tree {
    /// Vector is used here as an arena to index or search Node from its id.
    /// Because Node tracks its parent and childrens with ids.
    pub tree: Vec<Node>,
    height: usize,
}

impl DecisionTree {
    pub fn new(layers: usize, height: usize) -> Self {
        let mut trees = vec![];

        for _ in 0..layers {
            let tree = Tree::new(height);
            trees.push(tree);
        }

        for i in 0..layers {
            let tree = &mut trees[i];
            tree.generate();
        }

        Self { trees }
    }
    pub fn key(&mut self, path: u8) -> Option<u32> {
        let mut ret = None;
        let mut status = false;
        let mut curr = 0;

        while !status {
            let res = self.trees[curr].key(path);
            ret = match res {
                Some(k) => {
                    status = true;
                    Some(k)
                }
                None => {
                    let bin_repr = format!("{:08b}", path);
                    println!("Path: {}, Degraded on Layer: {}", bin_repr, curr);
                    curr += 1;
                    None
                }
            };
        }
        ret
    }
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
        let mut counter = 2usize.pow(self.height as u32);
        let mut parent_node = 2usize.pow((self.height - 1) as u32) - 1;
        while counter > 0 {
            let new_node = Node::memory(self.tree.len());
            self.tree.push(new_node);

            if self.tree[parent_node].children[0] == -1 {
                self.tree[parent_node].children[0] = new_node.node_id as isize;
            } else {
                self.tree[parent_node].children[1] = new_node.node_id as isize;
                parent_node += 1;
            }
            counter -= 1;
        }
    }
    /// Traverse the decision tree using the path given.
    pub fn key(&mut self, path: u8) -> Option<u32> {
        let bin_repr = format!("{:08b}", path);

        // Make sure path's bit length is approriate.
        let path = bin_repr[(bin_repr.len() - self.height)..].to_string();
        let key = self.traverse_tree(0, &path);
        key
    }

    fn traverse_tree(&mut self, node_id: usize, path: &str) -> Option<u32> {
        // Current Switch Node has degraded, value == 0.
        if self.tree[node_id].value == 0 {
            return None;
        }
        // Manage to traverse to memory Node.
        if self.tree[node_id].children[0] == -1 {
            return Some(self.tree[node_id].value);
        }

        for (i, p) in path.chars().enumerate() {
            match p {
                '0' => {
                    // As root node are not switch Node.
                    if node_id != 0 && self.tree[node_id].value > 0 {
                        self.tree[node_id].value -= 1;
                    }
                    return self
                        .traverse_tree(self.tree[node_id].children[0] as usize, &path[i + 1..]);
                }
                '1' => {
                    // As root node are not switch Node.
                    if node_id != 0 && self.tree[node_id].value > 0 {
                        self.tree[node_id].value -= 1;
                    }
                    return self
                        .traverse_tree(self.tree[node_id].children[1] as usize, &path[i + 1..]);
                }
                _ => {}
            }
        }
        return None;
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
            // If Node is memory Node.
            if node.children[0] == -1 {
                continue;
            } else if self.tree[node.children[0] as usize].children[0] == -1 {
                // When Node is last layer of switch.
                println!(
                    "  SW_{} -> {{M_{}, M_{}}}",
                    &node.node_id, &node.children[0], &node.children[1]
                );
            } else {
                println!(
                    "  SW_{} -> {{SW_{}, SW_{}}}",
                    &node.node_id, &node.children[0], &node.children[1]
                );
            }
        }

        println!("}}");
    }
}
