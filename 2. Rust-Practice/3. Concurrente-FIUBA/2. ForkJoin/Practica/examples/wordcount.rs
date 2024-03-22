// Este archivo cuenta cuantas veces aparece cada palabra en un texto
// Este tipo de programa se puede ver acelerado mediante programación concurrente
// en particular utilizando un modelo MAP REDUCE.
// Se utilizan pares (k, v) donde, en este caso, cada clave será una palabra y 
// el valor inicial será 1 y simplemente lo usaremos para contabilizar.
// Luego de este mapeo inicial, se pasa a una etapa de MERGE & SORT, en la que se 
// agrupan los pares con misma clave, es decir, con la misma palabra.
// En la etapa REDUCE es cuando finalmente se suman todos los pares de misma clave
// obteniendo así la cantidad de veces que se obtiene cada palabra.

// Como el procesamiento de cada palabra es independiente de otra, esto se puede
// hacer en paralelo.

// EN ESTE ARCHIVO NO SE USA CONCURRENCIA.

use std::time::{Instant, Duration};
use std::fs::{File, read_dir};
use std::io::{BufRead, BufReader};
use std::{env, thread};
use std::collections::HashMap;

fn main() {
	// To measure the running time of the map-reduce
	let start = Instant::now();

	// Obtengo un iterador sobre el directorio con los datos.
	let result = read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/data")).unwrap()
		// Map es lazy. Retorna un iterador que va procesando a medida que itera.
		// En este caso se recorren los elementos de la carpeta data y se retorna el path
		.map(|d| d.unwrap().path())
		// flat map viene a resolver el siguiente problema: map retorna un iterador base
		// y en el caso de abajo estamos retornando otro iterador. Con map nos 
		// quedaría un iterador de iteradores de línea.
		// Flat map "aplana" esta estructura y nos retorna un solo iterador => estás
		// tratando todas las líneas de archivos como si fueran una sola secuencia de líneas
		.flat_map(|path| {
			let file = File::open(path).unwrap();
			let reader = BufReader::new(file);
			reader.lines() // Retorna un iterador, para eso flat map
		})
		// Ahora es la parte en que mapeamos cada palabra a formato k,v.
		// Este map retorna un iterador donde cada elemento es un Hashmap
		.map(|l| {
			let line: String = l.unwrap();
			let words = line.split(' ');
			// Simulamos un retraso
			thread::sleep(Duration::from_millis(100));

			let mut counts: HashMap<String, i32> = HashMap::new();
			words.for_each(|w| 
				// Usa * para obetener una referencia mutable al valor en la entrada
				// Como es el primero, siempre va a insertar un 0 y después le suma uno
				// Supongo que inserta un 0 por cuestión de formalidad.
				*counts.entry(w.to_string()).or_insert(0) += 1
			);
			counts // TODO: no sería más eficiente retornar una tupla?
		})
		// Fold siempre recibe un accumulator que va a tener el resultado final. 
		// Es como un reduce. El primer argumento es el acumulador.
		.fold(HashMap::new(), |mut acc, word| {
			word.iter().for_each(|(k, v)|
				*acc.entry(k.clone()).or_insert(0) += v
			);
			acc
		});
	println!("{:?}", start.elapsed());
	
	// Con retraso simulado: 5.23s
	// Sin retraso simulado: 3.15s

	// println!("{:?}", result);
}