// EASY ------------------------------------------------------------------------
// 1. Valid Parentheses	--------------------------------------------------------
// Given a string s containing just the characters '(', ')', '{', '}', 
// '[' and ']', determine if the input string is valid.

// For an input string to be valid:

// * Open brackets must be closed by the same type of brackets.
// * Open brackets must be closed in the correct order.
// * Every close bracket must have a corresponding open bracket of the same type.
pub fn is_valid(s: String) -> bool {
    // Los open brackets se tienen que cerrar en el "orden correcto". Esto 
    // significa que tengo que cerrar primero el último que abrí.
    // Esa idea apunta a usar un stack: cada vez que encuentro un opening lo 
    // pusheo al stack. Cada vez que encuentro un closing, hago un pop del stack.
    // Si no es un opening del mismo estilo => no es valido.
    let mut stack: Vec<char> = Vec::new();
    
    for c in s.chars() {
        if c == '(' || c == '{' || c == '[' {
            stack.push(c);

        } else {
            if let Some(opening) = stack.pop() {
                match (opening, c) {
                    ('(', ')') | ('{', '}') | ('[', ']') => continue,
                    _ => return false,
                }
            } else {
                return false;
            }
        }
    }

    stack.is_empty()
}

// MEDIUM ----------------------------------------------------------------------
// 2. Min Stack	----------------------------------------------------------------
// Design a stack that supports push, pop, top, and retrieving the minimum 
// element in constant time.

// Implement the MinStack class:
// * MinStack() initializes the stack object.
// * void push(int val) pushes the element val onto the stack.
// * void pop() removes the element on the top of the stack.
// * int top() gets the top element of the stack.
// * int getMin() retrieves the minimum element in the stack.
// You must implement a solution with O(1) time complexity for each function.
pub struct MinStack<T: PartialOrd + Clone> {
    stack: Vec<T>,
    // Vas a mantener dos stacks en simultaneo, uno para las consultas típicas y
    // otro exclusivo para las consultas del mín.
    min_stack: Vec<T>
}

impl<T: PartialOrd + Clone> MinStack<T> {
    pub fn new() -> Self {
        MinStack {
            stack: vec![],
            min_stack: vec![]
        }
    }

    // En el push estás siempre actualizando el mínimo (si hace falta)
    pub fn push(&mut self, val: T) {
        self.stack.push(val.clone());
        // Checkeo si inserté el nuevo mínimo
        if self.min_stack.is_empty() || Some(&val) <= self.min_stack.last() {
            self.min_stack.push(val);
        }
    }

    // En el pop, gracias a que estás usando un stack para los mínimos, podés
    // estar seguro que si sacan el mínimo actual, seguís teniendo la información
    // del nuevo mínimo sin tener que recorrer el stack O(1)
    pub fn pop(&mut self) -> T {
        let val = self.stack.pop().unwrap();
        //Checkeo si acabo de sacar el mínimo
        if Some(&val) == self.min_stack.last() {
            self.min_stack.pop();
        }

        val
    }

    pub fn top(&self) -> T {
        self.stack.last().cloned().unwrap()
    }

    // get_min simplemente te dice cuál es el mínimo del stack actualmente. No 
    // hace un pop de ese elemento.
    pub fn get_min(&self) -> T {
        self.min_stack.last().cloned().unwrap()
    }
}

// 3. Evaluate Reverse Polish Notation -----------------------------------------
// You are given an array of strings tokens that represents an arithmetic expression 
// in a Reverse Polish Notation.
// Evaluate the expression. Return an integer that represents the value of the expression.

