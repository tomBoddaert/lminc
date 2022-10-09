use std::io::stdin;

/// The representation of the computer
pub struct Computer {
    pub state: ComputerState,
    pub memory: [u16; 100],
    pub counter: u8,
    pub register: u16,
    pub negative_flag: bool,
}

/// The computer's state
#[derive(Clone, Copy, Debug)]
pub enum ComputerState {
    Running,
    Halted,
    ReachedEnd,
}

/// Errors for the computer runtime
#[derive(Clone, Copy, Debug)]
pub enum ComputerError {
    Done(ComputerState),
    InvalidInstruction,
    BadInput,
}

impl Default for Computer {
    fn default() -> Self {
        Self {
            state: ComputerState::Running,
            memory: [0; 100],
            counter: 0,
            register: 0,
            negative_flag: false,
        }
    }
}

/// Runs a computer in a loop, until it halts or errors
pub fn run(computer: &mut Computer) -> Result<(), ComputerError> {
    loop {
        // Step the computer and check for errors
        if let Err(err) = step(computer) {
            match err {
                // If the computer is done, stop
                ComputerError::Done(_) => return Ok(()),
                ComputerError::BadInput => {
                    // If there is a bad input, output that
                    println!("LMinC: Invalid input!");
                }
                // Return any other error
                _ => return Err(err),
            }
        }
    }
}

/// Attempts to execute one fetch-execute cycle on the computer
pub fn step(computer: &mut Computer) -> Result<(), ComputerError> {
    // If the computer is not running, error
    if !matches!(computer.state, ComputerState::Running) {
        return Err(ComputerError::Done(computer.state));
    }

    // If the counter is past the end, set the state and stop
    if computer.counter > 99 {
        computer.state = ComputerState::ReachedEnd;
        return Ok(());
    }

    // Get the current instruction and increment the instruction counter
    let instruction = computer.memory[computer.counter as usize];
    computer.counter += 1;

    // Execute the instruction
    match instruction / 100 {
        0 => {
            // HALT
            computer.state = ComputerState::Halted;
        }
        1 => {
            // ADD
            computer.register = computer
                .register
                .wrapping_add(computer.memory[(instruction % 100) as usize])
                % 1000;
        }
        2 => {
            // SUBTRACT
            computer.negative_flag =
                computer.register < computer.memory[(instruction % 100) as usize];
            computer.register = computer
                .register
                .wrapping_sub(computer.memory[(instruction % 100) as usize])
                % 1000;
        }
        3 => {
            // STORE
            computer.memory[(instruction % 100) as usize] = computer.register;
        }
        5 => {
            // LOAD
            computer.register = computer.memory[(instruction % 100) as usize];
        }
        6 => {
            // BRANCH
            computer.counter = (instruction % 100) as u8;
        }
        7 => {
            // BRANCH on ZERO
            if computer.register == 0 {
                computer.counter = (instruction % 100) as u8;
            }
        }
        8 => {
            // BRANCH on POSITIVE
            if !computer.negative_flag {
                computer.counter = (instruction % 100) as u8;
            }
        }
        9 => match instruction % 100 {
            1 => {
                // INPUT
                let mut input = String::new();
                if match stdin().read_line(&mut input) {
                    Ok(_) => {
                        // Read a line and attempt to convert it to a number
                        if let Ok(value) = input[..input.len() - 1].parse::<u16>() {
                            // If the number is too large, error
                            if value > 999 {
                                true
                            } else {
                                // Set the register to the input
                                computer.register = value;
                                false
                            }
                        } else {
                            true
                        }
                    }
                    Err(_) => true,
                } {
                    // Decrement counter to stay on same instruction and error
                    computer.counter -= 1;
                    return Err(ComputerError::BadInput);
                }
            }
            2 => {
                // OUTPUT
                println!("{}", computer.register);
            }
            _ => {}
        },
        _ => {
            // If the instruction is invalid, decrement the counter to stay on the instruction, set the state and error
            computer.counter -= 1;
            return Err(ComputerError::InvalidInstruction);
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fibonacci_no_output() {
        let memory = [
            605, 000, 001, 000, 100, 501, 102, 303, 502, 301, 503, 302, 204, 816, 605, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
        ];

        let mut computer = Computer {
            memory,
            ..Default::default()
        };
        run(&mut computer).unwrap();

        let expected_memory = [
            605, 089, 144, 144, 100, 501, 102, 303, 502, 301, 503, 302, 204, 816, 605, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
        ];

        assert!(
            computer
                .memory
                .iter()
                .zip(expected_memory)
                .all(|(number, expected)| *number == expected),
            "Computer did not run Fibonacci correctly!"
        );

        assert!(
            matches!(computer.state, ComputerState::Halted),
            "Computer failed to set halt state!"
        );
    }
}
