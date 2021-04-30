//!
//!

#[macro_use]
extern crate clap;

use std::io::{ErrorKind, Read};
use clap::Clap;
use serialport::SerialPortInfo;
use std::io::{stdin, stdout, Write};

#[derive(Clap)]
#[clap(version = crate_version ! (), author = crate_authors ! (), about = crate_description ! ())]
struct Opts {
    #[clap(short, long, default_value = "3000000")]
    baudrate: usize,

    #[clap(short, long)]
    device: Option<String>,
}


fn port_info(port: &SerialPortInfo) -> Option<String> {
    let port_info = match &port.port_type {
        serialport::SerialPortType::UsbPort(port_info) => port_info,
        _ => return None
    };

    let manufacturer = port_info.manufacturer.as_deref().unwrap_or("<>");
    let product = port_info.product.as_deref().unwrap_or("<>");

    Some(format!("{} ({} - {})", port.port_name, manufacturer, product))
}


fn device_prompt(ports: &Vec<SerialPortInfo>) -> String {
    let mut s = String::new();
    loop {
        eprintln!("Select device:");
        for (i, port) in ports.iter().enumerate() {
            let info = match port_info(&port) {
                Some(info) => info,
                None => continue
            };

            eprintln!("({}) {}", i, info);
        }
        s.clear();

        eprint!("Choose [0]: ");
        let _ = stdout().flush();
        match stdin().read_line(&mut s) {
            Ok(_n) => {}
            Err(e) => {
                eprintln!("Error ({})", e);
                continue;
            }
        }

        // if enter is given, the first element is chosen
        if s.eq("\n") {
            break ports[0].port_name.clone();
        }

        let i: usize = match s.trim_end().parse() {
            Ok(i) => i,
            Err(_) => {
                eprintln!("\"{}\" is not a valid integer", s);
                continue;
            }
        };

        if i >= ports.len() - 1 {
            eprintln!("\"{}\" is out of range", i);
            continue;
        }
        break ports[i].port_name.clone();
    }
}


fn main() {
    let opts: Opts = Opts::parse();

    let ports = match serialport::available_ports() {
        Ok(ports) => ports,
        Err(_e) => {
            eprintln!("No devices found.");
            return;
        }
    };

    let device = match opts.device {
        Some(device) => device,
        None => device_prompt(&ports)
    };

    eprintln!("Device: {}", device);

    let mut port = match serialport::new(&device, opts.baudrate as u32)
        .timeout(std::time::Duration::from_secs_f32(2.0))
        .open() {
        Ok(p) => p,
        Err(_e) => return
    };

    let mut buf = [0u8; 1024];
    let mut stdout = std::io::stdout();
    loop {
        // Check for errors
        let n = match port.read(&mut buf) {
            Ok(n) => n,
            Err(e) => {
                match e.kind() {
                    ErrorKind::InvalidData | ErrorKind::TimedOut => continue,
                    kind => {
                        eprintln!("can not read ({:?} - {})", kind, e);
                        return;
                    }
                }
            }
        };
        stdout.write_all(&buf[..n]).unwrap();
    }
}
