use std::{error::Error, process};

use args::RustodoArgs;
use clap::Parser;

mod args;

fn main() {
    let args = RustodoArgs::parse();

    if let Err(e) = run(args){
        eprint!("{e}");
        process::exit(1);
    }
}

fn run(args : RustodoArgs) -> Result<(), Box<dyn Error>>{¨
    

    Ok(())
}
