use otp::tree::DecisionTree;
use rand::Rng;

fn main() {
    let mut otp_tree = DecisionTree::new(128, 7);

    for i in 0..=10 {
        let path = rand::thread_rng().gen_range(0..=255);
        let bin_repr = format!("{:08b}", path);

        let otp_key = otp_tree.key(path);
        println!("Iteration: {} Path: {} -> {:?}", i, bin_repr, otp_key);
    }

    // TODO: Need to implement global allocator in order to use Vec type for STM32 (or generate
    // decision tree as static data(bss) in memory layout.
    // REF : https://github.com/rust-embedded/alloc-cortex-m
    // REF : https://github.com/yvt/rlsf
    //
    // TODO: Port to STM32 or Raspberry Pi.
    // REF : https://github.com/golemparts/rppal
}
