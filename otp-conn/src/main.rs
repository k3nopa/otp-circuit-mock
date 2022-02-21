#![allow(unused_assignments)]

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use otp::tree::DecisionTree;
use pbr::ProgressBar;
use serialport;
use std::io;
use std::time::Duration;

static LAYER: usize = 128;
static HEIGHT: usize = 7;

fn main() {
    let port_name = "/dev/ttyS0";
    let baud_rate = 115200;

    let builder = serialport::new(port_name, baud_rate)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .timeout(Duration::from_millis(10));

    let port = builder.open();
    let (_otp_tree, memory_nodes) = DecisionTree::new(LAYER, HEIGHT);

    match port {
        Ok(mut port) => {
            println!("UART on {} at {} baud:", &port_name, &baud_rate);
            send_decision_tree(&mut port, &memory_nodes);
            println!("Decision Tree: {:?}", memory_nodes);
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
}

#[derive(PartialEq)]
enum Status {
    SEND,
    RECV,
}

fn send_decision_tree(port: &mut Box<dyn serialport::SerialPort>, memory_nodes: &Vec<Vec<u32>>) {
    let total = memory_nodes.len() * memory_nodes[0].len();
    let mut pb = ProgressBar::new(total as u64);
    let mut status = Status::SEND;

    for node in memory_nodes {
        for val in node {
            let mut data = vec![];
            data.write_u32::<BigEndian>(*val).unwrap();

            if Status::SEND == status {
                port.write(&data).unwrap();
                status = Status::RECV;
            }
            if Status::RECV == status {
                let mut buf = [0; 1];
                let mut resp = 0;
                while resp == 0 {
                    match port.read(&mut buf) {
                        Ok(_) => {
                            let mut buffer = io::Cursor::new(buf);
                            resp = buffer.read_u8().unwrap();
                            status = Status::SEND;
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {}
                        Err(e) => {
                            eprintln!("{:?}", e);
                            resp = 1;
                        }
                    };
                }
            }
            pb.inc();
        }
    }
}
