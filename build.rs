use clap::{ArgEnum, IntoApp};
use clap_complete::{generate_to, Shell};

#[path = "src/opts.rs"]
#[allow(dead_code)]
mod opts;

fn main() {
    // Generate shell completions
    let mut app = opts::Opts::into_app();
    let bin_name: String = app.get_name().to_string();

    let comp_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap_or_default())
        .join(std::path::PathBuf::from("completions"));

    for shell in Shell::value_variants() {
        let out_dir = comp_path.join(shell.to_string());
        std::fs::create_dir_all(&out_dir).unwrap();

        generate_to(*shell, &mut app, &bin_name, out_dir).unwrap();
    }
}
