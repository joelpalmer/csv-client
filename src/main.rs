extern crate csv;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::process;

fn run() -> Result<(), Box<Error>> {
    let file_path = get_first_arg()?;
    let mut rdr = csv::Reader::from_path(file_path)?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

/// Returns 1st positional arg sent to process.
/// If there are no positional args, return an error.
fn get_first_arg() -> Result<OsString, Box<Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 arg, got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
