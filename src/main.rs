extern crate rand;


use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Adivina el numero");
    
    let secret_number =  rand::thread_rng().gen_range(1,101);
    println!("El numero secreto es: {}",secret_number);
    
    println!("Ingresa tu numero....>");
    
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
      .expect("Fallo al leer linea");
      
    println!("Tu numero: {}",guess);
    
    match guess.cmp(&secret_number){
      Ordering::Less => println!("Mas pequeño"),
      Ordering::Greater => println!("Mas Grande",;
      Ordering::Equal => println!("Ganaste"),
    }
}
