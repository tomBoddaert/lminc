use core::{fmt, mem};
#[cfg(feature = "std")]
use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

use crate::{computer::Memory, file::MAX_FILE_SIZE};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Loading Errors
pub enum Error {
    /// The buffer is more than [`MAX_FILE_SIZE`] bytes long
    BufferTooLarge(usize),
    /// A number in the decoded buffer is too large (> 999)
    InvalidNumber(usize, u16),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BufferTooLarge(size) => write!(
                f,
                "The buffer is too long bytes long ({size} bytes > {MAX_FILE_SIZE} bytes)"
            ),
            Self::InvalidNumber(index, number) => write!(
                f,
                "A number in the decoded buffer is too large (index {index}, {number} > 999)"
            ),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[allow(clippy::module_name_repetitions)]
/// Load [Memory] from a saved buffer
///
/// # Errors
/// See [Error]
pub fn load_from_buffer(buffer: &[u8]) -> Result<Memory, Error> {
    if buffer.len() > MAX_FILE_SIZE {
        return Err(Error::BufferTooLarge(buffer.len()));
    }

    // Initialise the memory, address index and offset
    let mut memory = [0; 100];
    let mut address = 0;
    let mut offset: u8 = 2;

    for (index, byte) in buffer.iter().enumerate() {
        // Get the parts of the byte in the first and second 10 bit number
        let lower = u16::from(*byte) >> (10 - offset);
        let upper = (u16::from(*byte) << offset) % 0b100_0000_0000;

        // Add the lower
        memory[address] |= lower;

        if index != 0 {
            // If the latest complete number added is too large, error
            if memory[address] > 999 {
                return Err(Error::InvalidNumber(address, memory[address]));
            }

            // Increment the address
            address += 1;
        }

        // Add the upper
        memory[address] |= upper;

        // Increase the offset, resetting it and decreasing the address index if past 8
        offset += 2;
        if offset == 10 {
            address -= 1;
            offset = 0;
        }
    }

    // The numbers have already been checked and are not
    //  over 999, so it is safe to transmute
    Ok(unsafe { mem::transmute(memory) })
}

#[cfg(feature = "std")]
#[derive(Debug)]
/// File-specific loading errors
pub enum FromFileError {
    /// Encountered an Os error while performing a file system operation
    IoError(io::Error),
    /// The file is more than [MAX_FILE_SIZE] bytes long
    FileTooLarge(u64),
    /// The contents of the file could not be loaded, see [Error]
    LoadError(Error),
}

#[cfg(feature = "std")]
impl fmt::Display for FromFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(error) => write!(
                f,
                "An OS error occurred while reading a file!\nError: {error}"
            ),
            Self::FileTooLarge(size) => write!(
                f,
                "The file is too large ({size} bytes > {MAX_FILE_SIZE} bytes)"
            ),
            Self::LoadError(error) => fmt::Display::fmt(error, f),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FromFileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::IoError(error) => Some(error),
            Self::LoadError(error) => Some(error),
            Self::FileTooLarge(_) => None,
        }
    }
}

#[cfg(feature = "std")]
impl From<io::Error> for FromFileError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

#[cfg(feature = "std")]
impl From<Error> for FromFileError {
    fn from(value: Error) -> Self {
        Self::LoadError(value)
    }
}

#[cfg(feature = "std")]
#[allow(clippy::module_name_repetitions)]
/// Load [Memory] from the given file
///
/// This function will move the cursor inside file,
/// unless you know what you are doing, do not use `file` after calling this function
///
/// # Errors
/// See [`FromFileError`]
pub fn load_from_file(file: &mut File) -> Result<Memory, FromFileError> {
    // Make sure the file is not too large
    let file_size = file.metadata()?.len();
    if file_size > MAX_FILE_SIZE as u64 {
        return Err(FromFileError::FileTooLarge(file_size));
    }

    // Read the file
    let mut buffer = [0; MAX_FILE_SIZE];
    let bytes_read = file.read(&mut buffer)?;

    // Load it
    load_from_buffer(&buffer[..bytes_read]).map_err(FromFileError::from)
}

