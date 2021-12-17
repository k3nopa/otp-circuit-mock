extern crate serial;

use otp::tree::DecisionTree;
use rand::Rng;
use serial::prelude::*;
use std::io;
use std::time::Duration;

fn main() {
    let mut otp_tree = DecisionTree::new(128, 7);

    for i in 0..=10 {
        let path = rand::thread_rng().gen_range(0..=255);
        let bin_repr = format!("{:08b}", path);

        let otp_key = otp_tree.key(path);
        println!("Iteration: {} Path: {} -> {:?}", i, bin_repr, otp_key);
    }

    let mut port = serial::open("/dev/ttyS0").unwrap();
    interact(&mut port).unwrap();
}

fn interact<T: SerialPort>(port: &mut T) -> io::Result<()> {
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud115200).unwrap();
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })
    .unwrap();

    port.set_timeout(Duration::from_millis(1000)).unwrap();

    let mut buf: Vec<u8> = (0..255).collect();

    port.write(&buf[..]).unwrap();
    //port.read(&mut buf[..]).unwrap();

    Ok(())
}
