use crate::road_map::*;

// TODO: determinar si vale la pena o no meter mejores nombres

// Excercise 1 -----------------------------------------------------------------
use self::arrays::contains_duplicate;

#[test]
fn excercise_1_test_1() {
  let nums: Vec<i32> = vec![1,2,3,1];
  assert!(contains_duplicate(nums));
}

#[test]
fn excercise_1_test_2() {
  let nums: Vec<i32> = vec![1,2,3,4];
  assert!(!contains_duplicate(nums));
}

#[test]
fn excercise_1_test_3() {
  let nums: Vec<i32> = vec![1,1,1,3,3,4,3,2,4,2];
  assert!(contains_duplicate(nums));
}

// Excercise 2 -----------------------------------------------------------------
use self::arrays::is_anagram;
#[test]
fn excercise_2_test_1() {
	let s = "anagram";
	let t = "nagaram";
	assert!(is_anagram(s, t));
}

#[test]
fn excercise_2_test_2() {
	let s = "rat";
	let t = "car";
	assert!(!is_anagram(s, t));
}