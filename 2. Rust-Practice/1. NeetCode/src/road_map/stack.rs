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
// Given an array of integers temperatures represents the daily temperatures, 
// return an array answer such that answer[i] is the number of days you have to 
// wait after the ith day to get a warmer temperature. If there is no future day 
// for which this is possible, keep answer[i] == 0 instead.

pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    // Por fuerza bruta, esto es O(n^2). Pruebo usar un stack para intentar 
    // hacerlo en O(n)
    // La idea base es esta: vamos a ir pusheando los valores que leemos en un 
    // stack. Cuando lees un nuevo valor (t2), te fijás si tiene mayor temperatura 
    // el que se encuentra en el stack (t1).
    //    * Si t2 > t1 => haces un pop del stack y registras cuantos dias pasaron
    //    * Si t2 <= t1 => pusheas t2
    // Haciendo esto, te queda un stack en orden decreciente. Cuando encuentres 
    // un t2 mayor, vas a popear todo lo que puedas y luego pusheas t2.
    
    // En el stack guardo tanto el valor como el índice. El índice es para calcular
    // los días que pasaron cuando haga el pop
    let mut stack: Vec<(usize, i32)>  = vec![];
    let mut answer: Vec<i32> = vec![0; temperatures.len()];


    for (i, temp) in temperatures.iter().enumerate() {
        while !stack.is_empty() && *temp > stack.last().unwrap().1 {
            let (stack_index, _) = stack.pop().unwrap();
            answer[stack_index] = (i - stack_index) as i32;
        }

        stack.push((i, *temp));
    }
    // Como inicializaste todo a 0 en ans, no importa los que quedaron en el stack

    answer
}


// 6. Car Fleet	----------------------------------------------------------------
// There are n cars going to the same destination along a one-lane road. 
// The destination is target miles away.

// You are given two integer array position and speed, both of length n, where 
// position[i] is the position of the ith car and speed[i] is the speed of the 
// ith car (in miles per hour).

// A car can never pass another car ahead of it, but it can catch up to it and 
// drive bumper to bumper at the same speed. The faster car will slow down to 
// match the slower car's speed. The distance between these two cars is ignored 
// (i.e., they are assumed to have the same position).

// A car fleet is some non-empty set of cars driving at the same position and 
// same speed. Note that a single car is also a car fleet.

// If a car catches up to a car fleet right at the destination point, it will 
// still be considered as one car fleet.

// Return the number of car fleets that will arrive at the destination.

/// target: destination in miles
/// position: in miles
/// speed: in miles per hour
pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    // |----x----x--x-------x------x---|
    // Arriba tenés los autos. Cada x es un car fleet (1 o más autos).
    // Autos en un mismo car fleet tienen la misma posición.
    // Para que un auto pueda llegar a tener que reducir la velocidad, tiene que 
    // tener un auto más lento adelanta.
    // Quiero saber el tiempo de llegada de cada auto, suponiendo que no hay
    // tráfico => después puedo hacer lo siguiente:
    //     el auto i se va a frenar <=> hay un auto más adelante y que llego en
    //     un tiempo t mayor al suyo
    //     => el auto i va a llegar al mismo tiempo que este otro auto. Van a formar 
    //     un car fleet. Sino no.
    // => Para saber cuantos car fleets hay, tengo que saber cuantos autos 
    // tienen autos atrás que los alcanzan.
    // Arrancas recorriendo de derecha a izquierda. 
    // Llegas antes que yo? 
    // => te vas a chocar conmigo Me paro al final, recorro todo, borro los 
    // que se chocan conmigo. Voy al siguiente que quedo y repito.
    // Sos más lento que yo? 
    // Entonces yo no freno a más nadie, avanzo.
    // 
    // Al final me va a quedar el vector de longitud = car fleets.
    // Y lo haces en O(n)
    // Pero para esto los tenés que ordenar => finalmente te queda O(n*log n)
    // SOLUCIÓN

    // Stack donde voy cargando los tiempos de llegada. Cada tiempo de llegada es un fleet
    let mut stack: Vec<f64> = vec![];
    let mut cars: Vec<(i32, i32)> = position
        .into_iter()
        .zip(speed.into_iter())
        .collect();

    cars.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    for (pos, speed) in cars.iter().rev() 
    {
        let arrival_time = (target - pos) as f64 / *speed as f64;
        stack.push(arrival_time);
        // Si ya había un arrival time en el stack, y este era menor que el nuevo
        // => colisionan. Saco el que acabo de agregar
        if stack.len() >= 2 && stack.last() <= stack.get(stack.len() - 2) 
        {
            stack.pop();
        }
    }

    stack.len() as i32 
}

