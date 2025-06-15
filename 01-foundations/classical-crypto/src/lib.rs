//! Classical Cryptography Implementations
//! 
//! This crate implements historical ciphers to understand basic cryptographic concepts.

pub mod caesar;
pub mod vigenere;
pub mod substitution;

pub use caesar::Caesar;
pub use vigenere::Vigenere;
