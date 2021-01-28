use crate::errors::*;

const CHARS: [char; 29] = ['0','1', '2', '3', '4', '5', '6', '7', '8', '9', 'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'z'];

/// Computes the checksum char of a string with the NCDA algorithm.
/// 
/// ```
/// # fn main() {
/// let checksum = ncda::checksum("cb32752361");
/// 
/// assert_eq!(checksum, Ok('d'));
/// # }
/// ```
pub fn checksum<R: AsRef<str>>(input: R) -> Result<char, NcdaError> {
    let data = input.as_ref();

    if data.len() > 29 {
        return Err(NcdaError::InputDataTooLong(InputDataTooLong(data.len())))
    }

    if data.is_empty() {
        return Err(NcdaError::InputDataTooShort(InputDataTooShort(0)))
    }
        
    let sum = data.chars().enumerate().try_fold(0, |acc, (index, char)| {
        match &CHARS[..].binary_search(&char) {
            Ok(c) => Ok(c * (index + 1) + acc),
            Err(_) => Err(NcdaError::InvalidInputChar(InvalidInputChar {
                found: char,
                in_input_data: data.to_string(),
                expected_one_of: CHARS.to_vec(),
            }))
        }
    })?;
    
    Ok(CHARS[sum % 29])
}

/// Checks the checksum char of a string with the NCDA algorithm.
/// 
/// ```
/// # fn main() {
/// let checksum = ncda::check("cb32752361d");
/// 
/// assert_eq!(checksum, Ok(()));
/// # }
/// ```
pub fn check<R: AsRef<str>>(input: R) -> Result<(), NcdaError> {
    let data = input.as_ref();

    let data_len = data.len();

    if data_len >= 2 {
        let (d, last_char) = &data.split_at(data_len - 1);

        let checksum = checksum(d)?;
        let last_char = last_char.chars().next().unwrap();
        
        if checksum == last_char {
            return Ok(())
        } else {
            return Err(NcdaError::InvalidChecksum(InvalidChecksum {
                computed_checksum: checksum,
                for_input_data: data.to_string(),
                expected: last_char,
            }))
        }
    } else {
        return Err(NcdaError::InputDataTooShort(InputDataTooShort(data_len)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_checksum() {
        assert_eq!(checksum("cb32752361"), Ok('d'));

        assert_eq!(checksum(""), Err(NcdaError::InputDataTooShort(InputDataTooShort(0))));

        assert_eq!(checksum("cb32752361cb32752361cb327523611"), Err(NcdaError::InputDataTooLong(InputDataTooLong(31))));
    }
    
    #[bench]
    fn bench_checksum(b: &mut Bencher) {
        b.iter(|| checksum("cb32752361"));
    }

    #[test]
    fn test_check() {
        assert_eq!(check("cb32752361d").is_ok(), true);

        assert_eq!(check("cb32752361x"), Err(NcdaError::InvalidChecksum(InvalidChecksum{
            computed_checksum: 'd',
            expected: 'x',
            for_input_data: "cb32752361x".into(),
        })));

        assert_eq!(check("c"), Err(NcdaError::InputDataTooShort(InputDataTooShort(1))));

        assert_eq!(check("AAA"), Err(NcdaError::InvalidInputChar(InvalidInputChar{
            found: 'A',
            expected_one_of: vec!['0','1', '2', '3', '4', '5', '6', '7', '8', '9', 'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'z'],
            in_input_data: "AA".into()
        })));

        assert_eq!(check("cb32752361cb32752361cb327523611"), Err(NcdaError::InputDataTooLong(InputDataTooLong(30))));
    }

    #[bench]
    fn bench_check(b: &mut Bencher) {
        b.iter(|| check("cb32752361d"));
    }
}