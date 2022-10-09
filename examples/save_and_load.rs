use lminc::{assembler, loader};
use std::env::temp_dir;
use uuid::Uuid;

const ASSEMBLY: &str = include_str!("fib.txt");

/// Runs the Fibonacci sequence and stops when it exceeds 100
fn main() {
    // Assemble the assembly
    let memory = assembler::assemble_from_assembly(ASSEMBLY).unwrap();

    // Get a temp path to write the file to
    // This would usually have an extension of ".bin"
    let mut path = temp_dir();
    path.push(Uuid::new_v4().to_string());
    let bin_path = path.to_str().unwrap();

    // Write the memory to the file
    loader::write_to_file(bin_path, memory).unwrap();

    // Read it from the file
    let read_memory = loader::read_from_file(bin_path).unwrap().unwrap();

    assert_eq!(memory, read_memory);
    println!("Successfully wrote a program to a file and read it.")
}
