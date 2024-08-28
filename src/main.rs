use std::{collections::VecDeque, env, fmt::Display};
use {enums::*, helper::*};

mod enums;
mod helper;
#[cfg(test)]
mod tests;

fn main() {
    // Collect command line arguments into a vector of strings
    let args = env::args().collect::<Vec<_>>();

    // Provide a default empty string in case no arguments are provided
    let default_input = String::default();

    // Get the first argument if available, otherwise use the default empty string
    let input = args.first().unwrap_or(&default_input);

    // Determine and print if the input XML string is valid or not
    println!("{}", determine_xml(input));
}