// Note that:
// * The valid operators are '+', '-', '*', and '/'.
// * Each operand may be an integer or another expression.
// * The division between two integers always truncates toward zero.
// * There will not be any division by zero.
// * The input represents a valid arithmetic expression in a reverse polish notation.
// * The answer and all the intermediate calculations can be represented in a 32-bit integer.
pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    // RPN es una forma de representar expresiones matemáticas en la que primero
    // se muestran los argumentos (números) y luego se muestran los operadores.

    // Está pensado para ser calculado usando un stack. Ej: 3 10 5 + * retorna 
    // lo siguiente: (5 + 10) * 3.
    // => pusheas los números y cuando aparecen los operadores popeas dos números
    // y procesas. Al resultado lo volvés a pushear para procesarlo con el 
    // siguiente operador o finalmente retornarlo como respuesta.
    // unwrap() va a paniquear si no es un número/símbolo o si no hay 2 elementos
    // en el stack para procesar.
    let mut stack: Vec<i32> = vec![];
    
    for token in tokens {
        match token.parse::<i32>() {
            Ok(number) => stack.push(number),
            _ => { // Not a number
                let right = stack.pop().unwrap();
                let left  = stack.pop().unwrap();

                let answer = solve_operation(left, right, token);

                stack.push(answer);
            }
        }
    }

    stack[0]
}

fn solve_operation(a: i32, b: i32, op: String) -> i32 {
    match op.as_str() {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        _ => unreachable!()
    }
}

// 4. Generate Parentheses -----------------------------------------------------
// Given n pairs of parentheses, write a function to generate all combinations 
// of well-formed parentheses.
// Por ej: input n = 2 => rta = ()(), (())
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    // La idea es pensar la construcción de las combinaciones a partir de 
    // inserciones en un árbol binario.
    // * El nodo raíz es "(", ya que no podés empezar con un ")".
    // * Ahora tenés dos opciones (pensá en el árbol binario): agregar un "(" 
    //   o agregar un ")".
    // Cada opción válida va a ser un hijo del nodo => el nodo "(" tiene dos hijos:
    // "((" y "()". Así se repite la idea con los siguientes hijos.
    // Ahora, tenés a n que te limita la cantidad de combinaciones. Para esto
    // tenés un contador que te indica cuantos openings pusiste y cuantos closers
    // pusiste. Openings tiene que ser siempre >= a closers y tenés que seguir
    // hasta tener n openings y n closers.
    // Los nodos hoja del árbol van a ser todas las posibles combinaciones.

    // SOLUCIÓN:
    // No hace falta que crees el árbol. Es conceptual. Podés pensar que el árbol
    // ya está creado y vas a recorrer hasta cada hoja para encontrar las 
    // las combinaciones.
    // Para eso vas a hacer una especie de recorrido conceptual del árbol en el 
    // que para cada recorrido, vas cargando los "(" o ")" en un stack y cuando
    // llegas a que Openings = Closers = n cargas esa combinación en la respuesta.
    let mut result: Vec<String> = vec![];

    // Función que crea las combinaciones recursivamente y tiene acceso a las
    // variables n, stack y result.
    // Uso un closure ya que puede capturar su entorno => puede acceder a las variables
    fn depth_traversal(result: &mut Vec<String>, stack: String, openings: i32, closers: i32) {
        if openings == 0 && closers == 0 { // Caso Base: nodo hoja
            result.push(stack);
            return;
        }

        if openings == closers { // Solo puedo agregar un '('
            depth_traversal(result, stack.clone() + "(", openings - 1, closers);
        } else {
            // Con la condición de arriba ya no puede pasar que agregues un ")"
            // cuando no hay un "(" que le corresponda.
            if openings > 0 { // Podés agregar un (
                depth_traversal(result, stack.clone() + "(", openings - 1, closers);
            }  
            
            if closers > 0 { // Podés agregar un )
                depth_traversal(result, stack.clone() + ")", openings, closers - 1);
            }
        }
    }

    depth_traversal(&mut result, String::from(""), n, n);

    result
}

// 5. Daily Temperatures -------------------------------------------------------

// 6. Car Fleet	----------------------------------------------------------------

// HARD ------------------------------------------------------------------------
// 7. Largest Rectangle In Histogram -------------------------------------------
