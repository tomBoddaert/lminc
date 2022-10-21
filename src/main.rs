#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::perf,
    clippy::cargo
)]

use lminc::{assembler, loader, runner};
use std::{env, fs, io::BufRead};

macro_rules! HELP_TEXT {
    () => {
        "Usage: {} <subcommand> <arguments...>

Subcommands:
    help
        Display this message

    assemble <in path> <out path>
        Assemble the assembly from the input and output a binary file

    assembleNumbers <in path> <out path>
        Assemble the numbers from the input and output a binary file

    run <path>
        Run the binary file in the input

    runAssembly <path>
        Assemble the assembly from the input and run it

    runNumbers <path>
        Assemble the numbers from the input and run it

    memDump <path>
        Read the memory from a binary file and print it out
        
    test <test path> <bin path>
        Run the tests in the CSV file

    author
        Information about the author
"
    };
}

macro_rules! AUTHOR_TEXT {
    () => {
        "This program was created by:

    Tom Boddaert
        https://tomBoddaert.github.io/
"
    };
}

/// Reads a file and assembles it using the assemble function provided
macro_rules! read_and_assemble {
    ( $path:expr, $assemble:path ) => {{
        // Read the file to a string
        let contents = match fs::read_to_string($path) {
            Ok(contents) => contents,
            Err(err) => return Err(err.to_string()),
        };

        // Assemble the file
        match $assemble(&contents) {
            Ok(memory) => memory,
            Err(err) => return Err(err.to_string()),
        }
    }};
}

/// Run the command line mode
#[allow(clippy::too_many_lines)]
pub fn main() -> Result<(), String> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Check the number of arguments and return a usage string if there are not the correct number
    macro_rules! check_arguments {
        ( $number:expr, $usage:expr ) => {
            if args.len() != $number {
                return Err(format!($usage, args[0]));
            }
        };
    }

    // Get the first command line argument, error if None
    match args.get(1) {
        Some(subcommand) => match subcommand.as_str() {
            "help" => {
                print!(HELP_TEXT!(), args[0]);
            }
            "assemble" => {
                // If there are not enough arguments, error
                check_arguments!(4, "Usage: '{} assemble <in path> <out path>'");

                // If <in path> == <out path>, error
                if args[2] == args[3] {
                    return Err("Cannot overwrite input assembly with output binary!".to_owned());
                }

                // Load the file and assemble
                let memory = read_and_assemble!(&args[2], assembler::assemble_from_assembly);

                // Write the assembled code to the output file
                if let Err(err) = loader::write_to_file(&args[3], memory) {
                    return Err(err.to_string());
                }
            }
            "assembleNumbers" => {
                // If there are not enough arguments, error
                check_arguments!(4, "Usage: '{} assembleNumbers <in path> <out path>'");

                // If <in path> == <out path>, error
                if args[2] == args[3] {
                    return Err("Cannot overwrite input numbers with output binary!".to_owned());
                }

                // Load the file and assemble
                let memory = read_and_assemble!(&args[2], assembler::assemble_from_numbers);

                // Write the assembled code to the output file
                if let Err(err) = loader::write_to_file(&args[3], memory) {
                    return Err(err.to_string());
                }
            }
            "run" => {
                // If there are not enough arguments, error
                check_arguments!(3, "Usage: '{} run <path>'");

                // Read the memory from the file
                let memory: [u16; 100] = match loader::read_from_file(&args[2]) {
                    Ok(Ok(memory_from_file)) => memory_from_file,
                    Ok(Err(err)) => return Err(err.to_string()),
                    Err(err) => return Err(err.to_string()),
                };

                // Initialise the computer
                let mut computer = runner::Computer::new(memory);

                // Run the computer
                if let Err(err) = runner::run(&mut computer) {
                    println!("{err}");
                }
            }
            "runAssembly" => {
                // If there are not enough arguments, error
                check_arguments!(3, "Usage: '{} runAssembly <path>'");

                // Load the file and assemble
                let memory = read_and_assemble!(&args[2], assembler::assemble_from_assembly);

                // Initialise the computer
                let mut computer = runner::Computer::new(memory);

                // Run the computer
                if let Err(err) = runner::run(&mut computer) {
                    println!("{err}");
                }
            }
            "runNumbers" => {
                // If there are not enough arguments, error
                check_arguments!(3, "Usage: '{} runNumbers <path>'");

                // Load the file and assemble
                let memory = read_and_assemble!(&args[2], assembler::assemble_from_numbers);

                // Initialise the computer
                let mut computer = runner::Computer::new(memory);

                // Run the computer
                if let Err(err) = runner::run(&mut computer) {
                    println!("{err}");
                }
            }
            "memDump" => {
                // If there are not enough arguments, error
                check_arguments!(3, "Usage: '{} memDump <path>'");

                // Read the memory from the file
                let memory: [u16; 100] = match loader::read_from_file(&args[2]) {
                    Ok(Ok(memory_from_file)) => memory_from_file,
                    Ok(Err(err)) => return Err(err.to_string()),
                    Err(err) => return Err(err.to_string()),
                };

                println!("{memory:?}");
            }
            "test" => {
                // If there are not enough arguments, error
                check_arguments!(4, "Usage: '{} test <test path> <bin path>'");

                // Read the CSV file
                let file = match fs::File::open(&args[2]) {
                    Ok(file) => file,
                    Err(err) => return Err(err.to_string()),
                };

                // Create a vector of names and tests
                let mut tests: Vec<(String, runner::Tester)> = Vec::new();

                // For each line of csv:
                for line in std::io::BufReader::new(file).lines() {
                    match line {
                        // Create a test from the line and add it to the tests
                        Ok(line) => tests.push(match runner::Tester::from_csv_line(&line) {
                            Ok(test) => test,
                            Err(err) => return Err(err),
                        }),
                        Err(err) => return Err(err.to_string()),
                    }
                }

                // Read the memory from the file
                let memory: [u16; 100] = match loader::read_from_file(&args[3]) {
                    Ok(Ok(memory_from_file)) => memory_from_file,
                    Ok(Err(err)) => return Err(err.to_string()),
                    Err(err) => return Err(err.to_string()),
                };

                // Initialise the computer
                let mut computer = runner::Computer::new(memory);

                for (name, tester) in tests {
                    // Reset the computer and run each test
                    computer.reset();
                    computer.tester = Some(Box::new(tester));
                    if let Err(err) = runner::run(&mut computer) {
                        return Err(format!("Test '{name}': {}", err));
                    }
                }

                println!("All tests run successfully!");
            }
            "author" => {
                print!(AUTHOR_TEXT!());
            }
            _ => return Err("Unknown subcommand".to_owned()),
        },
        None => {
            // If no command line arguments were given:
            return Err(format!(
                "Usage: '{} <subcommand> <arguments...>', Use '{} help' for help",
                args[0], args[0]
            ));
        }
    }

    Ok(())
}
