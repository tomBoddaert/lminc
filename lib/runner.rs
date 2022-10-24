use std::collections::LinkedList;
use std::io::stdin;

/// The representation of the computer
pub struct Computer {
    pub state: ComputerState,
    pub memory: [u16; 100],
    pub counter: u8,
    pub register: u16,
    pub negative_flag: bool,
    pub extended_mode: bool,
    pub tester: Option<Box<Tester>>,
}

impl Default for Computer {
    fn default() -> Self {
        Self {
            state: ComputerState::Running,
            memory: [0; 100],
            counter: 0,
            register: 0,
            negative_flag: false,
            extended_mode: false,
            tester: None,
        }
    }
}

impl Computer {
    /// Creates a new computer from a `[u16; 100]` memory
    pub fn new(memory: [u16; 100]) -> Self {
        Self {
            memory,
            ..Self::default()
        }
    }

    /// Resets the computer but not the memory
    pub fn reset(&mut self) {
        self.state = ComputerState::Running;
        self.counter = 0;
        self.register = 0;
        self.negative_flag = false;
        self.extended_mode = false;
        self.tester = None;
    }
}

/// The computer's state
#[derive(Clone, Copy, Debug)]
pub enum ComputerState {
    Running,
    Halted,
    ReachedEnd,
}

impl std::fmt::Display for ComputerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ComputerState::*;

        match self {
            Running => write!(f, "is running")?,
            Halted => write!(f, "has halted")?,
            ReachedEnd => write!(f, "has reached the end of its memory")?,
        }

        Ok(())
    }
}

/// Errors for the computer runtime
#[derive(Clone, Copy, Debug)]
pub enum ComputerError {
    Done(ComputerState),
    InvalidInstruction(u8, u16),
    BadInput,
    TestError(TesterError),
}

impl std::fmt::Display for ComputerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ComputerError::*;

        match self {
            Done(state) => write!(f, "The computer {state}!")?,
            InvalidInstruction(i, instruction) => {
                write!(f, "Instruction {i} ({instruction}) is invalid!")?
            }
            BadInput => write!(f, "An invalid input was entered!")?,
            TestError(error) => write!(f, "Tester: {}", error)?,
        }

        Ok(())
    }
}

/// The tester for the computer
pub struct Tester {
    pub fe_cycles: u32,
    pub max_fe_cycles: u32,
    pub inputs: LinkedList<u16>,
    pub ainputs: LinkedList<char>,
    pub outputs: LinkedList<u16>,
    pub aoutputs: LinkedList<char>,
}

impl Default for Tester {
    fn default() -> Self {
        Self {
            fe_cycles: 0,
            max_fe_cycles: 10000,
            inputs: LinkedList::new(),
            ainputs: LinkedList::new(),
            outputs: LinkedList::new(),
            aoutputs: LinkedList::new(),
        }
    }
}

