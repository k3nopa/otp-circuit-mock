extern crate serial;

use otp::tree::DecisionTree;
use rand::Rng;
use serial::prelude::*;
use std::io;
use std::time::Duration;

fn main() {
    let mut port = serial::open("/dev/ttyS0").unwrap();
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud115200).unwrap();
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })
    .unwrap();
    let mut otp_tree = DecisionTree::new(128, 7);

    loop {
        let input = loop {
            let input = match serial_recv(&mut port) {
                Ok(x) => break x,
                Err(_) => {}
            };
            input
        };
        let input = String::from_utf8_lossy(&input).to_string();
        let path = input.parse::<u8>().unwrap();
        println!("Path Recv: {}", path);

        let otp_key = otp_tree.key(path).unwrap();
        let key = otp_key.to_string();
        serial_send(&mut port, key.as_bytes()).unwrap();
    }
}

fn serial_send<T: SerialPort>(port: &mut T, key: &[u8]) -> io::Result<()> {
    port.set_timeout(Duration::from_millis(1000)).unwrap();
    port.write(key).unwrap();

    Ok(())
}

fn serial_recv<T: SerialPort>(port: &mut T) -> io::Result<[u8; 50]> {
    port.set_timeout(Duration::from_millis(1000)).unwrap();
    let mut key = [0u8; 50];
    port.read(&mut key[..]).unwrap();

    Ok(key)
}
