//use num::integer::Roots;

extern crate primes;
extern crate palindrome;

use primes::is_prime;
use palindrome::is_palindrome;

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
pub fn problem4() -> u64 {
    let mut num;
    let mut palindrome = 0;
    for i in 100..=999 {
        for j in 100..=999 {
            num = i * j;
            if is_palindrome(num) && num > palindrome {
                palindrome = num;
            }
        }
    }
    return palindrome;
}


/*pub fn problem4() -> u64 {
    let mut number1 = 999;
    let mut number2 = 999;
    let mut found = false;
    let mut num = 0;
    let mut palindrome = 0;

    while !found && number1>=100{
        num = number1 * number2;
        if is_palindrome(num) {
            found = true;
            if num > palindrome {
                palindrome = num;
            }
        } 
        if number2 >= 100 {
            number2 -=1;
        } else {
            number1 -=1;
            number2 = 999;
        }
    }

    return num;
}*/


// https://projecteuler.net/problem=5

pub fn problem5() -> u64 {
    let mut number: u64 = 20;
    let mut found = false;
    while !found {
        found = true;
        number += 1;
        
        for i in 1..=20 {
            if number % i != 0 {
                found = false
            }      
        }
    }
    println!("{}", number);
    return number; 
}
