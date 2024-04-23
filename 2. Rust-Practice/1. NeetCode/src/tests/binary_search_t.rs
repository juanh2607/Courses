use crate::road_map::*;

// Exercise 1 ------------------------------------------------------------------
use self::binary_search::search;

#[test]
pub fn exercise_1_test_1() {
    let nums = vec![-1,0,3,5,9,12]; 
    let target = 9;

    assert_eq!(search(nums, target), 4);
}

#[test]
pub fn exercise_1_test_2() {
    let nums = vec![-1,0,3,5,9,12]; 
    let target = 2;

    assert_eq!(search(nums, target), -1);
}

// Exercise 2 ------------------------------------------------------------------
use self::binary_search::search_matrix;

#[test]
pub fn exercise_2_test_1() {
    let nums = vec![
        vec![1,3,5,7],
        vec![10,11,16,20],
        vec![23,30,34,60]
    ]; 
    let target = 3;

    assert!(search_matrix(nums, target));
}

#[test]
pub fn exercise_2_test_2() {
    let nums = vec![
        vec![1,3,5,7],
        vec![10,11,16,20],
        vec![23,30,34,60]
    ]; 
    let target = 13;

    assert!(!search_matrix(nums, target));
}

#[test]
pub fn exercise_2_test_3() {
    let nums = vec![
        vec![1,3,5,7],
        vec![10,11,16,20],
        vec![23,30,34,60]
    ]; 
    let target = 11;

    assert!(search_matrix(nums, target));
}

// Exercise 3 ------------------------------------------------------------------
use self::binary_search::min_eating_speed;

#[test]
pub fn exercise_3_test_1() {
    let piles = vec![30,11,23,4,20];
    let h = 6;
    
    assert_eq!(min_eating_speed(piles, h), 23);
}

#[test]
pub fn exercise_3_test_2() {
    let piles = vec![3,6,7,11];
    let h = 8;
    
    assert_eq!(min_eating_speed(piles, h), 4);
}

#[test]
pub fn exercise_3_test_3() {
    let piles = vec![30,11,23,4,20];
    let h = 5;
    
    assert_eq!(min_eating_speed(piles, h), 30);
}

// Exercise 4 ------------------------------------------------------------------
use self::binary_search::find_min;

#[test]
pub fn exercise_4_test_1() {
    let nums = vec![4,5,6,7,0,1,2];
    assert_eq!(find_min(nums), 0);
}

#[test]
pub fn exercise_4_test_2() {
    let nums = vec![3,4,5,1,2];
    assert_eq!(find_min(nums), 1);
}


#[test]
pub fn exercise_4_test_3() {
    let nums = vec![11,13,15,17];
    assert_eq!(find_min(nums), 11);
}


// Exercise 5 ------------------------------------------------------------------
use self::binary_search::search_2;

#[test]
pub fn exercise_5_test_1() {
    let nums = vec![4,5,6,7,0,1,2];
    let target = 0;

    assert_eq!(search_2(nums, target), 4);
}

#[test]
pub fn exercise_5_test_2() {
    let nums = vec![4,5,6,7,0,1,2];
    let target = 3;

    assert_eq!(search_2(nums, target), -1);
}

#[test]
pub fn exercise_5_test_3() {
    let nums = vec![1];
    let target = 0;

    assert_eq!(search_2(nums, target), -1);
}
// Exercise 1 ------------------------------------------------------------------
// Exercise 1 ------------------------------------------------------------------
