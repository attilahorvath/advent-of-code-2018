extern crate day01;

use std::fs::File;
use std::io::{BufRead, BufReader};

use day01::*;

fn main() -> Result<(), std::io::Error> {
    let file = BufReader::new(File::open("input.txt")?);

    let changes = file
        .lines()
        .map(|line| {
            line.expect("error reading file")
                .parse()
                .expect("invalid frequency change")
        }).collect::<Vec<_>>();

    println!("Resulting frequency: {}", resulting_frequency(&changes));
    println!("Duplicate frequency: {}", duplicate_frequency(&changes));

    Ok(())
}
