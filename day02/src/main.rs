extern crate day02;

use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use day02::*;

fn main() -> Result<(), Error> {
    let file = BufReader::new(File::open("input.txt")?);

    let ids = file
        .lines()
        .map(|line| line.expect("error reading file"))
        .collect::<Vec<_>>();

    println!("Checksum: {}", checksum(&ids));
    println!("Common letters: {}", common_letters(&ids));

    Ok(())
}
