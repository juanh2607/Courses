use rayon::prelude::*;

struct Person {
  age: u32
}

fn main() {
  let v: Vec<Person> = vec![
    Person { age: 23 },
    Person { age: 19 },
    Person { age: 42 },
    Person { age: 17 },
    Person { age: 17 },
    Person { age: 31 },
    Person { age: 30 },
  ];

  // par_iter converts the vector into a parallel iterator

  // Cada uno de los métodos que no son un reduce, retornan su propio iterador
  // (ej: Iter, Filter, Map, etc)
  
  // Amount of Persons with age bigger than 30
  let age_over_30 = v.par_iter()
      .filter(|&x| x.age > 30)
      .count() as f32;

  // par_iter(): Este método crea un iterador paralelo sobre la referencia inmutable de v. 
  // Aquí, x es una referencia inmutable a cada Person en v

  // map(|x| x.age): Aquí, x es una referencia inmutable a Person, y x.age es una copia del valor de la edad, 
  // porque u32 implementa el trait Copy.

  // filter(|&x| x > 30): Aquí, x es una copia del valor de la edad, porque &x en la cláusula de captura 
  // desreferencia la referencia a la edad. &x está desreferenciando la referencia a u32, 
  // y x en x > 30 es el valor de u32 en sí, no una referencia.

  let sum_over_30 = v.par_iter()
      .map(|x| x.age)
      .filter(|&x| x > 30)
      .reduce(|| 0, |x, y| x + y);
   
  let alt_sum_30: u32 = v.par_iter()
      .map(|x| x.age)
      .filter(|&x| x > 30)
      .sum();

  let avg_over_30 = sum_over_30 as f32 / age_over_30;
  let alt_avg_over_30 = alt_sum_30 as f32 / age_over_30;

  // Simplemente mide que la diferencia sea muy muy chica. Deberían ser iguales pero
  // errores de redondeo
  assert!((avg_over_30 - alt_avg_over_30).abs() < std::f32::EPSILON);
  println!("El promedio de edad de los mayores de 30 es {}", avg_over_30);
} 