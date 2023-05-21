#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::perf,
    clippy::cargo
)]

use lminc::helper::case_insensitive::Str;
use std::env;

mod error;
use error::Error;

mod subcommands;
use subcommands::{assemble, assemble_numbers, mem_dump, run, run_assembly, run_numbers, test};

macro_rules! HELP_TEXT {
    () => {
        "\
Usage: {} <subcommand> <arguments...>

Subcommands:
    help
        Display this message

    assemble <in path> <out path>
        Assemble the assembly from an input and output a binary file

    assembleNumbers <in path> <out path>
        Assemble the numbers from an input and output a binary file

    run <path>
        Run a binary file

    runAssembly <path>
        Run an assembly file

    runNumbers <path>
        Run a number file

    memDump <path>
        Read the memory from a binary file and print it out

    test <test path> <bin path>
        Run the tests in a CSV file

    version
        Print the version number

    author
        Information about the author
"
    };
}

const AUTHOR_TEXT: &str = "\
https://github.com/tomboddaert/lminc
This program was created by:

    Tom Boddaert
        https://tomBoddaert.com/
";

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

pub fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    let Some(subcommand) = args.get(1) else {
        // If no command line arguments were given:
        eprintln!(
            "Usage '{} <subcommand> <arguments...>'\n Use '{} help' for help",
            args[0], args[0]
        );
        return;
    };

    // Check the number of arguments and return a usage string if there are not the correct number
    macro_rules! check_arguments {
        ( $number:expr, $usage:expr, $fn:path ) => {
            if args.len() != $number {
                Err(Error::Usage(format!($usage, args[0])))
            } else {
                $fn(&args)
            }
        };
    }

    // Get the first command line argument, error if None
    if let Err(error) = match Str::from(subcommand.as_str()) {
        sc if sc == "help" => {
            print!(HELP_TEXT!(), args[0]);
            Ok(())
        }
        sc if sc == "assemble" => {
            check_arguments!(4, "{} assemble <in path> <out path>", assemble)
        }
        sc if sc == "assembleNumbers" => check_arguments!(
            4,
            "{} assembleNumbers <in path> <out path>",
            assemble_numbers
        ),
        sc if sc == "run" => check_arguments!(3, "{} run <path>", run),
        sc if sc == "runAssembly" => check_arguments!(3, "{} runAssembly <path>", run_assembly),
        sc if sc == "runNumbers" => check_arguments!(3, "{} runNumbers <path>", run_numbers),
        sc if sc == "memDump" => check_arguments!(3, "{} memDump <path>", mem_dump),
        sc if sc == "test" => check_arguments!(4, "{} test <test path> <bin path>", test),
        sc if sc == "version" => {
            println!("LMinC version {}", VERSION.unwrap_or("unknown"));
            Ok(())
        }
        sc if sc == "author" => {
            print!("{AUTHOR_TEXT}");
            Ok(())
        }
        _ => Err("Unknown subcommand".into()),
    } {
        eprintln!("{error}");
    }
}
