use clap::{crate_authors, crate_description, crate_version, Parser};

#[derive(Default, Debug, Clone, PartialEq, Eq, clap::ValueEnum)]
pub enum FlushOpt {
    /// Do not flush, relies on external reader (ie terminal emulator)
    Never,

    /// Will always flush after data is written to the output drains
    #[default]
    Always,
}

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

    /// Add timestamp to the output margin
    #[clap(short, long)]
    pub timestamp: bool,

    /// Log content to file
    #[clap(short, long)]
    pub outfile: Option<String>,

    /// Print line if regex gets match
    #[clap(long)]
    pub regex_match: Option<String>,

    /// Filter line if regex gets match
    #[clap(long)]
    pub regex_filter: Option<String>,

    /// Prefix filename with timestamp of program start
    #[clap(long)]
    pub prefix_filename_with_timestamp: bool,

    /// If set, ucom will force flush the output drains whenever there is data
    #[clap(value_enum, short, long, default_value_t = FlushOpt::Always)]
    pub flush: FlushOpt,
}

pub fn get_opts() -> Opts {
    Opts::parse()
}
