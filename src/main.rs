use std::{
    error::Error,
    fs::{File, OpenOptions},
    path::Path,
    process,
};

use args::RustodoArgs;
use clap::Parser;

mod args;

fn main() {
    let args = RustodoArgs::parse();

    if let Err(e) = run(args) {
        eprint!("{e}");
        process::exit(1);
    }
}

fn run(args: RustodoArgs) -> Result<(), Box<dyn Error>> {
    let file = get_csv()?;

    Ok(())
}

/// Gets the csv file. If it doesn't exist, it creates one and writes the header to it.
fn get_csv() -> Result<File, Box<dyn Error>> {
    let path = Path::new("./data.csv");

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(path)?;

    if file.metadata()?.len() == 0 {
        let header = vec![String::from("Title"), String::from("Is done")];
        write_single_to_csv(&file, header)?;
    }

    Ok(file)
}

/// Writes a single record to the csv.
fn write_single_to_csv(file: &File, record: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(file);

    wtr.write_record(record)?;
    wtr.flush()?;

    Ok(())
}
