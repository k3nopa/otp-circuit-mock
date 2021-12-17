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
        let input = serial_recv(&mut port).unwrap();
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

fn serial_recv<T: SerialPort>(port: &mut T) -> io::Result<String> {
    port.set_timeout(Duration::from_millis(1000)).unwrap();
    let mut key = [0u8; 5];
    loop {
        match port.read(&mut key[..]) {
            Ok(_) => break,
            Err(_) => port.set_timeout(Duration::from_millis(1000)).unwrap(),
        };
    }
    println!("Byte Recv: {:?}", key);
    let mut zero_index = 0;
    for i in 0..key.len() {
        if key[i] == 0u8 {
            zero_index = i + 1;
            break;
        }
    }
    let key = String::from_utf8(key[0..zero_index].to_vec()).unwrap();
    Ok(key)
}
