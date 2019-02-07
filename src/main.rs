mod bits;
mod constants;
mod micro;

use constants::*;
use micro::*;

use std::collections::HashMap;
use yaml_rust::YamlLoader;
use yaml_rust::Yaml;
use clap::*;
use simple_logger;
use log::*;

fn main() {
    let matches = App::new("Micro Assembler")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .help("Sets the input file to use")
                .default_value("input.yaml"),
        )
        .arg(
            Arg::with_name("dispatch")
                .short("d")
                .long("dispatch-output")
                .help("Sets the file to output dispatch to")
                .default_value("dispatch1"),
        )
        .arg(
            Arg::with_name("microcode")
                .short("m")
                .long("microcode-output")
                .help("Sets the file to output microcode to")
                .default_value("microcode"),
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .takes_value(true)
                .possible_values(&["disabled", "info", "warn", "debug", "error", "trace"])
                .default_value("warn")
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    let verbosity = matches.value_of("verbosity").unwrap();

    let log_level = match verbosity {
        "disabled" => None,
        "info" => Some(Level::Info),
        "warn" => Some(Level::Warn),
        "debug" => Some(Level::Debug),
        "error" => Some(Level::Error),
        "trace" => Some(Level::Trace),
        _ => None,
    };

    if let Some(level) = log_level {
        if let Err(e) = simple_logger::init_with_level(level) {
            eprintln!("Failed to initialize logging: {}", e);
        }
    }

    trace!("{:?}", matches);

    let input_filename = matches.value_of("input").unwrap();
    let dispatch_filename = matches.value_of("dispatch").unwrap();
    let microcode_filename = matches.value_of("microcode").unwrap();

    let string = match std::fs::read_to_string(input_filename) {
        Ok(res) => res,
        Err(e) => {
            error!("Failed to open input file: {:?}", e);
            return;
        }
    };

    let input = match YamlLoader::load_from_str(&string) {
        Ok(res) => res,
        Err(e) => {
            error!("Failed parse input file: {:?}", e);
            return;
        }
    };

    debug!("{:#?}", input);

    let input = &input[0];

    let hash = input
        .clone()
        .into_hash()
        .expect("Error: Root value must be a HashMap");

    let mut operations = HashMap::new();

    for (key, value) in hash {
        match key {
            Yaml::String(ref string) => {
                if VALID_OPERATIONS.contains(&string.as_str()) {
                    if let Yaml::Array(array_val) = value {
                        operations.insert(string.clone(), array_val);
                    } else {
                        warn!(
                            "Unexpected value for instruction '{}'. Found '{:?}' instead.",
                            string, value
                        );
                    }
                } else {
                    warn!("Unknown instruction: {}", string)
                }
            }
            _ => {
                warn!("Unexpected item '{:?}'", key);
            }
        }
    }

    debug!("{:#?}", operations);

    let instructions = collapse_instructions(operations);

    let dispatch = generate_dispatch(instructions.clone());

    if let Err(e) = dispatch.write_to_file(dispatch_filename) {
        error!("Failed to write to the output dispatch file: {:?}", e);
    }

    if let Err(e) = write_microcode(microcode_filename, instructions) {
        error!("Failed to write to the output microcode file: {:?}", e);
    }
}
