//!
//!

#[macro_use]
extern crate clap;

use std::io::{BufReader, BufRead, ErrorKind};
use clap::Clap;
use serialport::SerialPortInfo;
use std::io::{stdin, stdout, Write};

#[derive(Clap)]
#[clap(version = crate_version ! (), author = crate_authors ! (), about = crate_description ! ())]
struct Opts {
    #[clap(short, long, default_value = "3_000_000")]
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
        println!("Select device:");
        for (i, port) in ports.iter().enumerate() {
            let info = match port_info(&port) {
                Some(info) => info,
                None => continue
            };

            println!("({}) {}", i, info);
        }
        s.clear();

        print!("Choose: ");
        let _ = stdout().flush();
        match stdin().read_line(&mut s) {
            Ok(n) => {}
            Err(e) => {
                eprintln!("Error ({})", e);
                continue;
            }
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
    eprintln!("Value for config: {}", opts.baudrate);

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

    println!("Device: {}", device);

    let port = match serialport::new(&device, opts.baudrate as u32)
        .timeout(std::time::Duration::from_secs_f32(2.0))
        .open() {
        Ok(p) => p,
        Err(_e) => return
    };

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
