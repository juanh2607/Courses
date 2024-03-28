use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::vec;

// EASY ------------------------------------------------------------------------

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

// 3. Two Sum ------------------------------------------------------------
// Given an array of integers nums and an integer target, return indices of the 
// two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, and you may 
// not use the same element twice.
// You can return the answer in any order.
pub fn two_sum(a: &Vec<i32>, target: i32) -> Vec<i32> {
    // HashMap stores (value, index)
    let mut map: HashMap<&i32, i32> = HashMap::new();

    for (i, value) in a.iter().enumerate() {
        let diff = target - value;

        if let Some(&j) = map.get(&diff) {
            return vec![i as i32, j];
        }

        map.insert(value, i as i32);
    } 

    vec![-1, -1] // Now answer found
}

// MEDIUM ----------------------------------------------------------------------

// 4. Group Anagrams -----------------------------------------------------------
// Given an array of strings strs, group the anagrams together. You can return 
// the answer in any order. Words contain a-z.

// COMPLEJIDAD: Es O(m*n), donde m son la cantidad de vectores (los tenés que 
// recorrer a todos, y n es la longitud de la palabra más larga (tenés que 
// contar la frecuencia de todos los chars))
// OBS: los hashmaps son O(1) tanto para inserción como para obtener datos
// (en los casos promedios). El peor de los casos es O(n)
pub fn group_anagrams(strs: &Vec<String>) -> Vec<Vec<String>> {
    // [u8; 26] es un array u8 de 26 espacios. Para que? Las letras son de a-z 
    // entonces tenés 26 chars. Este array lo usas para contar la cantidad de 
    // veces que aparece cada char. La posición te indica que char es.
    // El hashmap lo necesitas para asociarlo al string.
    let mut hmap: HashMap<[u8; 26], Vec<String>> = HashMap::new();

    for word in strs {
        // Inicializa todo a 0
        let mut key = [0_u8; 26];
        // Cuento la cantidad de veces que aparece cada letra (frecuencia)
        for c in word.chars() {
            // Pasas los chars a su nro ASCII. 'a' es el 0. 'b' el 1 y así
            key[c as usize - 'a' as usize] += 1;
        }
        // Ahora la key es ese array. Si coincide con alguna en el map 
        // => son anagramas
        if let Some(anagrams) = hmap.get_mut(&key) {
            // to_string es para generar una copia
            anagrams.push(word.to_string());
        } else { // Nuevo anagrama
            hmap.insert(key, vec![word.to_string()]);
        }
    }

    hmap.into_values().collect()
}

// 5. Top K Frequent Elements -----------------------------------------------------------
// Given an integer array nums and an integer k, return the k most frequent elements. 
// You may return the answer in any order.
pub fn top_k_frequent(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    // La idea es esta: te llega un vector de longitud n. Vas a crear un array 
    // en donde en la posición X vas a tener todos los elementos de frecuencia X
    // Se le dice Bucket Sort. Cuando quieras sacar los K de mayor frecuencia, 
    // simplemente haces pop empezando desde el final hasta tener k elementos.

    // Guarda frecuencias
    let mut hmap: HashMap<i32, i32> = HashMap::new();
    // Donde la posición i tiene un arreglo con los elementos de frecuencia i
    let mut bucket: Vec<Vec<i32>> = vec![vec![]; nums.len() + 1];
    // Cargo frecuencias
    for n in nums {
        *hmap.entry(*n).or_insert(0) += 1;
    }
    // Asigno cada elemento a su correspondiente posición en el bucket
    for (k, v) in hmap {
        bucket[v as usize].push(k);
    }
    
    let mut result: Vec<i32> = vec![];
    // Ahora me quedo solo con los K más frequentes:
    for freq in bucket.iter().rev() {
        for item in freq {
            if result.len() < k as usize {
                result.push(*item);

                if result.len() == k as usize {
                    return result;
                }
            }
        } 
    }

    unreachable!();
}

