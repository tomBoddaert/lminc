use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

/// Errors for the assembly process
#[derive(Debug)]
pub enum Error {
    TooManyLines,
    MultipleInstructions(usize, String),
    InvalidInstruction(usize, String),
    ExcessWords(usize, String),
    NoInstruction(usize),
    InvalidVariable(usize, String),
    InvalidAddress(usize, String),
    MultipleAssignment(usize, String),
    ExpectedAddress(usize),
    ExpectedNumber(usize),
    InvalidNumber(usize, String),
    UnexpectedAddress(usize, String),
    TooManyVariables(usize, String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;

        match self {
            TooManyLines => write!(f, "The input file has too many lines of instructions (>100)!")?,
            MultipleInstructions(i, inst) => write!(f, "Instruction line {i} has multiple instructions ('{inst}')!")?,
            InvalidInstruction(i, inst) => write!(f, "The instruction '{inst}' on instruction/number line {i} is invalid!")?,
            ExcessWords(i, word) => write!(f, "Instruction line {i} has too many words ('{word}')!")?,
            NoInstruction(i) => write!(f, "Instruction line {i} has no instruction!")?,
            InvalidVariable(i, var) => write!(f, "The address variable '{var}' on instruction line {i} is invalid!")?,
            InvalidAddress(i, num) => write!(f, "The address '{num}' on instruction line {i} is invalid!")?,
            MultipleAssignment(i, variable) => write!(f, "The variable address of '{variable}' has already been set (instruction line {i})!")?,
            ExpectedAddress(i) => write!(f, "An address variable was expected on instruction line {i}!")?,
            ExpectedNumber(i) => write!(f, "A number was expected on number line {i}!")?,
            InvalidNumber(i, num) => write!(f, "The number '{num}' on instruction line {i} is invalid!")?,
            UnexpectedAddress(i, variable) => write!(f, "The address variable '{variable}' on instruction line {i} was not expected!")?,
            TooManyVariables(i, variable) => write!(f, "The input file contains too many variables (variable '{variable}', instruction line {i})!")?
        }

        Ok(())
    }
}

lazy_static! {
    pub static ref NUMBER_REGEX: Regex = Regex::new(r"(?:^|\n)[ \t\d]+").unwrap();
    pub static ref ASSEMBLY_REGEX: Regex = Regex::new(r"(?:^|\n)[ \ta-zA-Z\d_]+").unwrap();
    pub static ref DECIMAL_NUMBER: Regex = Regex::new(r"^\d+$").unwrap();
}

/// Takes a &str with instructions as 3 digit numbers, seperated by
/// line breaks. Ignores any character not in [ \t\d] and all
/// characters after it on that line.
pub fn assemble_from_numbers(text: &str) -> Result<[u16; 100], Error> {
    // Initialise the memory
    let mut memory: [u16; 100] = [0; 100];

    // Get the matched lines (up to an invalid character) in the text
    for (i, line) in NUMBER_REGEX
        .find_iter(text)
        .filter_map(|mat| {
            // Split the lines by spaces and remove if empty
            let split = mat
                .as_str()
                .split_whitespace()
                .fold("".to_owned(), |acc, text| acc + text);
            if split.is_empty() {
                None
            } else {
                Some(split)
            }
        })
        .enumerate()
    {
        // If there are too many instructions, error
        if i == 100 {
            return Err(Error::TooManyLines);
        }

        // Parse the instruction as a u16, or error
        let inst = match line.parse::<u16>() {
            Ok(num) => num,
            Err(_) => return Err(Error::InvalidInstruction(i + 1, line.clone())),
        };

        // If the instruction is too large, error
        if inst > 999 {
            return Err(Error::InvalidInstruction(i + 1, line));
        }

        memory[i] = inst;
    }

    Ok(memory)
}

// Defines the patterns for matching an instruction
macro_rules! instructionPatternWithAddress {
    () => {
        "ADD" | "SUB" | "STO" | "STA" | "LDA" | "BR" | "BRA" | "BRZ" | "BRP"
    };
}
macro_rules! instructionPatternWithoutAddress {
    () => {
        "IN" | "INP" | "OUT" | "HLT"
    };
}
macro_rules! extendedInstructions {
    () => {
        "INA" | "OTA"
    };
}
macro_rules! instructionPattern {
    () => {
        instructionPatternWithAddress!()
            | instructionPatternWithoutAddress!()
            | "DAT"
            | extendedInstructions!()
    };
}

