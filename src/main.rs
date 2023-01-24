use clap::Parser;
use eyre::Context;
use kdbx_rs::CompositeKey;
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

    let key_file = args
        .key_file
        .map(std::fs::read)
        .transpose()
        .context("Cannot open key file")?;
    let passwords =
        io::BufReader::new(File::open(&args.passwords).context("Cannot open passwords file")?)
            .lines()
            .collect::<Result<Vec<_>, _>>()
            .context("Error reading passwords file")?;
    match find_password(&args.file, passwords, key_file, args.verbose) {
        Some(p) => println!("Found working password: {p}"),
        None => println!("No working password found"),
    }
    Ok(())
}

fn find_password<P>(
    file: &P,
    passwords: Vec<String>,
    key_file: Option<Vec<u8>>,
    verbose: bool,
) -> Option<String>
where
    P: AsRef<Path> + Sync,
{
    passwords.into_par_iter().find_any(|p| {
        if verbose {
            println!("Checking {p}");
        };
        is_right_password(file, p, key_file.clone())
    })
}

fn is_right_password<P>(file: &P, password: &str, key_file: Option<Vec<u8>>) -> bool
where
    P: AsRef<Path>,
{
    let key = CompositeKey::new(Some(password.to_owned()), key_file);
    let db = kdbx_rs::open(file).context("Cannot open database").unwrap();
    db.unlock(&key).is_ok()
}
