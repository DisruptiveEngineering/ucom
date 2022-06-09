# ucom
Serial console written in rust


## Installation instructions

1. Run the following command:
```bash
cargo install --git https://github.com/DisruptiveEngineering/ucom
```

### Optionally copy completions and generate man pages:
Clone the repository, and enter it.

Installing the ZSH completions with symlink:

    sudo ln -s $(pwd)/target/release/completions/zsh/_ucom /usr/share/zsh/site-functions/_ucom


Installing the ZSH completions with symlink:

    sudo ln -s $(pwd)/target/release/completions/zsh/_ucom /usr/share/zsh/site-functions/_ucom

Installing the fish completions with symlink:

```fish
ln -s (pwd)/target/release/completions/fish/ucom.fish ~/.config/fish/completions/ucom.fish
```

Generate and install man page:

    help2man ucom | gzip -c | sudo tee /usr/share/man/man1/ucom.1.gz > /dev/null
