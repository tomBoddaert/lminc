use lminc::{
    assembler,
    computer::Computer,
    file, number_assembler,
    runner::{stdio::Runner, tester::StdTest},
};
use std::{
    fs::{self, File},
    io::Read,
    mem,
};

use crate::error::Error;

macro_rules! read_and_assemble {
    ( $path:expr, $fn:path ) => {{
        // Load the file
        let mut file = File::open($path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        // Assemble
        $fn(&buffer)
    }};
}

pub fn assemble(args: &[String]) -> Result<(), Error> {
    // If <in path> == <out path>, error
    if args[2] == args[3] {
        return Err("Cannot overwrite input assembly with output binary!".into());
    }

    // Load the file and assemble
    let memory = read_and_assemble!(&args[2], assembler::assemble_from_text)??;

    // Write the assembled code to the output file
    file::save(&args[3], memory)?;

    Ok(())
}

pub fn assemble_numbers(args: &[String]) -> Result<(), Error> {
    // If <in path> == <out path>, error
    if args[2] == args[3] {
        return Err("Cannot overwrite input numbers with output binary!".into());
    }

    // Load the file and assemble
    let memory = read_and_assemble!(
        &args[2],
        number_assembler::NumberAssembler::assemble_from_text
    )?;

    // Write the assembled code to the output file
    file::save(&args[3], memory)?;

    Ok(())
}

pub fn run(args: &[String]) -> Result<(), Error> {
    // Read the memory from the file
    let memory = file::load(&args[2])?;

    // Initialise the computer
    let mut runner = Runner::new(memory);

    runner.run()?;

    Ok(())
}

pub fn run_assembly(args: &[String]) -> Result<(), Error> {
    // Load the file and assemble
    let memory = read_and_assemble!(&args[2], assembler::assemble_from_text)??;

    // Initialise the computer
    let mut runner = Runner::new(memory);

    runner.run()?;

    Ok(())
}

pub fn run_numbers(args: &[String]) -> Result<(), Error> {
    // Load the file and assemble
    let memory = read_and_assemble!(
        &args[2],
        number_assembler::NumberAssembler::assemble_from_text
    )?;

    // Initialise the computer
    let mut runner = Runner::new(memory);

    runner.run()?;

    Ok(())
}

pub fn mem_dump(args: &[String]) -> Result<(), Error> {
    // Read the memory from the file
    let memory = file::load(&args[2])?;

    // Cast to a u16 array to fix formatting
    let memory: [u16; 100] = unsafe { mem::transmute(memory) };

    println!("{memory:?}");

    Ok(())
}

pub fn test(args: &[String]) -> Result<(), Error> {
    // Read the CSV file
    let mut file = fs::File::open(&args[2])?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let tests = StdTest::from_csv(&buffer);

    // Read the memory from the file
    let memory = file::load(&args[3])?;

    // Initialise the computer
    let mut computer = Computer::new(memory);

    let mut failed = 0;
    let mut succeeded = 0;

    for test in tests {
        let test = test?;
        test.name.map_or_else(
            || println!("Running test:"),
            |name| println!("Running test '{name}':"),
        );

        // Reset the computer and the test
        computer.reset();
        let cycles = match test.run(&mut computer) {
            Ok(cycles) => {
                println!("  Test ran successfully.\n  Program {}", computer.state());
                succeeded += 1;
                cycles
            }
            Err(error) => {
                println!("  Error: {}", error.1);
                failed += 1;
                error.1 .0 .0
            }
        };

        // Print the number of cycles
        println!("  Program stopped after {cycles} fetch-execute cycles.\n",);
    }

    // Print success and failure
    println!("{succeeded} tests ran successfully.\n{failed} tests failed.");

    // Print successful
    if failed == 0 {
        println!("All tests run successfully!");
    } else {
        println!("Some tests failed!");
    }

    Ok(())
}
