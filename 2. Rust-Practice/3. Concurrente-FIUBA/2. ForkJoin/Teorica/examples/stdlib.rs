use std::thread;
use std::thread::JoinHandle;

// Cada línea va a ser un data segment procesado por un thread
const DATA: &str= "86967897737416471853297327050364959
                   11861322575564723963297542624962850
                   70856234701860851907960690014725639
                   38397966707106094172783238747669219
                   52380795257888236525459303330302837
                   58495327135744041048897885734297812
                   69920216438980873548808413720956532
                   16278424637452589860345374828574668";

fn main() {
    // Vector con los join handles de los vectores creados
    let mut children:Vec<JoinHandle<u32>> = vec![];
    // chunked_data es un iterador sobre el texto separado por espacio
    let chunked_data = DATA.split_whitespace();

    for (i, data_segment) in chunked_data.enumerate() {
        println!("segmento numero: {} es: \"{}\"", i, data_segment);

        // El move es porque estás cediendo el data_segment al thread
        children.push(thread::spawn(move || -> u32 {
            // Cada data_segment es un str slice. Convierto cada char del segmento a u32 y los sumo
            let result = data_segment
                         .chars()
                         .map(|c| c.to_digit(10).expect("Debe ser un digito"))
                         .sum();
            
            println!("Segmento procesado: {}, resultado={}", i, result);

            result
        }));
    } 

    // Hago join de los threads y muestro la suma final por pantalla
    let mut intermediate_sums: Vec<u32> = vec![];
    for child in children {
        let intermediate_sum = child.join().unwrap();
        intermediate_sums.push(intermediate_sum);
    }

    let final_result = intermediate_sums.iter().sum::<u32>();

    println!("Suma final: {}", final_result);
}   