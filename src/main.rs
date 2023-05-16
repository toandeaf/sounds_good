use std::env::args;

use crate::file_to_ouput::run as file_to_output_run;
use crate::input_to_file::run as input_to_file_run;
use crate::input_to_output::run as input_to_output_run;

mod file_to_ouput;
mod input_to_file;
mod input_to_output;
mod params;

fn main() {
    let args: Vec<String> = args().collect();
    let arg = &args[1];

    println!("Peep this {}", arg);

    match arg.as_str() {
        "input_output" => input_to_output_run(),
        "input_file" => input_to_file_run(),
        "file_output" => file_to_output_run(),
        _ => input_to_output_run(),
    }
    .unwrap();
}