/// Takes a &str with instructions in assembly, seperated by
/// line breaks. Ignores any character not in [ \tA-Za-z\d_]
/// and all characters after it on that line.
pub fn assemble_from_assembly(text: &str) -> Result<[u16; 100], Error> {
    // Initialise the memory
    let mut memory: [u16; 100] = [0; 100];

    // Create a map for the variables and their addresses
    let mut variables: HashMap<&str, usize> = HashMap::new();
    // Create a vector for the instructions and optional address variables
    let mut code: Vec<(&str, Option<&str>)> = Vec::new();

    // Get the matched lines (up to an invalid character) in the text
    let text_upper = text.to_uppercase();
    for (i, line) in ASSEMBLY_REGEX
        .find_iter(text_upper.as_str())
        .filter_map(|mat| {
            // Split the lines by spaces and remove if empty
            let split = mat.as_str().split_whitespace().collect::<Vec<&str>>();
            if split.is_empty() {
                None
            } else {
                Some(split)
            }
        })
        .enumerate()
    {
        // If there are too many instructions, error
        if i == 100 {
            return Err(Error::TooManyLines);
        }

        // Initialise the keys for the words as none
        let mut line_var: Option<&str> = None;
        let mut inst_opt: Option<&str> = None;
        let mut addr_var: Option<&str> = None;

        for word in line {
            // If the word is an instruction, and there already is one, error, if not, set the instruction
            if matches!(word, instructionPattern!()) {
                if inst_opt != None {
                    return Err(Error::MultipleInstructions(i + 1, word.to_owned()));
                }

                inst_opt = Some(word);
            } else {
                // If the instruction is not set, assume this is a line_var, error if there already is one
                if inst_opt == None {
                    if line_var != None {
                        return Err(Error::InvalidInstruction(i + 1, word.to_owned()));
                    }

                    line_var = Some(word);
                } else {
                    // otherwise, check if there is an addr_var, and error if there already is one
                    if addr_var != None {
                        return Err(Error::ExcessWords(i + 1, word.to_owned()));
                    }

                    addr_var = Some(word);
                }
            }
        }

        // If there is an instruction, unpack it, otherwise error
        let inst = match inst_opt {
            Some(inst) => inst,
            None => return Err(Error::NoInstruction(i + 1)),
        };

        // If there is a line_var, set it
        if let Some(var) = line_var {
            if DECIMAL_NUMBER.is_match(var) {
                return Err(Error::InvalidVariable(i + 1, var.to_owned()));
            }

            if variables.contains_key(&var) {
                return Err(Error::MultipleAssignment(i + 1, var.to_owned()));
            }
            variables.insert(var, i);
        }

        // Add the instruction and the optional address variable to the code
        code.push((inst, addr_var));
    }

    let mut variables_number = 0;

    for (i, &(inst, addr_var)) in code.iter().enumerate() {
        // Match the instruction with one with or without an address, or error
        memory[i] = match inst {
            instructionPatternWithAddress!() => {
                // Unpack the address from the address variable, or error
                let var = match addr_var {
                    Some(var) => var,
                    None => return Err(Error::ExpectedAddress(i + 1)),
                };

                // Set the opcode
                let opcode = match inst {
                    "ADD" => 100,
                    "SUB" => 200,
                    "STO" | "STA" => 300,
                    "LDA" => 500,
                    "BR" | "BRA" => 600,
                    "BRZ" => 700,
                    "BRP" => 800,
                    _ => 000,
                };

                // Get the address from the address variable, if it's not a number and creating one if it does not exist
                // Checking with regex before attempting to parse to catch numbers that woule be too large for a u16
                let addr: u16 = if DECIMAL_NUMBER.is_match(var) {
                    // Parse the text as a number, checking if it is out of bounds
                    match var.parse::<u16>() {
                        Ok(var_addr) if var_addr > 99 => {
                            return Err(Error::InvalidAddress(i + 1, var.to_owned()))
                        }
                        Ok(var_addr) => var_addr,
                        _ => return Err(Error::InvalidAddress(i + 1, var.to_owned())),
                    }
                } else if let Some(&var_addr) = variables.get(var) {
                    var_addr as u16
                } else {
                    // Get the next avaliable memory address, checking if it is out of bounds
                    let var_addr = code.len() + variables_number;
                    if var_addr > 99 {
                        return Err(Error::TooManyVariables(i + 1, var.to_owned()));
                    }

                    // Set the variable
                    variables.insert(var, var_addr);
                    variables_number += 1;

                    var_addr as u16
                };

                opcode + addr as u16
            }
            instructionPatternWithoutAddress!() => {
                // If an address variable was provided, error
                if let Some(var) = addr_var {
                    return Err(Error::UnexpectedAddress(i + 1, var.to_owned()));
                }

                // Get the instruction code
                match inst {
                    "IN" | "INP" => 901,
                    "OUT" => 902,
                    "HLT" => 000,
                    _ => 000,
                }
            }
            "DAT" => {
                let var = match addr_var {
                    Some(var) => var,
                    None => return Err(Error::ExpectedNumber(i + 1)),
                };

                // Check if var is a number, if there's a variable there, use the address there
                // Checking with regex before attempting to parse to catch numbers that woule be too large for a u16
                let number: u16 = if DECIMAL_NUMBER.is_match(var) {
                    // Parse the text as a number, checking if it is out of bounds
                    match var.parse::<u16>() {
                        Ok(var_addr) if var_addr > 999 => {
                            return Err(Error::InvalidNumber(i + 1, var.to_owned()))
                        }
                        Ok(var_addr) => var_addr,
                        Err(_) => return Err(Error::InvalidNumber(i + 1, var.to_owned())),
                    }
                } else if let Some(&var_addr) = variables.get(var) {
                    var_addr as u16
                } else {
                    // Get the next avaliable memory address, checking if it is out of bounds
                    let var_addr = code.len() + variables_number;
                    if var_addr > 99 {
                        return Err(Error::TooManyVariables(i + 1, var.to_owned()));
                    }

                    // Set the variable
                    variables.insert(var, var_addr);
                    variables_number += 1;

                    var_addr as u16
                };

                number
            }
            "INA" | "OTA" if memory[0] == 10 => {
                // Extended mode instructions
                // If an address variable was provided, error
                if let Some(var) = addr_var {
                    return Err(Error::UnexpectedAddress(i + 1, var.to_owned()));
                }

                match inst {
                    "INA" => 911,
                    "OTA" => 912,
                    _ => 000,
                }
            }
            _ => return Err(Error::InvalidInstruction(i + 1, inst.to_owned())),
        }
    }

    Ok(memory)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_assembly() {
        let assembly = "";
        let memory = assemble_from_assembly(assembly).unwrap();

        assert!(
            memory.iter().all(|num| *num == 0),
            "Could not assemble empty assembly!"
        );
    }

    #[test]
    fn full_assembly() {
        let assembly = "OUT\n".repeat(100);
        let memory = assemble_from_assembly(&assembly).unwrap();

        assert!(
            memory.iter().all(|num| *num == 902),
            "Could not assemble full assembly!"
        );
    }

    #[test]
    fn fibonacci_assembly() {
        let assembly = include_str!("fib.txt");
        let memory = assemble_from_assembly(assembly).unwrap();
        let expected_memory: [u16; 100] = [
            512, 113, 902, 315, 513, 312, 515, 313, 514, 215, 800, 000, 000, 001, 100, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
        ];

        assert!(
            memory
                .iter()
                .zip(expected_memory)
                .all(|(number, expected)| *number == expected),
            "Could not assemble Fibonacci!"
        );
    }

    #[test]
    fn absolute_address_assembly() {
        let assembly = include_str!("abs_addr.txt");
        let memory = assemble_from_assembly(assembly).unwrap();
        let expected_memory: [u16; 100] = [
            509, 398, 109, 399, 598, 902, 599, 902, 000, 001, 000, 000, 000, 000, 000, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
        ];

        assert!(
            memory
                .iter()
                .zip(expected_memory)
                .all(|(number, expected)| *number == expected),
            "Could not assemble assembly with absolute address!"
        )
    }

    #[test]
    fn empty_numbers() {
        let numbers = "";
        let memory = assemble_from_numbers(numbers).unwrap();

        assert!(
            memory.iter().all(|num| *num == 0),
            "Could not assemble empty numbers!"
        );
    }

    #[test]
    fn full_numbers() {
        let numbers = "902\n".repeat(100);
        let memory = assemble_from_numbers(&numbers).unwrap();

        assert!(
            memory.iter().all(|num| *num == 902),
            "Could not assemble full numbers!"
        );
    }

    #[test]
    fn fibonacci_numbers() {
        let numbers = include_str!("fib_num.txt");
        let memory = assemble_from_numbers(numbers).unwrap();

        let expected_memory: [u16; 100] = [
            605, 000, 001, 000, 100, 501, 102, 902, 303, 502, 301, 503, 302, 204, 816, 605, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
            000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000, 000,
        ];

        assert!(
            memory
                .iter()
                .zip(expected_memory)
                .all(|(number, expected)| *number == expected),
            "Could not assemble Fibonacci (numbers)!"
        );
    }
}
