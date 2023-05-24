use std::error::Error;
use std::fmt::Display;

pub const fn validate_identifier(name: &str) -> Result<(), InvalidIdentifier> {
    let mut index = 0;
    while index < name.len() {
        let byte = name.as_bytes()[index];
        if byte.is_ascii_alphanumeric() || byte == b'_' {
            index += 1;
        } else {
            return Err(InvalidIdentifier);
        }
    }
    Ok(())
}

pub fn pascal_case_to_snake_case(name: String) -> Result<String, InvalidIdentifier> {
    let mut bytes = name.into_bytes();
    let mut index = 0;
    let mut previous_was_upper = false;
    while let Some(ch) = bytes.get(index).copied() {
        if !ch.is_ascii_alphanumeric() && ch != b'_' {
            return Err(InvalidIdentifier);
        }
        let is_upper = ch.is_ascii_uppercase();
        let next_is_upper = bytes
            .get(index + 1)
            .map_or(false, |c| c.is_ascii_uppercase());
        if is_upper {
            if previous_was_upper && !next_is_upper {
                bytes.insert(index, b'_');
                index += 1;
            }
            bytes[index] = ch.to_ascii_lowercase();
            index += 1;
        } else {
            index += 1;
            if next_is_upper {
                bytes.insert(index, b'_');
                index += 1;
            }
        }
        previous_was_upper = is_upper;
    }
    Ok(String::from_utf8(bytes).expect("invalid unicode is rejected"))
}

#[derive(Debug)]
pub struct InvalidIdentifier;

impl Display for InvalidIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("invalid character found in identifier")
    }
}

impl Error for InvalidIdentifier {}

#[test]
fn pascal_conversion_tests() {
    assert_eq!(
        pascal_case_to_snake_case(String::from("Test")).unwrap(),
        "test"
    );
    assert_eq!(
        pascal_case_to_snake_case(String::from("aFFITest")).unwrap(),
        "a_ffi_test"
    );
    assert_eq!(
        pascal_case_to_snake_case(String::from("TestTest")).unwrap(),
        "test_test"
    );
}
