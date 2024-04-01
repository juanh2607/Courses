use crate::road_map::*;

// Como leer esto:
// ($($x:expr),*): esto sería el parámetro de la macro. Le estás diciendo 
// que esperas 0 o más expresiones (el *) separadas por coma. Las expresiones
// se van a referenciar como x dentro del cuerpo de la macro.
// $ designa una variable de macro (ej: $x). También se usa para capturar 
// repeticiones con $(...),*.
// expr indica que x se corresponde con una expresión de Rust.
macro_rules! vec_of_strings {
	($($x:expr),*) => (vec![$($x.to_string()),*]);
}

// Exercise 1 ------------------------------------------------------------------
use self::stack::is_valid;

#[test]
pub fn exercise_1_test_1() {
    let s = String::from("()");
    assert!(is_valid(s));

    let s = String::from("()[]{}");
    assert!(is_valid(s));

    let s = String::from("(]");
    assert!(!is_valid(s));

    let s = String::from("(");
    assert!(!is_valid(s));
}
// Exercise 2 ------------------------------------------------------------------
use self::stack::MinStack;

#[test]
pub fn exercise_2_test_1() {
    let mut stack: MinStack<i32> = MinStack::new();
    let mut answer: Vec<i32> = vec![];
    
    stack.push(-2);
    stack.push(0);
    stack.push(-3);

    answer.push(stack.get_min());
    stack.pop();
    answer.push(stack.top());
    answer.push(stack.get_min());

    assert_eq!(answer, vec![-3, 0, -2]);
}

// Exercise 3 ------------------------------------------------------------------
use self::stack::eval_rpn;

#[test]
pub fn exercise_3_test_1() {
    let tokens: Vec<String> = vec_of_strings!("2","1","+","3","*");
    assert_eq!(eval_rpn(tokens), 9);
}

#[test]
pub fn exercise_3_test_2() {
    let tokens: Vec<String> = vec_of_strings!("4","13","5","/","+");
    assert_eq!(eval_rpn(tokens), 6);
}

#[test]
pub fn exercise_3_test_3() {
    let tokens: Vec<String> = vec_of_strings!("10","6","9","3","+","-11","*","/","*","17","+","5","+");
    assert_eq!(eval_rpn(tokens), 22);
}

// Exercise 4 ------------------------------------------------------------------
use self::stack::generate_parenthesis;

#[test]
pub fn exercise_4_test_1() {
    let expected = vec_of_strings!("((()))","(()())","(())()","()(())","()()()");
    assert_eq!(generate_parenthesis(3), expected);
}

#[test]
pub fn exercise_4_test_2() {
    let expected = vec_of_strings!("()");
    assert_eq!(generate_parenthesis(1), expected);
}

// Exercise 1 ------------------------------------------------------------------
// Exercise 1 ------------------------------------------------------------------
// Exercise 1 ------------------------------------------------------------------
