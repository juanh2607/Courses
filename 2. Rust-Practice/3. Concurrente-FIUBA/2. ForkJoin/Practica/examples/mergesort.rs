// Este archivo implementa un merge-sort secuencial. Es un algoritmo de ordenamiento
// basado en el divide & conquer. Partis en listas pequeñas que son más fáciles de ordenar
// y vas mezclando las sublistas de forma ordenada hasta llegar al resultado final.

use std::thread;
use std::time::{Duration, Instant};

fn main() {
	let data = [7, 3, 2, 16, 24, 4, 11, 9];
	let start = Instant::now();

	let merged = mergesort(&data);

	println!("{:?}", start.elapsed());

	// Con retraso de 2 segundos tarda 30s
	// Sin retraso tarda 7 nanosegundos

    println!("{:?}", merged);
}

fn mergesort(data: &[i32]) -> Vec<i32> {
	thread::sleep(Duration::from_secs(2)); // Simular retraso
	let mid = data.len() / 2; // En realidad deberías redondear
	if mid == 0 { // Data está vacío o tiene un solo elemento. Condición de corte para cuando ya tenés bloques de 1
		return data.to_vec();
	}

	let left_data = &data[..mid];
	let right_data = &data[mid..];
	// Llamada recursiva para seguir partiendo hasta tener todos bloques de 2 datos
	let left = mergesort(left_data);
	let right = mergesort(right_data);

	merge(left, right)
}

fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
	let mut left_index = 0;
    let mut right_index = 0;
    let mut result_index = 0;
    let mut result = vec![0; left.len() + right.len()];

	while left_index < left.len() && right_index < right.len() {
		if left[left_index] <= right[right_index] { // Insertá el izquierdo
			result[result_index] = left[left_index];
			left_index += 1;
			result_index += 1;
		} else { // Insertá el derecho
			result[result_index] = right[right_index];
			right_index += 1;
			result_index += 1;
		}
	}

	// Alguno de los dos arrays no terminó, terminá de insertar los elementos
	if left_index < left.len() {
        result[result_index..].copy_from_slice(&left[left_index..]);
    }
    if right_index < right.len() {
        result[result_index..].copy_from_slice(&right[right_index..]);
    }

	result
}