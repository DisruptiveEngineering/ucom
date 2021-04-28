//!
//!
use std::io::{BufReader, BufRead, ErrorKind};

fn main() {
    eprintln!("ucom");

    let ports = match serialport::available_ports() {
        Ok(ports) => ports,
        Err(_e) => {
            eprintln!("No ports found.");
            return;
        }
    };

    for (i, p) in ports.iter().enumerate() {
        // Extract port info if usb
        let port_info = match &p.port_type {
            serialport::SerialPortType::UsbPort(port_info) => port_info,
            _ => continue
        };

        let manufacturer = port_info.manufacturer.as_deref().unwrap_or("<>");
        let product = port_info.product.as_deref().unwrap_or("<>");

        let port = match serialport::new(&p.port_name, 3_000_000)
            .timeout(std::time::Duration::from_secs_f32(2.0))
            .open() {
            Ok(p) => p,
            Err(_e) => return
        };

        eprintln!("{} {} ({} - {})", i, p.port_name, manufacturer, product);

        let mut reader = BufReader::new(port);
        let mut buf = String::with_capacity(128);

        loop {
            buf.clear();

            // Check for errors
            if let Err(e) = reader.read_line(&mut buf) {
                match e.kind() {
                    ErrorKind::InvalidData => continue,
                    kind => {
                        eprintln!("can not read ({:?} - {})", kind, e);
                        return;
                    }
                }
            }
            print!("{}", &buf);
        }
    }
}
