use anyhow::Error;
use clap::{load_yaml, App};
use kdbx4::{CompositeKey, Kdbx4};
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Error> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let file = File::open(matches.value_of("PASSWORDS").unwrap())?;
    let key_file = matches.value_of("key-file");
    key_file.map(|name| File::open(name).expect("cannot open key file"));
    let kdbx = matches.value_of("FILE").unwrap();
    let verbose = matches.is_present("verbose");
    let passwords = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;
    match find_password(kdbx, passwords, key_file, verbose) {
        Some(p) => println!("Found working password: {}", p),
        None => println!("No working password found"),
    }
    Ok(())
}

fn find_password(
    file: &str,
    passwords: Vec<String>,
    key_file: Option<&str>,
    verbose: bool,
) -> Option<String> {
    passwords.into_par_iter().find_any(|p| {
        if verbose {
            println!("Checking {}", p);
        };
        is_right_password(file, p, key_file)
    })
}

fn is_right_password(file: &str, password: &str, key_file: Option<&str>) -> bool {
    let key = CompositeKey::new(Some(password), key_file).unwrap();
    Kdbx4::open(file, key).is_ok()
}
