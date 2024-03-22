// Este archivo realiza el mergesort en paralelo. Solo cambian dos líneas de código:
// en main se llama a build_global y en mergesort se llama a rayon::join para el mergesort
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let data = [7, 3, 2, 16, 24, 4, 11, 9];

    // Usa build global porque estamos midiendo el tiempo => querémos ya tener 
    // todos los threads listos para medir correctamente el tiempo del mergesort
    // sin contar este tiempo de creación de threads.
    let _ = rayon::ThreadPoolBuilder::new().build_global();

    let start = Instant::now();
    let merged = mergesort(&data);
    println!("{:?}", start.elapsed());

    // Con retraso de 2 segundos tarda 8s
	// Sin retraso tarda 95 nanosegundos

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
	// Ahora usando paralelismo
    let (left, right) = 
        rayon::join(|| mergesort(left_data), || mergesort(right_data));

	merge(left, right)
}

// Es la misma función que en mergesort.rs
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