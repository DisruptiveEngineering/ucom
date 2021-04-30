//!
//!

#[macro_use]
extern crate clap;

use clap::Clap;
use serialport::{SerialPort, SerialPortInfo};
use std::io::{stdin, stdout, Write};
use std::io::{ErrorKind, Read};
use std::sync::mpsc;
use std::thread::sleep;
use std::time::Duration;

#[derive(Clap, Debug)]
#[clap(version = crate_version ! (), author = crate_authors ! (), about = crate_description ! ())]
struct Opts {
    #[clap(short, long, default_value = "3000000")]
    baudrate: usize,

    #[clap(short, long)]
    device: Option<String>,

    #[clap(short, long)]
    repeat: bool,
}

struct AsyncReader {
    rx: mpsc::Receiver<u8>,
}

impl AsyncReader {
    fn new<R: 'static + std::io::Read + Send>(reader: R) -> Self {
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(|| Self::start_reader(reader, tx));
        Self { rx }
    }

    fn start_reader<R: std::io::Read>(mut reader: R, tx: mpsc::Sender<u8>) -> std::io::Result<()> {
        let mut buf = [0u8; 1024];
        loop {
            let n = reader.read(&mut buf)?;
            for byte in buf[..n].iter() {
                if let Err(_) = tx.send(*byte) {
                    return Ok(());
                };
            }
        }
    }
}

impl std::io::Read for AsyncReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut n = 0;
        while let Ok(data) = self.rx.try_recv() {
            n += 1;
            if buf.len() > n {
                buf[n] = data
            } else {
                return Ok(n - 1);
            }
        }
        Ok(n)
    }
}

fn port_info(port: &SerialPortInfo) -> Option<String> {
    let port_info = match &port.port_type {
        serialport::SerialPortType::UsbPort(port_info) => port_info,
        _ => return None,
    };

    let manufacturer = port_info.manufacturer.as_deref().unwrap_or("<>");
    let product = port_info.product.as_deref().unwrap_or("<>");

    Some(format!(
        "{} ({} - {})",
        port.port_name, manufacturer, product
    ))
}

fn device_prompt(ports: &Vec<SerialPortInfo>) -> String {
    let mut s = String::new();
    loop {
        eprintln!("Select device:");
        for (i, port) in ports.iter().enumerate() {
            let info = match port_info(&port) {
                Some(info) => info,
                None => continue,
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

fn connect_to_port(path: &str, baudrate: u32) -> Option<Box<dyn SerialPort>> {
    match serialport::new(path, baudrate).open() {
        Ok(p) => Some(p),
        Err(e) => {
            eprintln!("Error when connecting to \"{}\": {:?}", path, e);
            None
        }
    }
}

fn start_terminal<R: std::io::Read>(mut port: Box<dyn SerialPort>, stdin: &mut R) {
    let mut buf = [0u8; 1024];
    let mut stdout = std::io::stdout();

    let mut stdin_buf: Vec<u8> = Vec::with_capacity(1024);

    loop {
        // Check for errors
        let n = match port.read(&mut buf) {
            Ok(n) => n,
            Err(e) => match e.kind() {
                ErrorKind::InvalidData | ErrorKind::TimedOut => continue,
                kind => {
                    eprintln!("can not read ({:?} - {})", kind, e);
                    break;
                }
            },
        };
        stdout.write_all(&buf[..n]).unwrap();

        // Read stdin
        match stdin.read(&mut buf) {
            Ok(n) => {
                for byte in buf[..n].iter() {
                    if byte == &b'\n' {
                        if let Err(e) = port.write_all(&stdin_buf) {
                            eprintln!("can not write to serial port ({})", e)
                        }
                        stdin_buf.clear();
                    } else {
                        stdin_buf.push(*byte);
                    }
                }
            }
            Err(e) => {
                eprintln!("can not read stdin ({})", e)
            }
        }
    }
}

fn main() {
    let opts: Opts = Opts::parse();
    /* ....[]< debug!("opts: {:?}", opts); >[].... */
    dbg!("opts: {:?}", &opts);

    let ports = match serialport::available_ports() {
        Ok(ports) => ports,
        Err(_e) => {
            eprintln!("No devices found.");
            return;
        }
    };

    let device = match opts.device {
        Some(device) => device,
        None => device_prompt(&ports),
    };

    eprintln!("Device: {}", device);
    let mut stdin = AsyncReader::new(std::io::stdin());

    loop {
        if let Some(port) = connect_to_port(&device, opts.baudrate as u32) {
            start_terminal(port, &mut stdin);
        }

        if !opts.repeat {
            return;
        }

        // Small sleep before connecting to port again
        sleep(Duration::from_secs(1));
    }
}
