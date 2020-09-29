## Purpose

Find kdbx4 password from a list of passwords.

## Compilation

Install [Rust](https://rust-lang.org) using [rustup](https://rustup.rs/) or use the Rust package from your operating system.

Compile `findkeepassword` using

```bash
$ cargo build --release
```

The executable is then in `target/release/findkeepassword`. Install it somewhere in your `$PATH`.

## Usage

```bash
$ findkeepassword --help
findkeepassword 1.0.0
Samuel Tardieu <sam@rfc1149.net>
Find your lost keepass master password by trying many from a list

USAGE:
    findkeepassword [FLAGS] [OPTIONS] <FILE> <PASSWORDS>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v               Display attempts

OPTIONS:
    -k, --key-file <FILE>    The optional key file

ARGS:
    <FILE>         The kdbx file
    <PASSWORDS>    The file containing the list of possible passwords
```

Checking a password on a large file may take some significant time. `findkeepassword` will use all the available cores on your computer in order to speedup the search.
