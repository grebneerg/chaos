extern crate chaos;
extern crate clap;

use chaos::RecursiveFunction;
use clap::{App, Arg};

use std::io::Result as IoResult;
use std::str::FromStr;

fn main() -> IoResult<()> {
    let app = App::new("cycles")
        .version("0.1")
        .author("Jack Greenberg")
        .about("Iterates a quadratic recursive function.")
        .arg(
            Arg::with_name("a")
                .short("a")
                .required(true)
                .help("the a value of the function.")
                .takes_value(true),
        );
    let matches = app.get_matches();

    let a = f64::from_str(matches.value_of("a").unwrap()).expect("Invalid value for a.");

    println!(
        "{}",
        RecursiveFunction::new(|x| a * x * (1.0 - x), 0.4).end_behavior(100)
    );

    Ok(())
}
