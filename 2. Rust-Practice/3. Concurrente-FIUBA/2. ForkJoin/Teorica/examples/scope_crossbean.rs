// use crossbeam::thread;

fn main() {
  // Note: Since Rust 1.63, this function is soft-deprecated in favor of the 
  // more efficient std::thread::scope.

  // scope crea un nuevo ámbito para el lanzamiento de hilos.
  // Cualquier hilo lanzado dentro de este ámbito tiene garantizado terminar
  // ANTES de que termine la invocación a crossbeam::scope

  // En rust, el ámbito es el contexto en que una variable es válida o puede ser utilizada.
  // Este se limita al bloque que la contiene. Cuando el ámbito términa, se desecha
  // la variable.

  // s es una referencia al ámbito creado. Con s podés lanzar hilos dentro de este 
  // ámbito

  // Para que sirve?
  // La idea es poder lanzar los hilos dentro del mismo contexto en el que existen
  // variables de ámbito local a las que pueden hacer una referencia segura.
  // Estas variables son globales para los threads pero locales al ámbito s
  // Esto es algo que no se puede hacer de forma segura con los hilos estándar en Rust.
  // Con crossbeam::scope podés lanzar hilos que toman referencias a datos en el stack
  // y esta garantizado que los hilos no duran más que el ámbito en el que fueron 
  // creados, por lo que la referencia siempre va a ser válida dentro del thread 
  crossbeam::scope(|s| {

    // Acá podrías crear de forma segura una variable a la que acceden los threads
    // Esta variable va a existir siempre para los threads.

    let handle = s.spawn(|_| {
      println!("A child thread is running");
      42
    });

    let res = handle.join().unwrap();
    assert_eq!(res, 42);
  }).unwrap();
}