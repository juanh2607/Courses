// EASY ------------------------------------------------------------------------
// 1. Binary Search	------------------------------------------------------------
// Given an array of integers nums which is sorted in ascending order, and an 
// integer target, write a function to search target in nums. If target exists, 
// then return its index. Otherwise, return -1.

// You must write an algorithm with O(log n) runtime complexity.
use std::cmp::Ordering::{Less, Equal, Greater};

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    // La única forma de hacerlo en O(log n) es con búsqueda binaria
    let (mut left, mut right) = (0, nums.len());

    // Como al left le sumas mid + 1, en la última iteración te va a quedar que 
    // left es igual o uno más que right.
    while left < right
    {
        // OBS: si left y right son valores muy grandes, podés tener un overflow
        // => let mid = (left + right) / 2; no es la forma de hacer el cálculo 
        // para ese caso. 
        // Como los números son muy grandes => no queremos sumarlos. Como hacemos?
        // Así: sumale a left la mitad de su distancia con right
        let mid = left + ((right - left) / 2);
        
        match target.cmp(&nums[mid]) {
            Equal => return mid as i32,
            // OBS: yo acá pondría (mid - 1) y en el while podrías sacar el =
            // El problema es que hay casos en los que tenés un overflow del usize
            // (pasa a ser -1). Lo pasaria a i32 pero mid sería i32 y no podés 
            // indexar nums con un i32.
            Less => if mid == 0 { // Evito overflow
                return -1;
            } else {
                right = mid
            },
            Greater => left = mid + 1,
        }
    }

    -1
}

// MEDIUM ----------------------------------------------------------------------
// 2. Search a 2D Matrix -------------------------------------------------------
// You are given an m x n integer matrix matrix with the following two properties:

// Each row is sorted in non-decreasing order.
// The first integer of each row is greater than the last integer of the previous row.
// Given an integer target, return true if target is in matrix or false otherwise.

// You must write a solution in O(log(m * n)) time complexity.
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    // Es una matriz donde si recorres de izquierda a derecha va a estar ordenado
    // => esto implica que las columnas también están ordenadas en orden creciente
    // Lo más fácil es hacer búsqueda binaria por cada fila y listo.
    // El problema es que eso es O(m * log n)
    // Como lo haces en O(log m * n)?
    // A priori, fijandote por columna podés descartar filas enteras: 
    //    si el primer elemento de la fila es más chico que el target =>
    //    todas las filas anteriores las descarto. Mismo pero al revez si es 
    //    más grande.
    // Una vez que tengo la fila en que se encuentra, hago búsqueda binaria 
    // dentro de esa fila.
    // => resulta O(log(m) + log(n)) = O(log m * n)
    let (mut low, mut high) = (0, matrix.len() - 1);
    let mut row = 0;
    
    // Busco la fila en la que puede estar el target
    while low <= high 
    {
        row = (low + high) / 2;
        // Para determinar la fila, tengo que ver tanto el primero como el último valor
        let first = matrix[row][0];
        let last = *matrix[row].last().unwrap();

        if target == first || target == last {
            return true;
        } else if target < first {
            if row == 0 { // Evito el overflow
                break;
            } else {
                high = row - 1;
            }
        } else if target > last {
            low = row + 1;
        } else { // El target está dentro de la fila
            break;
        }
    }

    if low > high { // No se encontró una fila
        return false;
    }

    // Ahora hago la búsqueda binaria sobre el último mid que quedó
    // Tenés que pasar una referencia, hago clone porque el search se adueñaba del
    // vector como estaba definido en LeetCode
    if search(matrix[row].clone(), target) != -1 {
        return true;
    }

    false
}

// 3. Koko Eating Bananas ------------------------------------------------------
// Koko loves to eat bananas. There are n piles of bananas, the ith pile has 
// piles[i] bananas. The guards have gone and will come back in h hours.

// Koko can decide her bananas-per-hour eating speed of k. Each hour, she 
// chooses some pile of bananas and eats k bananas from that pile. If the pile 
// has less than k bananas, she eats all of them instead and will not eat any 
// more bananas during this hour.

// Koko likes to eat slowly but still wants to finish eating all the bananas 
// before the guards return.

