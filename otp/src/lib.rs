pub mod node;
pub mod tree;

#[cfg(test)]
mod tests {
    use super::tree::DecisionTree;
    use rand::prelude::*;
    #[test]
    fn it_works() {
        let height = 7;
        let layer = 128;

        let (mut otp_tree, _memory_nodes) = DecisionTree::new(layer, height);

        let path = thread_rng().gen::<u8>();
        let bin_repr = format!("{:08b}", path);

        // Make sure path's bit length is approriate.
        let path_repr = bin_repr[(bin_repr.len() - height)..].to_string();

        let (res, mut layer) = otp_tree.key(path);

        println!("Path: {:?} {}", path, path_repr);
        println!("Key: {:?}, Length: {}", res, res.len());
        println!("Layer: {:?}, Length: {}", layer, layer.len());

        let mut key = String::new();
        for cand in &res {
            key.push_str(&cand.to_string());
        }
        println!("Full Key: {:?}, Length: {}", key, key.len());

        assert_eq!(res.len(), layer.len());
    }
}
