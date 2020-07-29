// Ahora que tenemos una base, es necesario generar un número aleatorio.
// Para eso, primero debemos modificar el archivo Cargo.toml
use std::io;

fn main() {
  println!("Guess the number!");
  println!("Please input your guess:");
  let mut guess = String::new();

  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line...");

  println!("You guessed: {}", guess);
}