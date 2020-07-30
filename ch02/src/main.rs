use std::io;
use rand::Rng;
// A diferencia de la 1era línea, en donde indicamos un preludio, aquí estamos indicando el uso
// de algo llamado 'traits' en Rust, de lo cual se hablará en el capítulo 10. Para resumirlo,
// con esto traemos al "scope" actual los métodos que implementan los generadores de números
// aleatorios.

fn main() {
  println!("Guess the number!");
  println!("Please input your guess:");
  let mut guess = String::new();
  let num = rand::thread_rng().gen_range(1, 101);
  // La función thread_rng nos da un generador de números aleatorios, el cual estará disponible
  // para el hilo de ejecución actual y que es "sembrado" por el sistema operativo. Por otro
  // lado, la función gen_range, toma eso que nos dió la función thread_rng y genera un número
  // aleatorio en el rango entre sus 2 argumentos. El 1er argumento es inclusivo, pero el 2do
  // es exclusivo.
  println!("Secret number: {}", num);

  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line...");

  println!("You guessed: {}", guess);
}