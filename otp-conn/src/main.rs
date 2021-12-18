use otp::tree::DecisionTree;
use serialport;
use std::io;
use std::time::Duration;

enum Status {
    ReadSize,
    ReadData,
}

fn main() {
    let mut otp_tree = DecisionTree::new(128, 7);
    let port_name = "/dev/ttyS0";
    let baud_rate = 115200;

    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open();

    match port {
        Ok(mut port) => {
            println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
            let path = serial_read(&mut port).unwrap();
            println!("Path: {:?}", path);
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
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