#[cfg(feature = "std")]
/// Load [Memory] from a file given the path str
///
/// # Errors
/// See [`FromFileError`]
pub fn load(path: &str) -> Result<Memory, FromFileError> {
    load_from_file(&mut File::open(path)?)
}

#[cfg(feature = "std")]
#[allow(clippy::module_name_repetitions)]
/// Load [Memory] from a file given the path
///
/// # Errors
/// See [`FromFileError`]
pub fn load_from_path(path: PathBuf) -> Result<Memory, FromFileError> {
    load_from_file(&mut File::open(path)?)
}

#[cfg(test)]
mod test {
    use std::{
        env::temp_dir,
        fs::{self, File},
        io::Write,
    };

    use uuid::Uuid;

    use crate::file::{load, MAX_FILE_SIZE};

    use super::load_from_buffer;

    #[test]
    fn empty_buffer() {
        let buffer = [];

        // Load the memory from the buffer
        let memory = load_from_buffer(&buffer[..]).expect("failed to load from buffer");

        assert!(
            memory.iter().all(|number| u16::from(*number) == 0),
            "Empty buffer did not load all zeros!"
        );
    }

    #[test]
    fn full_buffer() {
        let mut buffer = [0; MAX_FILE_SIZE];
        // Set the MSB in each 10-bit sequence to 1
        buffer.iter_mut().enumerate().for_each(|(index, byte)| {
            let shift = (2 * index) % 10;
            if shift < 8 {
                *byte = 0b1000_0000_u8 >> shift;
            }
        });

        // Load the memory from the buffer
        let memory = load_from_buffer(&buffer[..]).expect("failed to load from buffer");

        assert!(
            memory.iter().all(|number| u16::from(*number) == 512),
            "Full buffer did not load all 512s!"
        );
    }

    #[test]
    fn empty() {
        // Get a new path in the temp directory
        let mut path = temp_dir();
        path.push(format!("lminc-test-{}", Uuid::new_v4()));

        // Create the empty file
        let file = File::create(path.clone()).expect("failed to create file");
        file.sync_all().expect("failed to sync file data");
        drop(file);

        let path_str = path.to_str().expect("failed to convert path to str");

        // Load the memory from the file
        let memory = load(path_str).expect("failed to load memory from file");

        // Try to delete the file
        if let Err(error) = fs::remove_file(path.clone()) {
            eprintln!("Warning: Failed to remove file ({path_str})!\nError: {error}");
        }

        assert!(
            memory.iter().all(|number| u16::from(*number) == 0),
            "Empty file did not load all zeros!"
        );
    }

    #[test]
    fn full() {
        let mut buffer = [0; MAX_FILE_SIZE];
        // Set the MSB in each 10-bit sequence to 1
        buffer.iter_mut().enumerate().for_each(|(index, byte)| {
            let shift = 8 - 2 * ((index + 1) % 5);
            if shift < 8 {
                *byte = 1 << shift;
            }
        });

        // Get a new path in the temp directory
        let mut path = temp_dir();
        path.push(format!("lminc-test-{}", Uuid::new_v4()));

        // Create the file and write the buffer to it
        let mut file = File::create(path.clone()).expect("failed to create file");
        file.write_all(&buffer[..])
            .expect("failed to write buffer to file");
        file.sync_all().expect("failed to sync file data");
        drop(file);

        let path_str = path.to_str().expect("failed to convert path to str");

        // Load the memory from the file
        let memory = load(path_str).expect("failed to load memory from file");

        // Try to delete the file
        if let Err(error) = fs::remove_file(path.clone()) {
            eprintln!("Warning: Failed to remove file ({path_str})!\nError: {error}");
        }

        assert!(
            memory.iter().all(|number| u16::from(*number) == 256),
            "Full file did not load all 256s!"
        );
    }
}
