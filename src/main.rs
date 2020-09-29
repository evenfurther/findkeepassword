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
    let keyfile = matches.value_of("key-file");
    let kdbx = matches.value_of("FILE").unwrap();
    let verbose = matches.is_present("verbose");
    let passwords = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;
    match passwords.into_par_iter().find_any(|p| {
        if verbose {
            println!("Checking {}", p);
        };
        let key = CompositeKey::new(Some(p), keyfile).unwrap();
        Kdbx4::open(kdbx, key).is_ok()
    }) {
        Some(p) => println!("Found working password: {}", p),
        None => println!("No working password found"),
    }
    Ok(())
}