// HARD ------------------------------------------------------------------------
// 7. Largest Rectangle In Histogram -------------------------------------------
// Given an array of integers heights representing the histogram's bar height 
// where the width of each bar is 1, return the area of the largest rectangle 
// in the histogram.
pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
    // Tenés rectangulos de ancho 1 y altura x variable. Cuál es el rectángulo
    // de área máxima que podés meter adentro de los histogramas?
    // Primero lo primero. Dados n histogramas, como cálculo el área del 
    // mayor rectángulo que entra?
    // min(altura) * n
    // => a priori tendría que pararme en el primero e ir probando con todo 
    // el resto de histogramas. Esto sería O(n^2)
    // As usual, se puede hacer en O(n)
    // Como?
    // Vas a recorrer de izquierda a derecha. En un stack vas a ir pusheando
    // (index, altura).
    // Luego de pushear vas a avanzar al siguiente y te vas a preguntar lo siguiente:
    //   
    //   * Es más alto o igual?
    //     => pusheo en el stack y sigo avanzando. El rectangulo de esta altura
    //     se va a poder seguir extendiendo.
    //   
    //   * Es más chico?   
    //     El rectangulo de la altura anterior no se puede extender más hacia 
    //     la derecha.
    //     => ahora me voy a fijar que rectángulo puedo armar de altura de i-1.
    //     Si la altura es máx, la guardo.
    //     Hago un pop del stack (ya no me sirve la posición i)
    //     Ahora quiero ver que rectángulo podía armar hacia la izquierda:
    //     Calculas el área por la diferencia de indexes y la altura del elemento
    //     que popeaste.
    //     Hago otro pop. Así hasta llegar al final o llegar a un bloque más chico
    //     
    //     * Llegaste al final?
    //       Ahora hacemos el push. OBS: cuál va a ser el índice? Va a ser el del
    //       último elemento que popeamos.
    //       Sirve para cuando en los siguientes elementos volvamos a mirar para atrás,
    //       este te va a decir: "podés hacer de cuenta que hasta esta posición 
    //       tenemos todos la misma altura". Es para calcular el área
    //
    //     * Te encontraste una fila de menor altura?
    //       No hagas un pop, te va a seguir sirviendo este valor.
    // 
    //     Haces un push y le ponés el índice del último elemento que popeaste.
    //
    //  Cuando llegues al final, te van a quedar elementos en el stack. Termina
    //  de procesarlos. 
    //  Arrancas del final:
    //  El rectangulo va a ser de la altura del actual y va a ir todo a la derecha
    //  que pueda.
    //  OBS: en las alturas te quedo el índice que te dice cuanto puede ir hasta
    //  la izquierda.
    //
    // Es un bardo. Verlo visualmente con un ejemplo es mucho mejor pero tenés 
    // que poderlo explicar en palabras. Link para verlo:
    // https://www.youtube.com/watch?v=zx5Sw9130L0&t=344s
    // SOLUCIÓN

    let mut answer = 0;
    // En el stack solo necesito el index para ver la altura en heights
    let mut stack: Vec<usize> = Vec::new();
    // Seteamos los boundaries en 0
    heights.push(0);
    heights.insert(0, 0);

    for (i, height) in heights.iter().enumerate() {
        
        // previous_lowest_height = heights[*stack.iter().last().unwrap()];
        
        // Me fijo la última altura mínima con la cuál puedo avanzar a izquierda
        // desde mi altura actual. Solo si ahora soy más chico que mi anterior
        while stack.len() > 0 && heights[*stack.iter().last().unwrap()] > *height 
        {
            let j = stack.pop().unwrap();
            // Calculo el área
            let left_limit = stack[stack.len() - 1];
            let width = ((i - 1) - left_limit) as i32;
            let size = width * heights[j];

            answer = answer.max(size);
        }

        stack.push(i);
    }

    answer
}