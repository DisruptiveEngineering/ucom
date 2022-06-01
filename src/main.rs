mod opts;

use std::io::{stdin, stdout, ErrorKind, Read, Write};
use std::sync::mpsc;
use std::thread::sleep;
use std::time::Duration;

use serialport::{SerialPort, SerialPortInfo, SerialPortType};

struct AsyncReader {
    rx: mpsc::Receiver<u8>,
}

impl AsyncReader {
    fn new<R: 'static + Read + Send>(reader: R) -> Self {
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(|| Self::start_reader(reader, tx));
        Self { rx }
    }

    fn start_reader<R: Read>(mut reader: R, tx: mpsc::Sender<u8>) -> std::io::Result<()> {
        let mut buf = [0u8; 1024];
        loop {
            let n = reader.read(&mut buf)?;
            for byte in buf[..n].iter() {
                if tx.send(*byte).is_err() {
                    return Ok(());
                };
            }
        }
    }
}

impl Read for AsyncReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut n = 0;
        while let Ok(data) = self.rx.try_recv() {
            if buf.len() > n + 1 {
                buf[n] = data
            } else {
                return Ok(n - 1);
            }
            n += 1;
        }

        Ok(n)
    }
}

fn port_info(port: &SerialPortInfo) -> Option<String> {
    let port_info = match &port.port_type {
        SerialPortType::UsbPort(port_info) => port_info,
        _ => return None,
    };

    let manufacturer = port_info.manufacturer.as_deref().unwrap_or("<>");
    let product = port_info.product.as_deref().unwrap_or("<>");

    Some(format!(
        "{} ({} - {})",
        port.port_name, manufacturer, product
    ))
}

/// Prints a list of all serial devices
fn list_devices(ports: &[SerialPortInfo]) {
    for (i, port) in ports.iter().enumerate() {
        let info = match port_info(port) {
            Some(info) => info,
            None => format!("{} - {:?}", port.port_name, port.port_type),
        };
        eprintln!("({}) {}", i, info);
    }
}

fn device_prompt(ports: &[SerialPortInfo]) -> String {
    let mut s = String::new();
    loop {
        eprintln!("Select device:");
        list_devices(ports);
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

        let i: isize = match s.trim_end().parse() {
            Ok(i) => i,
            Err(_) => {
                eprintln!(
                    "\"{}\" is not a valid integer",
                    s.strip_suffix('\n').unwrap_or_default()
                );
                continue;
            }
        };

        if !(0 <= i && i < ports.len() as isize) {
            eprintln!("\"{}\" is out of range", i);
            continue;
        }
        break ports[i as usize].port_name.clone();
    }
}

fn connect_to_port(path: &str, baudrate: u32) -> Option<Box<dyn SerialPort>> {
    match serialport::new(path, baudrate)
        .timeout(Duration::from_millis(10))
        .open()
    {
        Ok(p) => Some(p),
        Err(e) => {
            eprintln!("Error when connecting to \"{}\": {:?}", path, e);
            None
        }
    }
}

/// Find all USB devices
fn find_devices() -> Vec<SerialPortInfo> {
    let mut ports = match serialport::available_ports() {
        Ok(ports) => ports,
        Err(_e) => Vec::new(),
    };
    ports.retain(|port| {
        matches!(
            &port.port_type,
            SerialPortType::UsbPort(_) | SerialPortType::PciPort
        )
    });
    ports
}

fn start_terminal<R: Read>(mut port: Box<dyn SerialPort>, stdin: &mut R, outputs: &mut [Box<dyn Write>]) {
    let mut buf = [0u8; 1024];

    loop {
        // Check for errors
        let n = match port.read(&mut buf) {
            Ok(n) => n,
            Err(e) => match e.kind() {
                ErrorKind::InvalidData | ErrorKind::TimedOut => 0,
                kind => {
                    eprintln!("can not read ({:?} - {})", kind, e);
                    break;
                }
            },
        };

        for out in outputs.iter_mut() {
            out.write_all(&buf[..n]).unwrap();
        }

        // Read stdin
        match stdin.read(&mut buf) {
            Ok(n) => {
                if let Err(e) = port.write(&buf[..n]) {
                    eprintln!("can not write to serial port ({})", e)
                }
                if let Err(e) = port.flush() {
                    eprintln!("can not flush serial port ({})", e)
                }
            }
            Err(e) => eprintln!("can not read stdin ({})", e),
        }
        std::hint::spin_loop();
    }
}

fn main() {
    let opts = opts::get_opts();
    let ports = find_devices();

    // Just list all ports
    if opts.list {
        list_devices(&ports);
        return;
    }

    let device = match opts.device {
        Some(device) => device,
        None => {
            if ports.is_empty() {
                eprintln!("No devices found");
                return;
            }
            device_prompt(&ports)
        }
    };
    let mut outputs: Vec<Box<dyn Write>> = vec![Box::new(stdout())];

    if let Some(filename) = opts.outfile {
        let file = match std::fs::File::open(&filename) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("error when opening file: {}", e);
                std::fs::File::create(&filename).unwrap()
            }
        };
        outputs.push(Box::new(file));
    }

    eprintln!("Device: {}", device);
    let mut stdin = AsyncReader::new(stdin());

    loop {
        if let Some(port) = connect_to_port(&device, opts.baudrate as u32) {
            start_terminal(port, &mut stdin, outputs.as_mut_slice());
        }

        if !opts.repeat {
            return;
        }

        // Wait some time before reconnecting
        sleep(Duration::from_secs(1));
    }
}
