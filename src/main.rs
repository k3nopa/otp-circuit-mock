use rand::Rng;
use rs_debugging::tree::Tree;

fn main() {
    let mut tree = Tree::new(3);

    tree.generate();
    tree.draw();

    let path = rand::thread_rng().gen_range(0..=255);
    let _otp_key = tree.key(path);

    // TODO: Need to implement global allocator in order to use Vec type for STM32.
    // REF : https://github.com/rust-embedded/alloc-cortex-m
    // REF : https://github.com/yvt/rlsf
    // TODO: Port to STM32 or Raspberry Pi.
    // REF : https://github.com/golemparts/rppal
}