// 6. Product of Array Except Self ---------------------------------------------
// Given an integer array nums, return an array answer such that answer[i] is equal 
// to the product of all the elements of nums except nums[i].

// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

// You must write an algorithm that runs in O(n) time and without using the division operation.
pub fn product_except_self(nums: &Vec<i32>) -> Vec<i32> {
    // Lo típico sería hacer dos for anidados y listo, pero eso es O(n^2)
    // Lo que es mejor es tener un array result al que primero agregás el primer
    // número en cada posición y después para cada número de nums multiplicas
    // todo por eso (menos la posición i). Podrías directamente multiplicar 
    // todo el array y dividir la posición i por el valor i pero no te dejan usar división.
    
    // Solución: para cada i pensá que te queda el array partido en dos partes,
    // un "prefix" y un "postfix". Multiplicando estas dos partes obtenés el valor
    // que querés para la posición i.
    // => vas a recorrer el array dos veces (O(2n) = O(n)). Usas un solo array
    // así que el uso de memoria es O(n)
    // En la primera pasada vas acumulando un valor "prefix". Lo agregas a la 
    // posición i y después lo multiplicas por n (acumulas).
    // En la segunda pasada la idea es la misma, pero recorres el array en sentido
    // contrario.

    let mut result: Vec<i32> = vec![1; nums.len()];

    let mut prefix = 1;
    // Agrego el valor de los prefijos multiplicados. Se va acumulando el valor
    for (i, n) in nums.iter().enumerate() {
        result[i] = prefix;
        prefix = prefix * n; // Acumulo valor para el siguiente
    }

    let mut postfix = 1;
    // Agrego el valor de los postfix. Misma idea pero recorro el array al revéz
    for (i, n) in nums.iter().enumerate().rev() {
        result[i] = result[i] * postfix;
        postfix = postfix * n; // Acumulo valor para el siguiente
    }
    
    result
}

// 7. Valid Sudoku -------------------------------------------------------------
// Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be 
// validated according to the following rules:
// 1. Each row must contain the digits 1-9 without repetition.
// 2. Each column must contain the digits 1-9 without repetition.
// 3. Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 
//    without repetition
// Notes:
// A Sudoku board (partially filled) could be valid but is not necessarily solvable.
// Only the filled cells need to be validated according to the mentioned rules.
// OBS: empty spaces are marked as "."
pub fn is_valid_sudoku(board: &Vec<Vec<char>>) -> bool {
    // Checkeo que números tiene una fila, columna o caja usando hashsets
    // Si o si tengo que recorrer toda la matriz => complejidad O(n). Como insertar
    // en un hashset es O(1), no afecta.
    const EMPTY: char = '.';
    
    let mut row_nums: HashSet<char> = HashSet::new();
    let mut col_nums: HashSet<char> = HashSet::new();
    let mut box_nums: HashSet<char> = HashSet::new();

    // Como las filas, columnas y subcajas son todo 9x9, podés recorrer todo de una
    for i in 0..9 {
        for j in 0..9 {
            // row y col tienen la misma lógica
            let row_num = board[i][j];
            let col_num = board[j][i];
            // Ahora tengo que iterar sobre la caja i.
            // Partimos al sudoku en 9 cajas, que las identificamos por [fil, col]
            // donde ambos van de 0 a 2. Para obtener la fila de la caja, simplemente
            // dividís la fila por 3 y redondeas para abajo. Misma idea con la columna
            // Los i/3*3 y j%3*3 son conceptuales, acordate que i/3 indica la fila 
            // de la sub caja. Lo multiplicás por 3 porque necesitas
            // El % te da el resto. O sea que por las tres iteraciones que
            // i/3 da lo mismo (redondea para abajo), con el resto estás avanzando
            // de a uno
            // Para verlo bien expícito, pedile a chatgpt: Podrías darme el listado 
            // de los siguientes resultados: x/3 y x%3 para x entre 0 y 8 inclusive?
            let box_num = board[i/3*3 + j/3][i%3*3 + j%3];
            
            if row_num != EMPTY {
                if row_nums.insert(row_num) == false {
                    return false;
                }
            }

            if col_num != EMPTY {
                if col_nums.insert(col_num) == false {
                    return false;
                }
            }

            if box_num != EMPTY {
                if box_nums.insert(box_num) == false {
                    return false;
                }
            }
        }
        row_nums.clear();
        col_nums.clear();
        box_nums.clear();
    }

    true
}

