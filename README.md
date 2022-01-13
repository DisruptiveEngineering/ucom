# ucom
Serial console written in rust


## Installation instructions

1. Clone the repository, and enter it.
2. Build and install binaries (eg. `cargo install --path .` to install it in `~/.cargo/bin/`)

### Optionally copy completions and generate man pages:
 
ZSH completions, symlink:

    sudo ln -s $(pwd)/target/release/completions/zsh/_ucom /usr/share/zsh/site-functions/_ucom

Generate and install man page:

    help2man ucom | gzip -c | sudo tee /usr/share/man/man1/ucom.1.gz > /dev/null