impl Tester {
    /// Creates a new tester from a line of csv in the format
    ///   `name;comma seperated inputs;comma seperated outputs;maximum fetch-execute cycles`
    pub fn from_csv_line(text: &str) -> Result<(String, Self), String> {
        let mut sections = text.split(';');

        let name = match sections.next() {
            Some(name) => name.to_owned(),
            None => return Err("Wrong number of sections in csv line!".to_owned()),
        };

        let inputs = match sections.next() {
            Some(inputs) => inputs.split(',').filter(|&input| !input.is_empty()),
            None => return Err("Wrong number of sections in csv line!".to_owned()),
        };

        let outputs = match sections.next() {
            Some(outputs) => outputs.split(',').filter(|&output| !output.is_empty()),
            None => return Err("Wrong number of sections in csv line!".to_owned()),
        };

        let max_cycles = match sections.next() {
            Some(max_cycles) => max_cycles,
            None => return Err("Wrong number of sections in csv line!".to_owned()),
        };

        let mut tester = Tester::default();

        for input in inputs {
            if crate::assembler::DECIMAL_NUMBER.is_match(input) {
                let value = match input.parse::<u16>() {
                    Ok(value) => value,
                    Err(_) => return Err("Invalid number input in csv!".to_owned()),
                };

                tester.inputs.push_back(value);
            } else {
                if input.len() != 1 {
                    return Err("Invalid ascii input in csv!".to_owned());
                }

                tester.ainputs.push_back(input.chars().next().unwrap());
            }
        }

        for output in outputs {
            if crate::assembler::DECIMAL_NUMBER.is_match(output) {
                let value = match output.parse::<u16>() {
                    Ok(value) => value,
                    Err(_) => return Err("Invalid number input in csv!".to_owned()),
                };

                tester.outputs.push_back(value);
            } else {
                if output.len() != 1 {
                    return Err("Invalid ascii input in csv!".to_owned());
                }

                tester.aoutputs.push_back(output.chars().next().unwrap());
            }
        }

        match max_cycles.parse::<u32>() {
            Ok(max_fe_cycles) => tester.max_fe_cycles = max_fe_cycles,
            Err(_) => return Err("Invalid maximum number of cycles in csv!".to_owned()),
        }

        Ok((name, tester))
    }
}

/// Errors for the tester
#[derive(Clone, Copy, Debug)]
pub enum TesterError {
    NoTesterAttatched,
    RunOutOfInputs,
    RunOutOfAInputs,
    RunOutOfOutputs,
    RunOutOfAOutputs,
    DifferentOutput(u16, u16),
    DifferentAOutput(char, char),
    ExpectedMoreInputs,
    ExpectedMoreAInputs,
    ExpectedMoreOutputs,
    ExpectedMoreAOutputs,
    RunOutOfCycles,
}

