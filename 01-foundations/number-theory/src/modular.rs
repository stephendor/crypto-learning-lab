//! Modular Arithmetic Operations

/// Compute (base^exp) mod modulus using fast exponentiation
pub fn mod_pow(base: u64, exp: u64, modulus: u64) -> u64 {
    if modulus == 1 { return 0; }
    
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;
    
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    
    result
}

/// Find modular multiplicative inverse using extended Euclidean algorithm
pub fn mod_inverse(a: i64, m: i64) -> Option<i64> {
    let (gcd, x, _) = extended_gcd(a, m);
    if gcd != 1 {
        None // Inverse doesn't exist
    } else {
        Some(((x % m) + m) % m)
    }
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (gcd, x1, y1) = extended_gcd(b % a, a);
        let x = y1 - (b / a) * x1;
        let y = x1;
        (gcd, x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow(2, 10, 1000), 24);
        assert_eq!(mod_pow(3, 4, 5), 1);
    }

    #[test]
    fn test_mod_inverse() {
        assert_eq!(mod_inverse(3, 11), Some(4));
        assert_eq!(mod_inverse(2, 4), None); // No inverse exists
    }
}
