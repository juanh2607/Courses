// Este archivo implementa el wordcount usando concurrencia para acelerar el procesamiento

use std::collections::HashMap;
use std::{env, thread};
use std::fs::{File, read_dir};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::time::{Instant, Duration};

use rayon::prelude::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};

fn main() {
	let start = Instant::now();

	let result = read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/data")).unwrap()
		.map(|dentry| dentry.unwrap().path())
		// Convierto el iterador en una colección para procesarla en paralelo
		.collect::<Vec<PathBuf>>()
		.par_iter()
		// Esto de abajo es lo mismo que teníamos en wordcount salvo las diferencias 
		// marcadas con comentarios
		.flat_map(|path| {
			let file = File::open(path).unwrap();
			let reader = BufReader::new(file);
			// Convierto el iterador que devuelve lines() en un iterador en paralelo
			reader.lines().par_bridge()
		})
		.map(|l| {
			let line = l.unwrap();
			let words = line.split(' ');
			// Simulamos un retraso
			//thread::sleep(Duration::from_millis(100));

			let mut counts: HashMap<String, i32> = HashMap::new();
			words.for_each(|w| 
				*counts.entry(w.to_string()).or_insert(0) += 1
			);
			counts // TODO: no sería más eficiente retornar una tupla?
		})
		// rayon usa reduce en vez de fold
		.reduce(|| HashMap::new(), |mut acc, word| {
			word.iter().for_each(|(k, v)|
				*acc.entry(k.clone()).or_insert(0) += v
			);
			acc
		});

	println!("{:?}", start.elapsed());
	
	// Con retraso simulado: 0.707s
	// Sin retraso simulado: 0.004s
}
