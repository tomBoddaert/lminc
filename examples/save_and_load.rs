#[cfg(feature = "std")]
mod example {
    use std::{assert_eq, env::temp_dir, fs::remove_file};

    use lminc::{
        assembler,
        file::{load_from_path, save_to_path},
    };
    use uuid::Uuid;

    /// Imported assembly
    const ASSEMBLY: &str = include_str!("fib.txt");

    pub fn main() {
        // Assemble the assembly
        let memory = assembler::assemble_from_text(ASSEMBLY)
            .expect("failed to parse")
            .expect("failed to assemble");

        // Get a temp path to write the file to
        // This could have an extension of ".bin"
        let mut path = temp_dir();
        path.push(format!("lminc-example-{}", Uuid::new_v4()));

        // Write the memory to the file
        save_to_path(path.clone(), memory).expect("failed to save the program to file");

        // Read the memory from the file
        let loaded_memory = load_from_path(path.clone()).expect("failed to load the file");

        // Try to delete the file
        if let Err(error) = remove_file(path.clone()) {
            eprintln!(
                "Warning: Failed to remove file ({})!\nError: {error}",
                path.to_str().expect("failed to convert the path to a str")
            );
        }

        assert_eq!(
            loaded_memory, memory,
            "loaded memory is not the same as the saved memory"
        );
        println!("Successfully saved a program to a file and loaded it.")
    }
}

#[cfg(not(feature = "std"))]
mod example {
    pub fn main() {
        eprintln!("To run this example, the `std` feature must be enabled!");
    }
}

/// Saves memory to a file and loads it
fn main() {
    example::main();
}
