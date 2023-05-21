use core::mem;
#[cfg(feature = "std")]
use std::{
    fs::File,
    io::{self, Write},
    path::PathBuf,
};

use crate::computer::Memory;

use super::MAX_FILE_SIZE;

#[allow(clippy::module_name_repetitions)]
/// Save the [Memory] to the buffer and return a trimmed version of it
pub fn save_to_buffer(buffer: &mut [u8; MAX_FILE_SIZE], memory: Memory) -> &[u8] {
    let memory: [u16; 100] = unsafe { mem::transmute(memory) };

    // Initialise the byte index and bit offset
    let mut index = 0;
    let mut offset: u8 = 2;

    for number in memory {
        // Get the parts in the first and second byte
        #[allow(clippy::cast_possible_truncation)]
        let lower = (number >> offset) as u8;
        #[allow(clippy::cast_possible_truncation)]
        let upper = (number << (8 - offset)) as u8;

        // Put the numbers into the buffer
        buffer[index] |= lower;
        index += 1;
        buffer[index] |= upper;

        // Increase the offset, adding one to the index and resetting if past 8
        offset += 2;
        if offset == 10 {
            index += 1;
            offset = 2;
        }
    }

    // Get the index of the last non-zero byte
    let last_index = buffer
        .iter()
        .enumerate()
        .rev()
        .find_map(|(last_index, number)| if *number == 0 { None } else { Some(last_index) });

    // Return the trimmed buffer
    last_index.map_or_else(|| &buffer[..0], |last_index| &buffer[..=last_index])
}

#[cfg(feature = "std")]
#[allow(clippy::module_name_repetitions)]
/// Save the [Memory] to the given file
///
/// This function will move the cursor inside file,
/// unless you know what you are doing, do not use `file` after calling this function
///
/// # Errors
/// [`io::Error`] - file system error
pub fn save_to_file(file: &mut File, memory: Memory) -> io::Result<()> {
    // Create a buffer
    let mut buffer = [0; MAX_FILE_SIZE];
    // Write the memory to the buffer and get the trimmed slice
    let buffer_trimmed = save_to_buffer(&mut buffer, memory);

    // Write the buffer slice to the memory
    file.write_all(buffer_trimmed)
}

#[cfg(feature = "std")]
/// Save the [Memory] to a file given the path str
///
/// # Errors
/// [`io::Error`] - file system error
pub fn save(path: &str, memory: Memory) -> io::Result<()> {
    save_to_file(&mut File::create(path)?, memory)
}

#[cfg(feature = "std")]
#[allow(clippy::module_name_repetitions)]
/// Save the [Memory] to a file given the path
///
/// # Errors
/// [`io::Error`] - file system error
pub fn save_to_path(path: PathBuf, memory: Memory) -> io::Result<()> {
    save_to_file(&mut File::create(path)?, memory)
}

#[cfg(test)]
mod test {
    use core::assert_eq;
    use std::{
        env::temp_dir,
        fs::{self, File},
        io::Read,
    };

    use uuid::Uuid;

    use crate::{
        file::{save, MAX_FILE_SIZE},
        num3::ThreeDigitNumber,
    };

    use super::save_to_buffer;

    #[test]
    fn empty_buffer() {
        let memory = [ThreeDigitNumber::ZERO; 100];
        let mut buffer = [0; MAX_FILE_SIZE];

        // Write the memory to the buffer
        let buffer_trimmed = save_to_buffer(&mut buffer, memory);

        assert_eq!(
            buffer_trimmed.len(),
            0,
            "Zeroed memory did not save an empty buffer!",
        );
    }

    #[test]
    fn full_buffer() {
        let memory = [unsafe { ThreeDigitNumber::from_unchecked(1) }; 100];
        let mut buffer = [0; MAX_FILE_SIZE];

        // Write the memory to the buffer
        let buffer_trimmed = save_to_buffer(&mut buffer, memory);
        assert_eq!(
            buffer_trimmed.len(),
            MAX_FILE_SIZE,
            "Full memory did not fill up buffer!"
        );

        assert!(
            buffer_trimmed.iter().enumerate().all(|(index, byte)| {
                *byte == {
                    let shift = 8 - 2 * (index % 5);
                    if shift < 8 {
                        1 << shift
                    } else {
                        0
                    }
                }
            }),
            "Full memory did not save all 1s!"
        );
    }

    #[test]
    fn empty() {
        // Get a new path in the temp directory
        let mut path = temp_dir();
        path.push(format!("lminc-test-{}", Uuid::new_v4()));

        // Create the file
        let file = File::create(path.clone()).expect("failed to create file");
        file.sync_all().expect("failed to sync file data");
        drop(file);

        let path_str = path.to_str().expect("failed to convert path to str");

        // Save the memory to the file
        let memory = [ThreeDigitNumber::ZERO; 100];
        save(path_str, memory).expect("failed to write to file");

        // Open the file
        let file = File::open(path.clone()).expect("failed to open file");

        assert_eq!(
            file.metadata().expect("failed to get file metadata").len(),
            0,
            "Zeroed memory did not save an empty file!",
        );

        // Try to delete the file
        if let Err(error) = fs::remove_file(path.clone()) {
            eprintln!("Warning: Failed to remove file ({path_str})!\nError: {error}");
        }
    }

    #[test]
    fn full() {
        // Get a new path in the temp directory
        let mut path = temp_dir();
        path.push(format!("lminc-test-{}", Uuid::new_v4()));

        // Create the file
        let file = File::create(path.clone()).expect("failed to create file");
        file.sync_all().expect("failed to sync file data");
        drop(file);

        let path_str = path.to_str().expect("failed to convert path to str");

        // Save the memory to the file
        let memory = [unsafe { ThreeDigitNumber::from_unchecked(1) }; 100];
        save(path_str, memory).expect("failed to write to file");

        // Open the file
        let mut file = File::open(path.clone()).expect("failed to open file");
        let mut buffer = Vec::new();

        assert_eq!(
            file.metadata().expect("failed to get file metadata").len(),
            MAX_FILE_SIZE as u64,
            "Full memory did not fill up file!"
        );

        file.read_to_end(&mut buffer).expect("failed to read file");

        assert!(
            buffer.iter().enumerate().all(|(index, byte)| {
                *byte == {
                    let shift = 8 - 2 * (index % 5);
                    if shift < 8 {
                        1 << shift
                    } else {
                        0
                    }
                }
            }),
            "Full memory did not save all 1s!"
        );
    }
}
