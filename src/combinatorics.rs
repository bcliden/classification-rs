pub fn factorial(n: i64) -> i64 {
    (1..=n).product()
}

pub fn naive_nCr(n: i64, r: i64) -> i64 {
    // this easily overflows
    factorial(n) / (factorial(r) * factorial(n - r))
}

/// more efficient nCr function w/ less risk of overflow
/// credit to: https://stackoverflow.com/a/11809815
pub fn nCr(n: i64, mut r: i64) -> i64 {
    if r > n - r {
        // because nCr(n, r) == nCr(n, n-r)
        r = n - r;
    }
    let mut ans = 1;
    for i in 1..=r {
        ans *= n - r + i;
        ans /= i;
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(7), 5040);
    }

    #[test]
    fn test_nCr() {
        assert_eq!(naive_nCr(6, 2), 15);
        // assert_eq!(naive_nCr(25, 3), 2300); // will overflow
        // assert_eq!(naive_nCr(18, 4), 3060); // will overflow
    }

    #[test]
    fn test_efficient_nCr() {
        assert_eq!(nCr(6, 2), 15);
        assert_eq!(nCr(25, 3), 2300);
        assert_eq!(nCr(18, 4), 3060);
    }
}