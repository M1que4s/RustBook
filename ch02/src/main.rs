use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  println!("Guess the number!");
  println!("Please input your guess:");
  let mut guess = String::new();
  let num = rand::thread_rng().gen_range(1, 101);

  println!("Secret number: {}", num);

  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line...");

  let guess: u32 = guess.trim().parse().expect("Invalid input!");
  // Con esto, convertimos la entrada del usuario (que por defecto es tipo String) a un
  // valor númerico válido que pueda ser comparado. Ńótese el 'u32' luego de 'guess', esto
  // es una notación de tipo. Al igual que otros lenguajes, Rust es un lenguaje tipado, por
  // lo que con esto nos estamos asegurando de guardar un valor numérico de 32 bits sin signo
  // (ya que el número secreto es entre 1 y 100). Adicional a eso, esto también le indica a la
  // función 'parse' que necesitamos convertir la entrada este tipo de dato, de no hacer esto,
  // tendríamos que escribir la línea anterior de la siguiente manera:
  //    let guess = guess.trim().parse::<u32>().expect("Invalid input!");
  // El uso del método 'trim' es para eliminar el carácter "\n" que es ingresado cuando el
  // usuario presiona enter para enviar su entrada. Finalmente, debido a que parse puede
  // generar errores (ya que si la entrada del usuario es algo como "A%·→", eso no se puede
  // convertir a un número claramente), se usa expect para evitar que el programa se rompa de
  // forma "extraña". Con esto, el proyecto ahora si compila correctamente.

  println!("You guessed: {}", guess);

  match guess.cmp(&num) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
  }
}