// 8. String Encode and Decode -------------------------------------------------
// Design an algorithm to encode a list of strings to a single string. 
// The encoded string is then decoded back to the original list of strings.

// Esta es más una pregunta de diseño.
// Forma naive es usar un único char "especial" como delimiter pero es 
// "bastante" posible que genere fallas ya que puede usarse en cualquier otro
// contexto ese char.
// Como hacemos para que no entre en juego la probabilidad de casualmente 
// encontrarse con el delimiter?
// Aplicamos el siguiente protocolo:
// <n>#<n chars>
// Es decir: cada palabra va a empezar con un entero que me indica la longitud 
// la palabra. Además se que voy a tener que leer bytes para el número hasta 
// encontrarme con un #. Luego simplemente leo n bytes, no me importa lo que haya
// dentro del string.

pub struct Codec {}

impl Codec {
    pub fn new() -> Self {
        Self {}
    }

    const DELIM: char = '#';

    // Encodes an array of strings into a single string
    pub fn encode(&self, strs: &Vec<String>) -> String {
        let mut encoded = String::new();

        for word in strs {
            encoded.push_str(&word.len().to_string());
            encoded.push(Self::DELIM);
            encoded.push_str(word);
        }

        encoded
    }

    pub fn decode(&self, s: &String) -> Vec<String> {
        let mut decoded: Vec<String> = vec![];
        let mut i = 0;

        while i < s.len() {
            let delim_index = s[i..].find(Self::DELIM);

            if let Some(index) = delim_index {
                // Obtengo longitud de la palabra
                let len_str = &s[i..(i + index)];

                // TODO: en una aplicación sería no paniqueas acá si falla
                // solo retornarias un error. Podrías testearlo también.
                let len = len_str.parse::<usize>().unwrap();
                // Leo la palabra
                let start = i + index + 1;
                let end   = i + index + 1 + len;
                let word = &s[start..end];

                decoded.push(word.to_string());

                i += index + 1 + len;
            } else { // No hay más delimitador
                break; 
            }
        }

        decoded
    }
}

// 9. Longest Consecutive Sequence ---------------------------------------------
// Given an unsorted array of integers nums, return the length of the longest 
// consecutive elements sequence.

// You must write an algorithm that runs in O(n) time.
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    // No podés ordenar ya que eso es O(n*log n)
    // 1. Convertís el vector a un HashSet (O(n))
    // 2. Vas a buscar a todos los números que comienzen una secuencia. Como?
    //    Te fijás si en el HashSet se encuentra su anterior (O(1))
    // 3. Si es un primer elemento, creas un vector con ese.
    // 4. Intentas "agrandar" el vector lo más que puedas, buscando el siguiente valor
    // 5. Guardas el resultado y seguís con la siguiente secuencia.

    // Costo? Cada número lo vas a leer como mucho 2 veces: siempre tenés que leer
    // todo el vector y después tenés que buscar los secuenciales por cada inicio de
    // secuencia. Esto es O(2n) = O(n), mucho mejor que O(n * log n)
    let hset: HashSet<i32> = nums.into_iter().collect();
    let mut longest = 0;

    for n in &hset {
        // Checkeo si es el primero de una secuencia
        if !hset.contains(&(n - 1)) {
            let mut current_len = 1;
            // Cuento hasta que se corte la secuencia
            while hset.contains(&(n + current_len)) {
                current_len += 1;
            }

            if current_len > longest {
                longest = current_len;
            }
        } 
    }

    longest
}