use rand::Rng;
use std::cmp::Ordering;
use std::io;

// Usando bucles para permitir múltiples entradas. En este punto, el juego está casi listo,
// solo faltarían unos pequeños detalles.
fn main() {
  println!("Guess the number!");
  let snum = rand::thread_rng().gen_range(1, 101);

  // El keyword 'loop' genera un bucle infinito. Sin embargo, eso de hecho es un problema
  // aquí, ya que incluso si el usuario da con la respuesta, el programa seguirá ejecutándose
  // por siempre. Por parte del usuario, hay 2 opciones posibles:
  //    1: usar la combinación de teclas Ctrl + C
  //    2: ingresar una entrada diferente de un número
  // Sin embargo, en ambos casos es... Innecesario, ya que eso se puede solucionar desde
  // este mismo código y para eso está el keyword 'break' al final del bloque match (ver
  // más abajo ↓).
  loop {
    let mut guess = String::new();
    // Si estoy en lo correcto, en el libro no se explica a detalle por qué ésta línea debe
    // permanecer dentro del bucle, pero básicamente la razón es porque cuando convertimos
    // la entrada del usuario a un número, literalmente estamos redeclarando esta variable
    // pero con un tipo diferente y eso generaría un error con io::stdin cuando tenga que
    // leer nuevamente la entrada del usuario. Otra posible solución sería usar 2 variables
    // con nombres diferentes, en ese caso, entonces una o ambas podrían estar fuera del
    // bucles.
    println!("Please input your guess:");

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line...");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Invalid input, please type a number!");
        continue;
      }
    };
    // Al usar una expresión match cuando convertimos la entrada del usuario, podemos hacer
    // que en lugar de finalizar el programa si la entrada es inválida, simplemente lo ignore
    // vuelva a preguntar al usuario por un valor. El método parse también retorna un tipo
    // Result, así que aquí se hace algo similar a lo que hicimos con io::stdin(), la única
    // diferencia es que estamos usando una expresión match.

    println!("You guessed: {}", guess);

    match guess.cmp(&snum) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
        // Si el usuario adivina el número, entonces break hará que el bucle termine
      }
    }
  }
}