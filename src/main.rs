extern crate csv;
use std::io;

fn main() {
    // Create a CSV parser that reads data from stdin
    let mut rdr = csv::Reader::from_reader(io::stdin());

    // Loop over each record
    for result in rdr.records() {
        // todo: handle possible errors
        let record = result.expect("a CSV record");

        // Print debug version of the record
        println!("{:?}", record);
    }
}
