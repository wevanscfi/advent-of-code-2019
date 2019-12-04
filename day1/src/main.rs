extern crate math;

use std::env;
use std::fs::File;
use math::round;
use std::error::Error;
use std::ffi::OsString;
use std::process;

/// Strucutre to track each modules mass and required fuel
struct Module {
  mass: f64,
  fuel_required: f64,
}

/// Return the required amount of fuel for a given mass
fn calc_fuel(mass: f64) -> f64 {
  return round::floor(mass / 3f64, 0) - 2f64;
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut modules = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let mass: f64 = record[0].parse().expect("Not a number!");
        let module = Module {
            mass: mass,
            fuel_required: calc_fuel(mass) 
        };
        modules.push(module);
    }
    
    for x in modules.iter() {
        println!("Mass: {} Fuel: {}", x.mass, x.fuel_required);
    }

    let total_fuel: f64 = modules.iter().map(|x|x.fuel_required).sum();
    println!("Total fuel required: {}", total_fuel);
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

