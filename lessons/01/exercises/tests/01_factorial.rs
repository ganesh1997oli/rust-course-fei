//! Run this file with `cargo test --test 01_factorial`.

// TODO: Implement a simple factorial function.
pub fn factorial(n: u32) -> u32 {
    (1..=n).product()
}

pub fn factorial_1(n: u32) -> u32 {
    // Base case: factorial of 0 is 1
    if n == 0 {
        return 1;
    }

    // Recursive case: Calculate factorial using recursion
    n * factorial(n - 1)
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::factorial;

    #[test]
    fn factorial_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_1() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn factorial_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_5() {
        assert_eq!(factorial(5), 120);
    }
}
