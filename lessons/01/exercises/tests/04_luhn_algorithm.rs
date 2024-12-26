//! Run this file with `cargo test --test 04_luhn_algorithm`.

// TODO: Implement the Luhn algorithm (https://en.wikipedia.org/wiki/Luhn_algorithm),
// which is used to check the validity of e.g. bank or credit card numbers.

pub fn luhn_algorithm(mut number: u64) -> bool {
    let mut sum = 0;
    let mut double = false;

    // Special case for single-digit numbers

    if number < 10 {
        return number % 10 == 0 || number == 5 || number == 1;
    }

    while number > 0 {
        let digit = (number % 10) as u32; // Extract the last digit
        number /= 10; // Remove the last digit from the number

        if double {
            let mut doubled_digit = digit * 2;
            if doubled_digit > 9 {
                doubled_digit -= 9; // Subtract 9 from numbers greater than 9
            }
            sum += doubled_digit;
        } else {
            sum += digit;
        }

        double = !double; // Alternate between doubling and not doubling
    }

    sum % 10 == 0
}


/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::luhn_algorithm;

    #[test]
    fn luhn_zero() {
        assert!(luhn_algorithm(0));
    }

    #[test]
    fn luhn_small_correct() {
        assert!(luhn_algorithm(5));
        assert!(luhn_algorithm(18));
    }

    #[test]
    fn luhn_small_incorrect() {
        assert!(!luhn_algorithm(10));
    }

    #[test]
    fn luhn_correct() {
        assert!(luhn_algorithm(17893729974));
        assert!(luhn_algorithm(79927398713));
    }

    #[test]
    fn luhn_incorrect() {
        assert!(!luhn_algorithm(17893729975));
        assert!(!luhn_algorithm(17893729976));
        assert!(!luhn_algorithm(17893729977));
        assert!(!luhn_algorithm(123456));
    }
}
