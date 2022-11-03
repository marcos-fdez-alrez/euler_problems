//use num::integer::Roots;

extern crate primes;

use primes::is_prime;

// https://projecteuler.net/problem=1
pub fn problem1(limit: i32) -> i32 {
    let mut sum = 0;

    for i in 1..limit {
        if (i % 3 == 0) || (i % 5 == 0) {
            sum += i;
        }
    }
    return sum;
}

// https://projecteuler.net/problem=2
pub fn problem2() -> i64 {
    let mut val1 = 1;
    let mut val2 = 2;
    let mut sum = 0;

    while val2 <= 4_000_000 {
        if val2 % 2 == 0 {
            sum += val2
        }

        let tmp = val1;
        val1 = val2;
        val2 = val1 + tmp;
    }
    return sum;
}

// https://projecteuler.net/problem=3
pub fn problem3(input: i64) -> i64 {
    let limit = (input as f64).sqrt();
    let mut max = 1;
    
    for i in 2..=limit.round() as i64 {
        if input % i == 0 && is_prime(i as u64) { // alternative to .try_into().unwrap()
            max = i;
        }
    }
    
    println!("{}",max);
    
    return max;
}

// https://projecteuler.net/problem=4


// https://projecteuler.net/problem=5


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        assert_eq!(problem1(1000), 233168);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(problem2(), 4613732);
    }

    #[test]
    fn test_problem3() {
        assert_eq!(problem3(600851475143), 6857);
    }

    //#[test]
    //fn test_problem4() {
    //    assert_eq!(add(1, 2), 906609);
    //}

    //#[test]
    //fn test_problem5() {
    //    assert_eq!(add(1, 2), 232792560);
    //}
}