impl std::fmt::Display for TesterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TesterError::*;

        match self {
            NoTesterAttatched => write!(f, "No tester attatched!")?,
            RunOutOfInputs => write!(f, "Run out of inputs!")?,
            RunOutOfAInputs => write!(f, "Run out of ASCII inputs!")?,
            RunOutOfOutputs => write!(f, "Run out of outputs!")?,
            RunOutOfAOutputs => write!(f, "Run out of ASCII outputs!")?,
            DifferentOutput(exp, fnd) => write!(f, "Expected {}, got {}!", exp, fnd)?,
            DifferentAOutput(exp, fnd) => write!(f, "Expected '{}', got '{}'!", exp, fnd)?,
            ExpectedMoreInputs => write!(f, "Expected more inputs!")?,
            ExpectedMoreAInputs => write!(f, "Expected more ASCII inputs!")?,
            ExpectedMoreOutputs => write!(f, "Expected more outputs!")?,
            ExpectedMoreAOutputs => write!(f, "Expected more ASCII outputs!")?,
            RunOutOfCycles => write!(f, "Run out of cycles!")?,
        }

        Ok(())
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

        // If a tester is attatched, check that all inputs and ouputs have been consumed
        if let Some(tester) = &mut computer.tester {
            if !tester.inputs.is_empty() {
                return Err(ComputerError::TestError(TesterError::ExpectedMoreInputs));
            }
            if !tester.ainputs.is_empty() {
                return Err(ComputerError::TestError(TesterError::ExpectedMoreAInputs));
            }
            if !tester.outputs.is_empty() {
                return Err(ComputerError::TestError(TesterError::ExpectedMoreOutputs));
            }
            if !tester.aoutputs.is_empty() {
                return Err(ComputerError::TestError(TesterError::ExpectedMoreAOutputs));
            }
        }

        return Ok(());
    }

    // Get the current instruction and increment the instruction counter
    let instruction = computer.memory[computer.counter as usize];
    computer.counter += 1;

    // Execute the instruction
    match instruction / 100 {
        0 if computer.counter == 1 && instruction == 10 => {
            // If the first number is 010, enable extended mode
            computer.extended_mode = true;
        }
        0 => {
            // HALT
            computer.state = ComputerState::Halted;
        }
        1 => {
            // ADD
            computer.register =
                (computer.register + computer.memory[(instruction % 100) as usize]) % 1000;
        }
        2 => {
            // SUBTRACT
            computer.negative_flag =
                computer.register < computer.memory[(instruction % 100) as usize];
            computer.register = ((computer.register as i16
                - computer.memory[(instruction % 100) as usize] as i16)
                % 1000) as u16;
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
        9 if !matches!(computer.tester, None) => {
            // I/O with tester
            let tester = match &mut computer.tester {
                Some(tester) => tester,
                None => return Err(ComputerError::TestError(TesterError::NoTesterAttatched)),
            };

            match instruction % 100 {
                1 => {
                    // INPUT with tester
                    let mut input = match tester.inputs.pop_front() {
                        Some(input) => input,
                        None => return Err(ComputerError::TestError(TesterError::RunOutOfInputs)),
                    };

                    if input > 999 {
                        input = 999;
                    }

                    // Set the register value to the input
                    computer.register = input;
                }
                2 => {
                    // OUTPUT with tester
                    let mut expected = match tester.outputs.pop_front() {
                        Some(expected) => expected,
                        None => return Err(ComputerError::TestError(TesterError::RunOutOfOutputs)),
                    };

                    if expected > 999 {
                        expected = 999;
                    }

                    if computer.register != expected {
                        return Err(ComputerError::TestError(TesterError::DifferentOutput(
                            expected,
                            computer.register,
                        )));
                    }
                }
                11 if computer.extended_mode => {
                    // ASCII INPUT (extended mode) with tester
                    let input = match tester.ainputs.pop_front() {
                        Some(input) => input,
                        None => return Err(ComputerError::TestError(TesterError::RunOutOfAInputs)),
                    };

                    computer.register = (input as u8) as u16;
                }
                12 if computer.extended_mode => {
                    // ASCII OUTPUT (extended mode) with tester
                    let expected = match tester.aoutputs.pop_front() {
                        Some(expected) => expected,
                        None => {
                            return Err(ComputerError::TestError(TesterError::RunOutOfAOutputs))
                        }
                    };

                    if computer.register != (expected as u8) as u16 {
                        return Err(ComputerError::TestError(TesterError::DifferentAOutput(
                            expected,
                            (computer.register as u8) as char,
                        )));
                    }
                }
                _ => {}
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
            11 if computer.extended_mode => {
                // ASCII INPUT (extended mode)
                let mut input = String::new();
                if match stdin().read_line(&mut input) {
                    Ok(_) => {
                        // Read a line and get the character code of the first character
                        if let Some(character) = input.chars().next() {
                            computer.register = (character as u8) as u16;
                            false
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
            12 if computer.extended_mode => {
                // ASCII OUTPUT (extended mode)
                print!("{}", (computer.register as u8) as char)
            }

            _ => {}
        },
        _ => {
            // If the instruction is invalid, decrement the counter to stay on the instruction, set the state and error
            computer.counter -= 1;
            return Err(ComputerError::InvalidInstruction(
                computer.counter,
                instruction,
            ));
        }
    }

    // If a tester is attatched:
    if let Some(tester) = &mut computer.tester {
        // Increment the number of fetch-execute cycles and check if it is more than the maximum
        tester.fe_cycles += 1;
        if tester.fe_cycles >= tester.max_fe_cycles {
            return Err(ComputerError::TestError(TesterError::RunOutOfCycles));
        }

        // If the computer has halted, make sure all the inputs and outputs have been consumed
        if matches!(computer.state, ComputerState::Halted) {
            if !tester.inputs.is_empty() {
                return Err(ComputerError::TestError(TesterError::ExpectedMoreInputs));
            }
            if !tester.ainputs.is_empty() {
                return Err(ComputerError::TestError(TesterError::ExpectedMoreAInputs));
            }
            if !tester.outputs.is_empty() {
                return Err(ComputerError::TestError(TesterError::ExpectedMoreOutputs));
            }
            if !tester.aoutputs.is_empty() {
                return Err(ComputerError::TestError(TesterError::ExpectedMoreAOutputs));
            }
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

        let mut computer = Computer::new(memory);
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

    #[test]
    fn test_no_io() {
        let (name, tester) = Tester::from_csv_line("t1;;;100").unwrap();

        assert_eq!(name, "t1", "Test does not have correct name!");
        assert_eq!(tester.inputs.len(), 0, "Tester does not have empty inputs!");
        assert_eq!(
            tester.ainputs.len(),
            0,
            "Tester does not have empty ASCII inputs!"
        );
        assert_eq!(
            tester.outputs.len(),
            0,
            "Tester does not have empty outputs!"
        );
        assert_eq!(
            tester.aoutputs.len(),
            0,
            "Tester does not have empty ASCII outputs!"
        );
        assert_eq!(
            tester.max_fe_cycles, 100,
            "Tester does not have correct max cycles!"
        );
    }

    #[test]
    fn test_fibonacci() {
        let assembly = include_str!("fib.txt");
        let memory = crate::assembler::assemble_from_assembly(assembly).unwrap();
        let mut computer = Computer::new(memory);

        let (_, tester) = Tester::from_csv_line("t1;;1,2,3,5,8,13,21,34,55,89,144;1000").unwrap();
        computer.tester = Some(Box::new(tester));

        run(&mut computer).unwrap();
    }

    #[test]
    fn test_input() {
        let mut memory: [u16; 100] = [0; 100];
        [
            901, 390, 901, 712, 211, 391, 592, 190, 392, 591, 603, 001, 592, 902,
        ]
        .iter()
        .enumerate()
        .for_each(|(i, &n)| memory[i] = n);
        let mut computer = Computer::new(memory);

        let (_, tester) = Tester::from_csv_line("t2;5,6;30;1000").unwrap();
        computer.tester = Some(Box::new(tester));

        run(&mut computer).unwrap();
    }

    #[test]
    fn test_extended() {
        let mut memory: [u16; 100] = [0; 100];
        [010, 911, 390, 901, 712, 211, 391, 590, 912, 591, 604, 001]
            .iter()
            .enumerate()
            .for_each(|(i, &n)| memory[i] = n);
        let mut computer = Computer::new(memory);

        let (_, tester) = Tester::from_csv_line("t3;g,4;g,g,g,g;1000").unwrap();
        computer.tester = Some(Box::new(tester));

        run(&mut computer).unwrap();
    }

    #[test]
    fn test_timeout() {
        let mut memory: [u16; 100] = [0; 100];
        [504, 706, 205, 601, 010, 001]
            .iter()
            .enumerate()
            .for_each(|(i, &n)| memory[i] = n);
        let mut computer = Computer::new(memory);

        let (_, tester) = Tester::from_csv_line("t4;;;10").unwrap();
        computer.tester = Some(Box::new(tester));

        assert!(
            matches!(
                run(&mut computer),
                Err(ComputerError::TestError(TesterError::RunOutOfCycles))
            ),
            "Tester did not stop the execution!"
        );
    }

    #[test]
    fn test_expected_inputs() {
        let mut computer = Computer::new([0; 100]);

        let (_, tester) = Tester::from_csv_line("t5;5;;100").unwrap();
        computer.tester = Some(Box::new(tester));

        assert!(
            matches!(
                run(&mut computer),
                Err(ComputerError::TestError(TesterError::ExpectedMoreInputs))
            ),
            "Tester did not expect more inputs!"
        );
    }

    #[test]
    fn test_expected_outputs() {
        let mut computer = Computer::new([0; 100]);

        let (_, tester) = Tester::from_csv_line("t5;;5;100").unwrap();
        computer.tester = Some(Box::new(tester));

        assert!(
            matches!(
                run(&mut computer),
                Err(ComputerError::TestError(TesterError::ExpectedMoreOutputs))
            ),
            "Tester did not expect more outputs!"
        );
    }
}
