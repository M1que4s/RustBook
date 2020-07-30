// Comparando la entrada del usuario con el número secreto.
// NOTA: Este código no compilará.
use rand::Rng;
use std::cmp::Ordering;
// Ordering es otro enum al igual que Result, pero posee 3 variantes (las cuales se pueden
// ver más abajo en el bloque 'match'). Forma parte de la librería estándar (en std::cmp)
// y se usa principalmente para comparar "cosas".
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

  println!("You guessed: {}", guess);

  // El método 'cmp' compara 2 dos valores y retorna una variante de Ordering. Se puede usar
  // en cualquier cosa que sea comparable (aunque desde mi punto de vista creo que aplica
  // principalmente con números).
  match guess.cmp(&num) {
    // Según puedo ver, una expresión 'match' sería básicamente como 'switch' en C, pero
    // difiere en 2 cosas (basándome en lo que se ha hablado hasta ahora):
    //    1: en C, el keyword 'switch' (hasta donde sé), solo trabaja con números, cosa que
    //       no parece ser el caso en Rust.
    //    2: no hay keywords adicionales como 'case' dentro de match, en lugar de eso, se usa
    //       algo similar a "functions arrows"/"lambdas" (JS y otros) en donde lo que vendría
    //       siendo el argumento, match lo usa como el valor a coincidir (ya que según el
    //       libro, match trabaja con patrones) y en caso de que lo haga, ejecuta el código
    //       que sigue a "=>".
    // Dicho eso, lo que hará el código de abajo es bastante obvio.
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
  }
}