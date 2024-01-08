use std::io::{self, BufWriter};
use std::{env, process};

use openapi::code;

fn main() {
    let file = match env::args_os().nth(1) {
        Some(file) => file,
        None => {
            eprintln!("missing file argument");
            process::exit(1);
        }
    };

    let spec = match openapi::read_from_file(file) {
        Ok(spec) => spec,
        Err(err) => {
            eprintln!("failed to parse specification: {}", err);
            process::exit(1);
        }
    };

    let output = BufWriter::new(io::stdout());
    let mut warnings = Vec::new();
    code::Generator::new(code::Rust)
        .write_to(&spec, output, &mut warnings)
        .expect("failed to write spec");
    for warning in warnings {
        eprintln!("warning: {:?}", warning);
    }
}
