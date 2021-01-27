use std::error::Error;
use std::fmt;

use serde::{Serialize, Deserialize};

/// An enum that holds all possible errors
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum NcdaError {
    InputDataTooShort(InputDataTooShort),
    InputDataTooLong(InputDataTooLong),
    InvalidInputChar(InvalidInputChar),
    InvalidChecksum(InvalidChecksum),
}

impl fmt::Display for NcdaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputDataTooShort(e) => write!(f, "{}", e),
            Self::InputDataTooLong(e) => write!(f, "{}", e),
            Self::InvalidInputChar(e) => write!(f, "{}", e),
            Self::InvalidChecksum(e) =>  write!(f, "{}", e),
        }
    }
}

impl Error for NcdaError {}

/// The computed checksum isn't valid
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InvalidChecksum {
    pub calculate_checksum : char,
    pub for_input_data: String,
    pub expected: char,
}

impl fmt::Display for InvalidChecksum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid checksum calculate checksum {} for {} expected {}", self.calculate_checksum, self.expected, self.for_input_data)
    }
}

impl Error for InvalidChecksum {}

/// Input data is too short. The checksum can't be computed.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InputDataTooShort(pub usize);

impl fmt::Display for InputDataTooShort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InvalidDataLength : {} expected", self.0)
    }
}

impl Error for InputDataTooLong {}

/// Input data is too long. The checksum can't be computed. Input data length has to be lower than 29 chars.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InputDataTooLong(pub usize);

impl fmt::Display for InputDataTooLong {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InvalidDataLength : {} expected at max 29", self.0)
    }
}

impl Error for InputDataTooShort {}

/// Input data contains invalid chars.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InvalidInputChar {
    pub found: char,
    pub in_input_data: String,
    pub expected_one_of: Vec<char>,
}

impl fmt::Display for InvalidInputChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid input char found {} in {}. Expected one of {:?}", self.found, self.in_input_data, self.expected_one_of)
    }
}

impl Error for InvalidInputChar {}