/// piles: each position has an amount of bananas
/// h: hours Koko has for eating
/// Return the minimum integer k such that she can eat all the bananas within h hours.
/// k = bananas per hour koko can eat
pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    // Koko puede comer como máximo una pila por hora (no importa el tamaño)
    // =>
    // solo va a poder tener una chance de comer todas las pilas si la cantidad
    // de pilas es <= la cantidad de horas que tiene para comer
    // Ahora, si se cumple esto, puedo elegir un k muy alto y listo, Koko va a
    // poder comer todo, pero cuál es el mínimo k tal que sigue comiendo todo?
    // En principio lo fácil es determinar la cota máxima: la altura de la pila 
    // más grande.
    // => podría simplemente ir probando con esa cota máxima, recorrer todo el
    // arreglo piles y determinar si supere h. Si lo superaste => resta en uno k.
    // Este sería el metodo de fuerza bruta, que es O(cota_max * n) (tenés que recorrer
    // todo el array (cota max - k) veces).
    // Como lo mejoro?
    // Lo podés pensar como una búsqueda binaria. Imgaginate un array 
    // [1..cota máx]. Por fuerza bruta vas probando todos los posibles valores
    // de k.
    // Además ese array está ordenado => anda probando en el medio y descartando 
    // mitades. Mientras tanto te vas guardando el valor más chico de k que encontraste
    // Criterio:
    //  * Si con este k no pudiste => tampoco vas a poder con ninguno más chico
    //  * Si con este k pudiste =>
    //    * Encontraste el minimo (como a priori no sabes, lo guardas)
    //    * Cualquier valor más grande de k no va a ser el mínimo, los descarto
    // Esto termina siendo O(log(cota max) * n)
    let max_k = *piles.iter().max().unwrap() as usize;
    let mut min_k = max_k;

    // Búsqueda binaria para determinar el k mínimo
    let (mut left, mut right) = (1, max_k);

    while left <= right 
    {
        let k: usize = (left + right) / 2;
        // Una cosa por la que esta piola hacerlo con map en vez de un for es que
        // hours te queda inmutable
        let hours: usize = piles.iter() 
            .map(|&q| {
                // En este caso queremos redondear para arriba. La división entera
                // redondea siempre para abajo. El + 1 cancela esto
                // Y el -1? Es para el caso en que q es múltiplo de k, en ese 
                // caso no queremos redondear.
                // No afecta al resto de los casos, da lo mismo.
                ((q - 1) as usize / k) + 1
            })
            .sum();

        match hours.cmp(&(h as usize)) {
            Less | Equal => { // Es un k válido
                min_k = min_k.min(k);
                right = k - 1;
            },
            Greater => left = k + 1,
        }
    }

    min_k as i32

}

// 4. Find Minimum In Rotated Sorted Array -------------------------------------
// Suppose an array of length n sorted in ascending order is rotated between 1 
// and n times. For example, the array nums = [0,1,2,4,5,6,7] might become:

// [4,5,6,7,0,1,2] if it was rotated 4 times.
// [0,1,2,4,5,6,7] if it was rotated 7 times.
// Notice that rotating an array [a[0], a[1], a[2], ..., a[n-1]] 1 time results 
// in the array [a[n-1], a[0], a[1], a[2], ..., a[n-2]].

// Given the sorted rotated array nums of unique elements, return the minimum 
// element of this array.

