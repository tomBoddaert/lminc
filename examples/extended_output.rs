#[cfg(all(feature = "std", feature = "extended"))]
mod example {
    use lminc::{assembler, runner::stdio::Runner};

    /// Imported assembly
    const ASSEMBLY: &str = include_str!("extended_output.txt");

    pub fn main() {
        // Assemble the assembly
        let memory = assembler::assemble_from_text(ASSEMBLY)
            .expect("failed to parse")
            .expect("failed to assemble");

        // Create the runner, which also initialises the computer
        let mut runner = Runner::new(memory);

        // Run the computer
        if let Err(error) = runner.run() {
            eprintln!("{error}");
        }
    }
}

#[cfg(not(all(feature = "std", feature = "extended")))]
mod example {
    pub fn main() {
        eprintln!("To run this example, the `std` and `extended` features must be enabled!");
    }
}

/// Prints a message
fn main() {
    example::main();
}
