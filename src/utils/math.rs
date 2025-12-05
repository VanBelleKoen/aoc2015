#![allow(dead_code)]

/// Calculate the greatest common divisor using Euclidean algorithm
pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
}

/// Calculate the least common multiple
pub fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

/// Calculate LCM of a vector of numbers
pub fn lcm_vec(numbers: &[i64]) -> i64 {
    numbers.iter().copied().reduce(lcm).unwrap_or(1)
}

/// Calculate GCD of a vector of numbers
pub fn gcd_vec(numbers: &[i64]) -> i64 {
    numbers.iter().copied().reduce(gcd).unwrap_or(0)
}

/// Calculate modular exponentiation (base^exp % modulus)
pub fn mod_pow(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    result
}

/// Check if a number is prime
pub fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

/// Get all divisors of a number
pub fn divisors(n: i64) -> Vec<i64> {
    let mut divs = Vec::new();
    for i in 1..=((n as f64).sqrt() as i64) {
        if n % i == 0 {
            divs.push(i);
            if i != n / i {
                divs.push(n / i);
            }
        }
    }
    divs.sort_unstable();
    divs
}
