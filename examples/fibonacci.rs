#[cfg(feature = "std")]
mod example {
    use lminc::{assembler, runner::stdio::Runner};

    /// Imported assembly
    const ASSEMBLY: &str = include_str!("fib.txt");

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

#[cfg(not(feature = "std"))]
mod example {
    pub fn main() {
        eprintln!("To run this example, the `std` feature must be enabled!");
    }
}

/// Runs the Fibonacci sequence and stops when it exceeds 100
fn main() {
    example::main();
}
