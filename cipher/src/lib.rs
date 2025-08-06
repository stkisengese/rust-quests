#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut expected_cipher = String::new();
    for c in original.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let mirrored = (25 - (c as u8 - base)) + base;
            expected_cipher.push(mirrored as char);
        } else {
            expected_cipher.push(c);
        }
    }
    if expected_cipher == ciphered {
        Ok(())
    } else {
        Err(CipherError { expected: expected_cipher })
    }
}