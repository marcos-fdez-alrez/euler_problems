use euler_problems;

#[test]
fn test_problem1() {
    assert_eq!(euler_problems::problem1(1000), 233168);
}

#[test]
fn test_problem2() {
    assert_eq!(euler_problems::problem2(), 4613732);
}

#[test]
fn test_problem3() {
    assert_eq!(euler_problems::problem3(600851475143), 6857);
}

#[test]
fn test_problem4() {
    println!("valor: {}",euler_problems::problem4());
    assert_eq!(euler_problems::problem4(), 906609);
}

#[test]
fn test_problem5() {
    assert_eq!(euler_problems::problem5(), 232792560);
}