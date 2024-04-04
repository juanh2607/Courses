use std::cmp::Ordering::{Equal, Greater, Less};
use std::cmp::min;

// EASY ------------------------------------------------------------------------
// 1. Valid Palindrome ---------------------------------------------------------
// A phrase is a palindrome if, after converting all uppercase letters into 
// lowercase letters and removing all non-alphanumeric characters, it reads the 
// same forward and backward. Alphanumeric characters include letters and numbers.

// Given a string s, return true if it is a palindrome, or false otherwise.
pub fn is_palindrome(s: String) -> bool {
    // Primero tengo que "estandarizar" el string
    let s = s.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect::<String>();

    let bytes = s.as_bytes();

    // Ahora recorro de una corrida en ambos sentidos.
    for i in 0..(bytes.len() / 2) {
        if bytes[i] != bytes[bytes.len() - 1 - i] {
            return false;
        }
    }
    
    true
}

// MEDIUM ----------------------------------------------------------------------
// 2. Two Sum II - Input Array Is Sorted ---------------------------------------
// Given a 1-indexed array of integers numbers that is already sorted in 
// non-decreasing order, find two numbers such that they add up to a specific 
// target number. Let these two numbers be numbers[index1] and numbers[index2] 
// where 1 <= index1 < index2 <= numbers.length.

// Return the indices of the two numbers, index1 and index2, added by one as an 
// integer array [index1, index2] of length 2.
// OBS: por cualquier motivo, usan un índice de base 1 en vez de base 0. Ni idea

// The tests are generated such that there is exactly one solution. You may not 
// use the same element twice.

// Your solution must use only constant extra space.

// use std::cmp::Ordering::{Equal, Greater, Less};


pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    // Como está ordenado, podés hacer la siguientes mejoras:
    // Busca el target y ya sabes que podés desechar la parte de la derecha 
    // (valdría la pena para un volúmen de datos muy grandes)
    // Después recorres con dos vectores: uno desde el principio y otro desde
    // el final => una sola iteración. Es O(n).
    // La idea es ir sumando y dependiendo del valor sabés que puntero avanzar,
    // hasta que tengas el target
    let (mut left, mut right) = (0, numbers.len() - 1);
    
    while left < right {
        match (numbers[left] + numbers[right]).cmp(&target) {
            Less => left += 1,
            Greater => right -= 1,
            Equal => return vec![left as i32 + 1, right as i32 + 1]
        }
    }
    // Not found
    vec![] 
}


// 3. 3Sum ---------------------------------------------------------------------
// Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] 
// such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.

// Notice that the solution set must not contain duplicate triplets.
pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    // A fuerza bruta haces 3 loops anidados: O(n^3)
    // Como mejoro eso?
    // La idea es la siguiente:
    // 1. Ordenas el vector: esto te permite eliminar duplicados ya que si estás
    //    parado en i y si en i - 1 está el mismo número seguí de largo porque 
    //    significa que ya analizaste todas las opciones con ese número
    // 2. Vas a empezar a recorrer el array. Cuando te paras en una posición, 
    //    tenés que buscar un par de números más tal que la suma te de 0. Como?
    //    Es un two sum, solo que te tiene que dar el número que está fijo en 
    //    vez de 0 y tenés que buscar todos los valores en vez de uno solo 
    //    (solo tenés que seguir iterando).
    //
    // Esto termina siendo O(n^2) ya que tenés solo dos loops anidados.
    // El ordenamiento es O(n*log n) => O(n*log n) + O(n^2) = O(n^2)
    merge_sort(&mut nums); // O(n*log n)
    // nums.sort_unstable(); // + fácil

    let mut result: Vec<Vec<i32>> = Vec::new(); 

    // O(n^2)
    for i in 0..nums.len() {
        // i va a ser el valor que queda fijado. Ahora hago un two sum con el resto
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let (mut left, mut right) = (i + 1, nums.len() - 1);

        while left < right {
            match (nums[left] + nums[right]).cmp(&(-nums[i])) {
                Less => left += 1,
                Greater => right -= 1,
                Equal => {
                    result.push(vec![nums[i], nums[left], nums[right]]);
                    
                    left += 1;
                    while nums[left] == nums[left - 1] && left < right {
                        left += 1;
                    }

                    right -= 1;
                    while nums[right] == nums[right + 1] && left < right {
                        right -= 1;
                    }
                }
            }
        }   
    }

    result

    // Otra solución piola TODO: verla bien
    // nums.sort();
    //     let mut answers = std::collections::BTreeSet::new();
    //     for (i, &n) in nums.iter().enumerate() {
    //         if n > 0 {
    //             break;
    //         }
            
    //         let mut memo = std::collections::HashSet::new();
    //         for &a in &nums[(i + 1)..] {
    //             let target = -(n + a);
    //             if memo.contains(&target) {
    //                 answers.insert((n, target, a));
    //             }
    //             memo.insert(a);
    //         }
    //     }
    //     answers.into_iter().map(|(a, b, c)| vec![a, b, c]).collect()
}

