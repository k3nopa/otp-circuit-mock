use rs_debugging::tree::Tree;

fn main() {
    let mut tree = Tree::new(4);

    tree.generate();
    tree.draw();

    // TODO: Retrieve key signal.
    // TODO: Generate nodes using weibull distribution.

    // TODO: Need to implement global allocator in order to use Vec type for STM32.
    // REF : https://github.com/rust-embedded/alloc-cortex-m
    // REF : https://github.com/yvt/rlsf
    // TODO: Port to STM32 or Raspberry Pi.
    // REF : https://github.com/golemparts/rppal
}
