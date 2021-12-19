use byteorder::{BigEndian, WriteBytesExt};
use otp::tree::DecisionTree;
use serialport;
use std::io;
use std::time::Duration;

enum Status {
    ReadSize,
    ReadData,
}

static LAYER: usize = 128;
static HEIGHT: usize = 7;

fn main() {
    let port_name = "/dev/ttyS0";
    let baud_rate = 115200;

    // let builder = serialport::new(port_name, baud_rate)
    //     .stop_bits(serialport::StopBits::One)
    //     .data_bits(serialport::DataBits::Eight)
    //     .timeout(Duration::from_millis(10));

    // let port = builder.open();
    let (mut otp_tree, mut memory_nodes) = DecisionTree::new(LAYER, HEIGHT);

    let slice = &memory_nodes[0];
    for val in slice {
        let mut data = vec![];
        data.write_u32::<BigEndian>(*val).unwrap();
        println!("{}: {:?}", val, data);
        //serial_write(&mut port, &data).unwrap();
    }
    // match port {
    //     Ok(mut port) => {
    //         println!("UART on {} at {} baud:", &port_name, &baud_rate);
    //         // 1. Send OTP info.
    //         // serial_write(&mut port, memory_nodes).unwrap();
    //         // let path = serial_read(&mut port).unwrap();
    //         // println!("Path: {:?}", path);
    //         // let path = path.parse::<u8>().unwrap();
    //         // let res = otp_tree.key(path).unwrap();
    //         // println!("Key: {:?}", res);
    //     }
    //     Err(e) => {
    //         eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
    //         ::std::process::exit(1);
    //     }
    // }
}

fn serial_read(port: &mut Box<dyn serialport::SerialPort>) -> io::Result<String> {
    let mut status = Status::ReadSize;
    let mut tmp_path = String::new();
    let mut size = 0;
    let mut buffer = [0; 10];

    let path = loop {
        match port.read(&mut buffer) {
            Ok(t) => match status {
                Status::ReadSize => {
                    // TODO: if the size == 0, do initialization.
                    // Sending OTP tree.
                    size = String::from_utf8_lossy(&buffer[..t])
                        .parse::<usize>()
                        .unwrap();
                    status = Status::ReadData;
                    tmp_path.reserve(size);
                }
                Status::ReadData => {
                    size -= t;
                    let input = String::from_utf8_lossy(&buffer[..t]);
                    tmp_path += &input;
                    if size == 0 {
                        break tmp_path;
                    }
                }
            },
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
    };
    Ok(path)
}

fn serial_write(port: &mut Box<dyn serialport::SerialPort>, data: &[u8]) -> io::Result<()> {
    port.write(data).unwrap();
    Ok(())
}
