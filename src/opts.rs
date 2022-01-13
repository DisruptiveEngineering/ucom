use clap::{crate_authors, crate_description, crate_version, Parser};

#[derive(Parser, Debug)]
#[clap(version = crate_version ! (), author = crate_authors ! (), about = crate_description ! ())]
pub struct Opts {
    /// Serial baudrate
    #[clap(short, long, default_value = "115200")]
    pub baudrate: usize,

    /// Device identifier
    #[clap(short, long)]
    pub device: Option<String>,

    /// Make the terminal reopen lost connections
    #[clap(short, long)]
    pub repeat: bool,

    /// Lists all available serial devices
    #[clap(short, long)]
    pub list: bool,
}

pub fn get_opts() -> Opts {
    Opts::parse()
}
