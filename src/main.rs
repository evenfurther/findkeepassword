use clap::Parser;
use kdbx4::{CompositeKey, Kdbx4};
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[clap(version, author, about)]
struct Args {
    // The optional key file
    #[clap(short, long)]
    key_file: Option<PathBuf>,
    // The kdbx file
    file: PathBuf,
    // The file containing the list of possible passwords
    passwords: PathBuf,
    // Verbose output
    #[clap(short, long)]
    verbose: bool,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    args.key_file
        .as_ref()
        .map(|name| File::open(name).expect("cannot open key file"));
    let passwords = io::BufReader::new(File::open(&args.passwords)?)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;
    match find_password(&args.file, passwords, args.key_file.as_ref(), args.verbose) {
        Some(p) => println!("Found working password: {p}"),
        None => println!("No working password found"),
    }
    Ok(())
}

fn find_password<P1, P2>(
    file: &P1,
    passwords: Vec<String>,
    key_file: Option<&P2>,
    verbose: bool,
) -> Option<String>
where
    P1: AsRef<Path> + Sync,
    P2: AsRef<Path> + Sync,
{
    passwords.into_par_iter().find_any(|p| {
        if verbose {
            println!("Checking {p}");
        };
        is_right_password(file, p, key_file)
    })
}

fn is_right_password<P1, P2>(file: &P1, password: &str, key_file: Option<&P2>) -> bool
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let key = CompositeKey::new(Some(password), key_file).unwrap();
    Kdbx4::open(file, key).is_ok()
}
