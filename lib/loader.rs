use std::io::{Read, Write};

/// Errors for the loading process
#[derive(Debug)]
pub enum Error {
    FileTooLarge,
    NumberTooLarge(usize, u16),
}

/// Write a memory instance to a file
pub fn write_to_file(path: &str, memory: [u16; 100]) -> std::io::Result<()> {
    // Initialise the file buffer, byte index and bit offset
    let mut buffer: [u8; 125] = [0; 125];
    let mut byte = 0;
    let mut offset: u8 = 2;

    for number in memory {
        // Get the parts in the first and second byte
        let a = (number >> offset) as u8;
        let b = (number << (8 - offset)) as u8;

        // Put the number into the buffer
        buffer[byte] |= a;
        byte += 1;
        buffer[byte] |= b;

        // Increase the offset, adding one to the byte and resetting if past 8
        offset += 2;
        if offset == 10 {
            byte += 1;
            offset = 2;
        }
    }

    // Get the index of the last non-zero byte
    let mut last_byte_i: usize = 0;
    for (i, &number) in buffer.iter().enumerate().rev() {
        if number != 0 {
            last_byte_i = i;
            break;
        }
    }

    // Get the slice of the buffer without the trailing zeros
    let prog = &buffer[0..=last_byte_i];

    // Write the slice to the file
    let mut file = std::fs::File::create(path)?;
    file.write_all(prog)?;

    Ok(())
}

/// Reads a memory instance from a file
pub fn read_from_file(path: &str) -> std::io::Result<Result<[u16; 100], Error>> {
    // Open the file and read it to a buffer
    let mut file = std::fs::File::open(path)?;
    let mut buf = [0; 125];
    let bytes_read = file.read(&mut buf)?;

    // Initialise the memory, address index and the bit offset
    let mut memory = [0; 100];
    let mut address = 0;
    let mut offset: u8 = 2;

    // Iterate over the buffer
    for (i, byte) in buf.iter().take(bytes_read).enumerate() {
        // Get the parts in the first and second 10 bit number
        let a = (*byte as u16) >> (10 - offset);
        let b = ((*byte as u16) << offset) % 0b100_0000_0000;

        // Put the number into the memory
        memory[address] |= a;
        if i != 0 {
            // If the latest complete number added is too large, error
            if memory[address] > 999 {
                return Ok(Err(Error::NumberTooLarge(address, memory[address])));
            }
            // Increment the address, erroring if it is too large
            address += 1;
            if address > 99 {
                return Ok(Err(Error::FileTooLarge));
            }
        }
        memory[address] |= b;

        // Increase the offset, resetting it and decreasing the address index if past 8
        offset += 2;
        if offset == 10 {
            address -= 1;
            offset = 0;
        }
    }

    Ok(Ok(memory))
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env::temp_dir;
    use uuid::Uuid;

    #[test]
    fn empty() {
        let mut path = temp_dir();
        path.push(Uuid::new_v4().to_string());
        let memory = [0; 100];

        write_to_file(path.to_str().unwrap(), memory).unwrap();
        let out_memory = read_from_file(path.to_str().unwrap()).unwrap().unwrap();

        assert!(
            memory
                .iter()
                .zip(out_memory)
                .all(|(expected, num)| *expected == num),
            "Empty memory written and then read was different!"
        );
    }

    #[test]
    fn full() {
        let mut path = temp_dir();
        path.push(Uuid::new_v4().to_string());
        let memory = [902; 100];

        write_to_file(path.to_str().unwrap(), memory).unwrap();
        let out_memory = read_from_file(path.to_str().unwrap()).unwrap().unwrap();

        assert!(
            memory
                .iter()
                .zip(out_memory)
                .all(|(expected, num)| *expected == num),
            "Full memory written and then read was different!"
        );
    }

    #[test]
    fn fibonacci() {
        let mut path = temp_dir();
        path.push(Uuid::new_v4().to_string());
        let memory = [
            605, 000, 001, 000, 100, 501, 102, 902, 303, 502, 301, 503, 302, 204, 816, 605, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
        ];

        write_to_file(path.to_str().unwrap(), memory).unwrap();
        let out_memory = read_from_file(path.to_str().unwrap()).unwrap().unwrap();

        assert!(
            memory
                .iter()
                .zip(out_memory)
                .all(|(expected, num)| *expected == num),
            "Fibonacci memory written and then read was different!"
        );
    }
}