// You must write an algorithm that runs in O(log n) time.
pub fn find_min(nums: Vec<i32>) -> i32 {
    // La idea es esta: si el array fue desplazado, va a ocurrir lo siguiente:
    // vas a tener dos partes que están ordenadas (parte izquierda y derecha).
    // El mínimo se va a encontrar en la parte derecha.
    // => lo que tengo que hacer es determinar si estoy en la parte derecha o no.
    // Como hago?
    // Si en donde estoy parado soy más grande que el right pointer =>
    // estoy en el lado izquierdo => sigo buscando del lado derecho: left = mid + 1.
    // Ahora, si donde estoy parado soy más chico que el left pointer =>
    // estoy del lado derecho => sigo buscando del lado izquierdo
    // Para ver si estoy o no en el mínimo, simplemente tengo que ver si el valor
    // que está a mi izquierda es más grande que yo. Si lo es => soy el mínimo.

    // Primero reviso si el vector está correctamente ordenado
    if nums.first().unwrap() <= nums.last().unwrap() {
        return nums[0];
    } 

    // El vector fue desplazado, hago la búsqueda binaria
    let (mut left, mut right) = (0, nums.len() - 1);
    while left < right 
    {
        let mid = (left + right) / 2;

        if nums[left] < nums[right] 
        {
            // El intervalo en el que estoy, está ordenado ascendentemente en 
            // su totatlidad. El mínimo tiene que ser el izquierdo
            // Si no lo fuera, significa que lo había dejado fuera y eso no puede pasar.
            return nums[left];
        } 
        else if nums[mid + 1] < nums[mid] 
        {
            // Si en donde estoy soy más chico que el de mi izquierda => soy el min
            return nums[mid + 1];
        } 
        else if nums[left] > nums[mid] 
        {
            // Estoy parado en la mitad ordenada derecha, quiero ir a la izquierda
            right = mid;
        } 
        else 
        { 
            // Estoy parado en la mitad ordenada derecha, quiero ir a la derecha
            left = mid;
        }
    }

    -1

    // Realmente no tengo idea porque esto no funciona. Pasa 2/3 pruebas. Al
    // debuggear la que falla, nums empieza a cambiar de tener los valores
    // correctos a basura. No entiendo nada
    /*while left <= right {
        let mid = (left + right) / 2;

        if nums[mid] < nums[mid - 1] { // Encontré el mínimo
            return nums[mid];
        } else if nums[mid] < nums[left] { // Estoy del lado ordenado derecho
            right = mid - 1;
        } else if nums[mid] > nums[right] { // Estoy del lado ordenado izquierdo
            left = mid + 1;
        }
    }

    unreachable!()*/
}
// 5. Search In Rotated Sorted Array -------------------------------------------
// There is an integer array nums sorted in ascending order (with distinct values).

// Prior to being passed to your function, nums is possibly rotated at an unknown 
// pivot index k (1 <= k < nums.length) such that the resulting array is 
// [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] 
// (0-indexed). For example, [0,1,2,4,5,6,7] might be rotated at pivot index 3 
// and become [4,5,6,7,0,1,2].

// Given the array nums after the possible rotation and an integer target, 
// return the index of target if it is in nums, or -1 if it is not in nums.

// You must write an algorithm with O(log n) runtime complexity.
pub fn search_2(nums: Vec<i32>, target: i32) -> i32 {
    // Vas a tener dos partes que están ordenadas ascendentemente =>
    // ahora no solo tengo que ver el mid, sino que tambien tengo que tener en
    // cuenta el valor de left y right:
    // Ej: target 9 [6, 7, 8, 9, 10, 0, 1, 2, 3, 4, 5]
    // 1. 0 es más chico que 9, pero el 9 podría estar a mi derecha o izquierda
    // 2. Está a mi derecha? No, porque 5 es más chico que 9. Y donde estoy 
    //    es más chico que 5 y se que va en orden ascendente.
    // 3. Está a mi izquierda? Si, porque 6 es más chico que 9 y donde estoy
    //    es más chico que 6 se que va en orden ascendente (estoy en la porción derecha)
    // 4. Ahora busco en [6, 7, 8, 9, 10]
    // 5. 8 es más chico que 9, voy a izquierda o derecha?
    // 6. Left (6) es más chico que yo y que 9 => del lado izquierdo se que no
    //    está.
    // 7. Lado derecho? Si, porque 10 es más grande que 8 y que 9 => voy a ese lado
    // 8. [9, 10], encontré 9.
    //   * mid es más chico que el target => el target puede estar tanto a mi 
    //     izquierda como a mi derecha:
    //     * Si right es más chico que el target => target está del lado izquierdo
    //     * Si right es más grande => target está del lado derecho
    //   * mid es más grande que el target => el target puede estar tanto a mi 
    //     izquierda como a mi derecha:
    //     * Si left es más chico que el target => target está del lado izquierdo.
    let (mut left, mut right) = (0, nums.len() - 1);

    while left <= right 
    {
        let mid = (left + right) / 2;
        let l = nums[left];
        let r = nums[right];
        let m = nums[mid];

        if m == target {
            return mid as i32;
        } 
        
        if l <= m { // Estás del lado ordenado izquierdo => desde l a m crece
            if target < l || target > m { // Target está del lado derecho
                left = mid + 1;
            } else { // Target está del lado izquierdo
                right = mid - 1;
            }
        } else { // Estás del lado ordenado derecho (m < l)
            if target < m || target > r { // Target está del lado izquierdo
                right = mid - 1;
            } else { // Target está del lado derecho
                left = mid + 1;
            }
        }
    }
    
    -1
}

// 6. Time Based Key Value Store -----------------------------------------------

// HARD ------------------------------------------------------------------------
// 7. Median of Two Sorted Arrays ----------------------------------------------