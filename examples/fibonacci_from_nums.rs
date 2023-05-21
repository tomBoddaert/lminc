#[cfg(feature = "std")]
mod example {
    use lminc::{number_assembler::NumberAssembler, runner::stdio::Runner};

    /// Imported assembly
    const ASSEMBLY: &str = include_str!("fib_num.txt");

    pub fn main() {
        // Assemble the numbers
        let memory =
            NumberAssembler::assemble_from_text(ASSEMBLY).expect("failed to assemble from numbers");

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

/// Runs the Fibonacci sequence and stops when it exceeds 100.
/// Assembled from numbers
fn main() {
    example::main();
}
