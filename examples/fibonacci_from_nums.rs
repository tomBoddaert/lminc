use lminc::{assembler, runner};

const NUMBERS: &str = include_str!("fib_num.txt");

/// Runs the Fibonacci sequence and stops when it exceeds 100
fn main() {
    // Assemble the assembly
    let memory = assembler::assemble_from_numbers(NUMBERS).unwrap();

    // Initialise the computer
    let mut computer = runner::Computer::new(memory);

    // Run the computer
    if let Err(err) = runner::run(&mut computer) {
        println!("{err}");
    }
}
