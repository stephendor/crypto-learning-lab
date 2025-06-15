//! Caesar Cipher Implementation
//! 
//! A simple substitution cipher that shifts each letter by a fixed amount.

/// Caesar cipher implementation
pub struct Caesar {
    shift: u8,
}

impl Caesar {
    pub fn new(shift: u8) -> Self {
        Self { shift: shift % 26 }
    }

    pub fn encrypt(&self, plaintext: &str) -> String {
        plaintext
            .chars()
            .map(|c| self.shift_char(c, self.shift))
            .collect()
    }

    pub fn decrypt(&self, ciphertext: &str) -> String {
        ciphertext  // Fixed: was "plaintext"
            .chars()
            .map(|c| self.shift_char(c, 26 - self.shift))
            .collect()
    }

    fn shift_char(&self, c: char, shift: u8) -> char {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let shifted = (c as u8 - base + shift) % 26;
            (base + shifted) as char
        } else {
            c
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caesar_basic() {
        let cipher = Caesar::new(3);
        let plaintext = "HELLO WORLD";
        let ciphertext = cipher.encrypt(plaintext);
        assert_eq!(cipher.decrypt(&ciphertext), plaintext);
    }
}