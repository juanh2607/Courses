use std::collections::HashMap;
use std::collections::HashSet;

// 1. Contains Duplicate -------------------------------------------------------
// Given an integer array nums, return true if any value appears at least twice 
// in the array, and return false if every element is distinct.

// use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hset: HashSet<i32> = HashSet::new();

    for n in nums {
        if hset.insert(n) == false {
            return true;
        }
    }

    false
}

// 2. Valid Anagram ------------------------------------------------------------
// Given two strings s and t, return true if t is an anagram of s, and false 
// otherwise.
pub fn is_anagram(s: &str, t: &str) -> bool {
    // Primero checkeo que tengan la misma longitud
    if s.len() != t.len() {
        return false;
    }
    // Segundo checkeo que tengan la misma cantidad por cada letra => uso hashmap
    let mut s_hash = HashMap::new();
    let mut t_hash = HashMap::new();

    for x in s.chars() {
        *s_hash.entry(x).or_insert(0) += 1;
    }

    for x in t.chars() {
        *t_hash.entry(x).or_insert(0) += 1;
    }

    s_hash == t_hash
}