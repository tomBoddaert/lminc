#[cfg(feature = "std")]
mod example {
    use lminc::{
        assembler,
        computer::{Computer, State},
    };

    /// Imported assembly
    const ASSEMBLY: &str = include_str!("van_eck.txt");

    pub fn main() {
        // Assemble the assembly
        let memory = assembler::assemble_from_text(ASSEMBLY)
            .expect("failed to parse")
            .expect("failed to assemble");

        // Initialise the computer
        let mut computer = Computer::new(memory);

        let mut buffer = Vec::with_capacity(152);

        loop {
            // Step the computer and compare the state
            match computer.step() {
                // If it is running normally, do nothing
                State::Running => (),
                // If it awaiting output, take the output and push it to the buffer
                State::AwaitingOutput => {
                    let output = computer
                        .output()
                        .expect("failed to get an output from a computer");

                    buffer.push(output);
                }
                // If it has halted, break
                State::Halted => break,
                // If it is anything else, panic
                state => panic!("Unexpected state: {state:?}"),
            }
        }

        // Print the buffer
        buffer.into_iter().for_each(|number| print!("{number}, "));
        println!("...");
    }
}

#[cfg(not(feature = "std"))]
mod example {
    pub fn main() {
        eprintln!("To run this example, the `std` feature must be enabled!");
    }
}

/// Runs the Van Eck sequence and buffers the output until the end
fn main() {
    example::main();
}