// Como leerlo: defino T como un tipo de dato que implementa el trait Ord.
// x es una referencia mutable a un slice de T 
// OBS: podrías usar Clone en vez de Copy. Copy es más restrictivo pero más rápido
// TODO: rust tiene su propia implementación del merge sort. Es legible. Intenta implementarla entendiendola bien
pub fn merge_sort<T: Ord + Copy>(x: &mut [T]) {
    let mid = x.len() / 2;

    if x.len() <= 1 {
        return;
    }

    // Seguís partiendo hasta llegar al caso base
    merge_sort(&mut x[0..mid]);
    merge_sort(&mut x[mid..]);

    let mut y: Vec<T> = vec![x[0]; x.len()];
    // Cada partición ordena su slice de x, hasta que finalmente ordenas las dos mitades finales
    merge(&x[0..mid], &x[mid..], &mut y[..]);
    x.copy_from_slice(&y);
}

// Recibe dos arrays ordenados ascendentemente (x1 y x2) y los carga de forma
// ordenada en y.
fn merge<T: Ord + Copy>(x1: &[T], x2: &[T], y: &mut [T]) {
    // TODO: ver manejo de errores más elegante que paniquear
    assert_eq!(x1.len() + x2.len(), y.len());
    // La idea es ir recorriendo los dos arrays e insertando el valor más chico de los dos
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < x1.len() && j < x2.len() {
        if x1[i] < x2[j] {
            y[k] = x1[i];
            i += 1;
            k += 1;
        } else {
            y[k] = x2[j];
            j += 1;
            k += 1;
        }
        // No hace falta checkear si x1[i] == x2[j], se soluciona igual
    }

    // Se terminó el array más corto de los dos. Inserto el resto del otro
    if i < x1.len() {
        y[k..].copy_from_slice(&x1[i..]);
    } else if j < x2.len() {
        y[k..].copy_from_slice(&x2[j..]);
    }
}

// 4. Container With Most Water ------------------------------------------------
// You are given an integer array height of length n. There are n vertical lines 
// drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).

// Find two lines that together with the x-axis form a container, such that the 
// container contains the most water.

// Return the maximum amount of water a container can store.

