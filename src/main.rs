extern crate csv;
use std::error::Error;
use std::io;
use std::process;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<Error>> {
    // Create a CSV parser that reads data from stdin
    let mut rdr = csv::Reader::from_reader(io::stdin());

    // Loop over each record
    for result in rdr.records() {
        // examine results
        // if no problem, print record
        // else, print error message & quit
        match result {
            Err(err) => return Err(From::from(err)),
            Ok(record) => println!("{:?}", record),
        }
    }

    Ok(())
}
