use crate::road_map::*;

// TODO: determinar si vale la pena o no meter mejores nombres

// Exercise 1 -----------------------------------------------------------------
use self::arrays::contains_duplicate;

#[test]
fn exercise_1_test_1() {
  let nums: Vec<i32> = vec![1,2,3,1];
  assert!(contains_duplicate(nums));
}

#[test]
fn exercise_1_test_2() {
  let nums: Vec<i32> = vec![1,2,3,4];
  assert!(!contains_duplicate(nums));
}

#[test]
fn exercise_1_test_3() {
  let nums: Vec<i32> = vec![1,1,1,3,3,4,3,2,4,2];
  assert!(contains_duplicate(nums));
}

// Exercise 2 -----------------------------------------------------------------
use self::arrays::is_anagram;

#[test]
fn exercise_2_test_1() {
	let s = "anagram";
	let t = "nagaram";
	assert!(is_anagram(s, t));
}

#[test]
fn exercise_2_test_2() {
	let s = "rat";
	let t = "car";
	assert!(!is_anagram(s, t));
}

// Exercise 3 -----------------------------------------------------------------
use self::arrays::two_sum;

#[test]
fn exercise_3_test_1() {
	let nums = vec![2,7,11,15]; 
	let target = 9;
	let result = two_sum(&nums, target);
	
	let correct_results: [Vec<i32>; 2] = [vec![0, 1], vec![1, 0]];
    assert!(correct_results.contains(&result));
}

// Exercise 4 -----------------------------------------------------------------
use self::arrays::group_anagrams;

#[test]
fn exercise_4_test_1() {
	// TODO: ver si hay una forma más elegante de crear estos arrays
	let strs: Vec<String> = vec!["eat","tea","tan","ate","nat","bat"]
		.into_iter()
		.map(|s| s.to_string())
		.collect();
	
	let expected: Vec<Vec<String>> = vec![
		vec!["bat"],
		vec!["nat","tan"],
		vec!["ate","eat","tea"]
	].into_iter().map(|v| v.into_iter().map(|s| s.to_string()).collect()).collect();
	
	let mut result: Vec<Vec<String>> = group_anagrams(&strs);
	// Ordeno el resultado para comparar en assert
	for vec in &mut result {
		vec.sort();
	}
	// Ordeno vectores por longitud
	result.sort_by(|a, b| a.len().cmp(&b.len()));

	assert_eq!(result, expected);
}

// Exercise 5 -----------------------------------------------------------------
use self::arrays::top_k_frequent;

#[test]
fn exercise_5_test_1() {
	let nums = vec![1,1,1,2,2,3];
	let k = 2;
	assert_eq!(top_k_frequent(&nums, k), vec![1, 2]);
}

#[test]
fn exercise_5_test_2() {
	let nums = vec![4,4,4,1,1,2,2,2,2,3];
	let k = 3;
	assert_eq!(top_k_frequent(&nums, k), vec![2, 4, 1]);
}

// Exercise 6 -----------------------------------------------------------------
use self::arrays::product_except_self;

#[test]
fn exercise_6_test_1() {
	let nums = vec![1,2,3,4];
	let expected = vec![24,12,8,6];
	assert_eq!(product_except_self(&nums), expected);
}

#[test]
fn exercise_6_test_2() {
	let nums = vec![-1,1,0,-3,3];
	let expected = vec![0,0,9,0,0];
	assert_eq!(product_except_self(&nums), expected);
}

// Exercise 7 -----------------------------------------------------------------
use self::arrays::is_valid_sudoku;

#[test]
fn exercise_7_test_1() {
	let board: Vec<Vec<char>> = vec![
        vec!['5','3','.','.','7','.','.','.','.'],
        vec!['6','.','.','1','9','5','.','.','.'],
        vec!['.','9','8','.','.','.','.','6','.'],
        vec!['8','.','.','.','6','.','.','.','3'],
        vec!['4','.','.','8','.','3','.','.','1'],
        vec!['7','.','.','.','2','.','.','.','6'],
        vec!['.','6','.','.','.','.','2','8','.'],
        vec!['.','.','.','4','1','9','.','.','5'],
        vec!['.','.','.','.','8','.','.','7','9']
    ];
    assert_eq!(is_valid_sudoku(&board), true);
}

// Same as Example 1, except with the 5 in the top left corner being modified to 8. 
// Since there are two 8's in the top left 3x3 sub-box, it is invalid.
#[test]
fn exercise_7_test_2() {
	let board: Vec<Vec<char>> = vec![
        vec!['8','3','.','.','7','.','.','.','.'],
        vec!['6','.','.','1','9','5','.','.','.'],
        vec!['.','9','8','.','.','.','.','6','.'],
        vec!['8','.','.','.','6','.','.','.','3'],
        vec!['4','.','.','8','.','3','.','.','1'],
        vec!['7','.','.','.','2','.','.','.','6'],
        vec!['.','6','.','.','.','.','2','8','.'],
        vec!['.','.','.','4','1','9','.','.','5'],
        vec!['.','.','.','.','8','.','.','7','9']
    ];

	assert_eq!(is_valid_sudoku(&board), false);
}