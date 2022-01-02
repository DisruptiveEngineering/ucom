use clap::{ArgEnum, IntoApp};
use clap_complete::{generate_to, Shell};

#[path = "src/opts.rs"]
#[allow(dead_code)]
mod opts;

fn main() {
    // Generate shell completions
    let mut app = opts::Opts::into_app();
    let bin_name: String = app.get_name().to_string();

    let comp_path = format!("{}/completions", std::env::var("OUT_DIR").unwrap());

    for shell in Shell::value_variants() {
        let out_dir = format!("{}/{}", comp_path, shell.to_string());
        std::fs::create_dir_all(&out_dir).unwrap();

        generate_to(*shell, &mut app, &bin_name, out_dir).unwrap();
    }
}
