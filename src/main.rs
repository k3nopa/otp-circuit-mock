use rand::Rng;
use rs_debugging::tree::DecisionTree;

fn main() {
    let mut otp_tree = DecisionTree::new(128, 7);

    for i in 0..=10 {
        let path = rand::thread_rng().gen_range(0..=255);
        let otp_key = otp_tree.key(path);
        println!("[{}]{} -> {:?}", i, path, otp_key);
    }

    // TODO: Need to implement global allocator in order to use Vec type for STM32 (or generate
    // decision tree as static data(bss) in memory layout.
    // REF : https://github.com/rust-embedded/alloc-cortex-m
    // REF : https://github.com/yvt/rlsf
    //
    // TODO: Port to STM32 or Raspberry Pi.
    // REF : https://github.com/golemparts/rppal
}
