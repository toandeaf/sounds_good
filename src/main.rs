use crate::file_to_ouput::run as file_output_run;
use crate::input_to_file::run as input_file_run;
use crate::input_to_output::run as input_output_run;

mod file_to_ouput;
mod input_to_file;
mod input_to_output;
mod params;

fn main() {
    match file_output_run() {
        Ok(_) => {}
        e => {
            eprintln!("Example failed with the following: {:?}", e);
        }
    }
}
