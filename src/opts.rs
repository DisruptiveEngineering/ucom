use clap::Parser;

#[derive(Default, Debug, Clone, PartialEq, Eq, clap::ValueEnum)]
pub enum FlushOpt {
    /// Do not flush, relies on external reader (ie terminal emulator)
    Never,

    /// Will always flush after data is written to the output drains
    #[default]
    Always,
}

#[derive(Parser, Debug)]
#[command(version, author, about)]
pub struct Opts {
    /// Serial baudrate
    #[arg(short, long, default_value_t = 115_200)]
    pub baudrate: usize,

    /// Device identifier
    #[arg(short, long)]
    pub device: Option<String>,

    /// Make the terminal reopen lost connections
    #[arg(short, long)]
    pub repeat: bool,

    /// Lists all available serial devices
    #[arg(short, long)]
    pub list: bool,

    /// Add timestamp to the output margin
    #[arg(short, long)]
    pub timestamp: bool,

    /// Log content to file
    #[arg(short, long)]
    pub outfile: Option<String>,

    /// Print line if regex gets match
    #[arg(long)]
    pub regex_match: Option<String>,

    /// Filter line if regex gets match
    #[arg(long)]
    pub regex_filter: Option<String>,

    /// Prefix filename with timestamp of program start
    #[arg(long, default_value_t = true)]
    pub prefix_filename_with_timestamp: bool,

    /// If set, ucom will force flush the output drains whenever there is data
    #[arg(value_enum, short, long, default_value_t = FlushOpt::Always)]
    pub flush: FlushOpt,
}

pub fn get_opts() -> Opts {
    Opts::parse()
}