// Notice that you may not slant the container. slant = "inclinar"
pub fn max_area(height: &Vec<i32>) -> i32 {
    // El vector contiene diferentes alturas. La idea es agarrar las alturas tal 
    // que el área sea máxima (teniendo también en cuenta la distancia en x). 
    // La joda del agua es que la altura del agua va a ser igual a la menor 
    // altura entre las dos paredes.
    // =>
    // Lo primero es hacerlo por fuerza bruta: me paro en cada pared, itero
    // sobre las siguientes mientras calculo el área y me voy quedando con la
    // mayor área => O(n^2).
    // Como lo mejoro?
    // Se puede resolver recorriendo el arreglo una sola vez (O(n)). La idea es 
    // tener un left y right pointer y empezar de ambas puntas.
    // Ej: tenés 1 | 7 => el área va a ser de 1*1*n. Esta va a ser el área máxima
    // que puedo lograr con la columna esa de altura 1 => no tengo que comparar
    // con el resto de columnas, simplemente calculo el área y avanzo el left 
    // pointer. Ahora tenés 8 | 7. Misma idea. Con el 7 no vas a lograr obtener un
    // área más grande ya que si avanzas más, achicas el ancho. Cálculas el área
    // y seguís. Así hasta que llegues al medio y solo recorriste el array una vez
    let (mut left, mut right) = (0, height.len() - 1);
    let mut max_area = 0;

    while left < right {
        let h = min(height[left], height[right]);
        let w = right - left;
        let area = h * w as i32;

        if area > max_area {
            max_area = area;
        }

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_area
}


// HARD ------------------------------------------------------------------------
// 5. Trapping Rain Water ------------------------------------------------------
// Given n non-negative integers representing an elevation map where the width 
// of each bar is 1, compute how much water it can trap after raining.
pub fn trapped_water(height: &Vec<i32>) -> i32 {
    // Te paras en el primer indice con altura > 0. Vas a la derecha: si la altura
    // en i + 1 es más chico => la diferencia en altura la acumulas como agua. 
    // Si la altura es >= a i => ahora te parás en el nuevo índice y seguís el proceso
    // => solo tenés que recorrer el array una vez => O(n)
    // Usas dos punteros
    // OBS: esto no funciona si la pared del lado izquierdo es más alta que la 
    // del derecho.

    // La idea es que te parás en cada punto i y checkeas a los dos costados cuales
    // son las alturas máximas. El mínimo de esas alturas va a ser el cuello de botella
    // que determina cuanta agua tenés arriba tuyo.
    // Ej: 2 1 2 3.
    // Te parás en el 1. El máx a la izquierda es 2. El máx a la derecha es el 3.
    // Como 2 es el mínimo hago 2 - 1 = 1 => tengo 1 bloque de agua arriba mio
    // SOLUCIÓN 1
    // Creas 3 vectores: maxLeft (donde guardas cuál es la máxima altura a la 
    // izquierda en la posición i. Lo sacás fácil, es solo recorrer manteniendo 
    // valor máximo. Es O(n)). Un max right (misma idea, solo recorres a la inversa).
    // Finalmente tenés un array en donde guardás el mínimo entre los máximos 
    // de izquierda / derecha.
    // Hasta ahora eso lo podés hacer en 2n (el min lo armás mientras encontrás
    // el máxRight).
    // El cálculo que haces al final es este: minLeftRight[i] - height[i]. Si te da
    // negativo va un 0. Sino, te queda el total de agua acumulada en esa posición    
    // Esta solución es buena pero se puede mejorar todavía más. Su complejidad 
    // em términos de memoria es O(n). Se puede hacer que sea O(1).

    // SOLUCIÓN FINAL
    // Usar dos punteros, uno para el principio y uno para el final. En vez de 
    // vectores ahora tenés dos variables: max_left y max_right. A medida que avanzas
    // con un puntero, si encontrás un height más grande entonces lo actualizas.
    // Vas a usar estos valores para saber si en i tenés agua acumulada.
    // De lo que se aprovecha esta solución es que, ponele que a la izquierda tenés
    // un 1. Para saber si acumulás agua, solo te importan 3 cosas: que a la izquierda
    // tenés un 1, tu altura actual y que en cualquier parte de la derecha hay algo
    // con altura de 1 o más.    
    let (mut left, mut right) = (0, height.len() - 1);
    let mut highest_left = height[left];
    let mut highest_right = height[right];
    let mut trapped_water = 0;

    while left < right {
        // Avanza el puntero del extremo más chico
        if highest_left <= highest_right {
            left += 1;

            highest_left = highest_left.max(height[left]);
            trapped_water += highest_left - height[left];
        } else {
            right -= 1;

            highest_right = highest_right.max(height[right]);
            trapped_water += highest_right - height[right];
        }
    }

    trapped_water
}