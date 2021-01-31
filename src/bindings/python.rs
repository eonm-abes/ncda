use crate::errors::NcdaError;
use crate::ncda;

use pyo3::prelude::*;

impl std::convert::From<NcdaError> for pyo3::PyErr {
    fn from(err: NcdaError) -> Self {
        match err {
            NcdaError::InvalidChecksum(_) => {
                pyo3::exceptions::PyAssertionError::new_err(err.to_string())
            }
            _ => pyo3::exceptions::PyValueError::new_err(err.to_string()),
        }
    }
}

#[pyfunction]
pub fn checksum(input: &str) -> Result<char, NcdaError> {
    Ok(ncda::checksum(input)?)
}

#[pyfunction]
pub fn check(input: &str) -> PyResult<bool> {
    let x = match ncda::check(input) {
        Ok(_) => Ok(true),
        Err(e) => Err(e),
    };

    Ok(x?)
}
