use crate::road_map::*;

// Exercise 1 ------------------------------------------------------------------
use self::two_pointers::is_palindrome;

#[test]
pub fn exercise_1_test_1() {
    let s = String::from("A man, a plan, a canal: Panama");
    assert!(is_palindrome(s));
}

#[test]
pub fn exercise_1_test_2() {
    let s = String::from("race a car");
    assert!(!is_palindrome(s));
}

#[test]
pub fn exercise_1_test_3() {
    let s = String::from(" ");
    assert!(is_palindrome(s));
}

// Exercise 2 ------------------------------------------------------------------
use self::two_pointers::two_sum;

#[test]
pub fn exercise_2_test_1() {
    let numbers = vec![2,7,11,15];
    let target = 9;
    assert_eq!(two_sum(numbers, target), vec![1, 2]);

    let numbers = vec![2,3,4];
    let target = 6;
    assert_eq!(two_sum(numbers, target), vec![1, 3]);

    let numbers = vec![-1, 0];
    let target = -1;
    assert_eq!(two_sum(numbers, target), vec![1, 2]);
}

// Exercise 3 ------------------------------------------------------------------
use self::two_pointers::merge_sort;
use self::two_pointers::three_sum;

#[test]
pub fn exercise_3_test_1() {
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    merge_sort(&mut numbers);

    let expected = vec![-31, 0, 1, 2, 2, 4, 65, 83, 99, 782];
    assert_eq!(numbers, expected[..]);
}

#[test]
pub fn exercise_3_test_2() {
    let nums = vec![-1,0,1,2,-1,-4];
    let expected = vec![vec![-1,-1,2],vec![-1,0,1]];
    assert_eq!(three_sum(nums), expected);
}

#[test]
pub fn exercise_3_test_3() {
    let nums = vec![0,1,1];
    let expected: Vec<Vec<i32>> = vec![];
    assert_eq!(three_sum(nums), expected);
}

#[test]
pub fn exercise_3_test_4() {
    let nums = vec![0,0,0];
    let expected = vec![vec![0,0,0]];
    assert_eq!(three_sum(nums), expected);
}


// Exercise 4 ------------------------------------------------------------------
use self::two_pointers::max_area;

#[test]
pub fn exercise_4_test_1() {
    let nums = vec![1,8,6,2,5,4,8,3,7];
    assert_eq!(max_area(&nums), 49);
}

#[test]
pub fn exercise_4_test_2() {
    let nums = vec![1,1];
    assert_eq!(max_area(&nums), 1);
}

// Exercise 5 ------------------------------------------------------------------
use self::two_pointers::trapped_water;

#[test]
pub fn exercise_5_test_1() {
    let nums = vec![0,1,0,2,1,0,1,3,2,1,2,1];
    assert_eq!(trapped_water(&nums), 6);
}

#[test]
pub fn exercise_5_test_2() {
    let nums = vec![4,2,0,3,2,5];
    assert_eq!(trapped_water(&nums), 9);
}

#[test]
pub fn exercise_5_test_3() {
    let nums = vec![4,2,3];
    assert_eq!(trapped_water(&nums), 1);
}