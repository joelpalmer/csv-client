extern crate csv;
use std::io;
use std::process;

fn main() {
    // Create a CSV parser that reads data from stdin
    let mut rdr = csv::Reader::from_reader(io::stdin());

    // Loop over each record
    for result in rdr.records() {
        // examine results
        // if no problem, print record
        // else, print error message & quit
        match result {
            Ok(record) => println!("{:?}", record),
            Err(err) => {
                println!("error reading CSV from <stdin>: {}", err);
                process::exit(1);
            }
        }
    }
}
