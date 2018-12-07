extern crate day03;

use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use day03::*;

fn main() -> Result<(), Error> {
    let file = BufReader::new(File::open("input.txt")?);

    let claims = file
        .lines()
        .map(|line| {
            line.expect("error reading file")
                .parse()
                .expect("invalid claim")
        }).collect::<Vec<_>>();

    let fabric = Fabric::new(&claims);

    println!("Covered area: {}", fabric.covered_area());

    println!(
        "Intact claim ID: {}",
        fabric.intact_claim_id(&claims).expect("no intact claims")
    );

    Ok(())
}
