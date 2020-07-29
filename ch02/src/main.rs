// En este capítulo, creamos un juego de adivinanza, en donde el usuario deberá ingresar
// un número y si coincide con el número secreto, entonces gana el juego!
// Para ello, comenzamos con algo de código base como este:
use std::io; // Preludio, ver abajo ↓
// Por defecto, Rust solo incluye los tipos necesarios en el alcance de cada programa.
// Si algún tipo no está disponible en el preludio por defecto de Rust (o de alguna
// librería externa), entonces debe "importarse" explícitamente con el keyword use.
// En este caso, con io podemos hacer operaciones de entrada/salida.

fn main() {
  println!("Guess the number!");
  println!("Please input your guess:");
  let mut guess = String::new();
  // `→ En Rust, las variables se crean con el keyword let, en este caso, creamos una llama
  // "guess" para almacenar la entrada del usuario en ella. Más adelante se hablará de esto.
  // Por ahora, la variable es de tipo String como se puede ver y solo contiene un string
  // vacío, además, se puede apreciar mucho la sintáxis al estilo C++ y el "método estático"
  // (función asociada en términos de Rust) llamado "new", que también se puede ver comunmente
  // en otros tipos de Rust (y librerías).

  io::stdin()
  // Con la primera línea (use std::io;) hemos incluído funcionalidad para entrada/salida y
  // aquí estamos haciendo uso de ella. Sin la 1era línea, podríamos haber llamado esta
  // función con std::io::stdin(). Dicha función, retorna una instancia de std::io::Stdin, el
  // cual es básicamente una representación de la entrada estándar de la terminal.
    .read_line(&mut guess)
  // 'read_line' es un método que al ser llamado, lee lo que es ingresado en la terminal y
  // lo retorna como un string. Éste método recibe como argumento una referencia y en este caso
  // estamos pasando nuestra variable guess. Las referencias son un aspecto complejo en Rust
  // bastante interesante en la que diferentes partes del código pueden acceder a datos sin
  // necesidad de copiarlos a la memoria varias veces, pero por ahora, no se hablará de eso.
    .expect("Failed to read line...");
  // 'read_line' además de retornar lo que el usuario ingresa, también retorna un io::Result,
  // un tipo de dato que, de hecho, se puede ver muchas veces en otras partes de la biblioteca
  // estándar. Normalmente, los tipos Result, son enumeraciones (también llamadas enums, de lo
  // cual se habla en el capítulo 6). Result solo tiene 2 variantes: Ok y Err. Para manejar
  // posibles errores, podemos usar el método 'expect' luego de read_line, que, si el valor
  // retornado resulta ser Err, el programa finaliza inmediatamente e imprime el texto que es
  // pasado como argumento. Cabe destacar que, si no se usa expect, el programa compilará de
  // igual manera, sin embargo, recibirá una advertencia por parte del compilador y su programa
  // podría ser inseguro

  println!("You guessed: {}.", guess);
  // Finalmente, podemos imprimir el valor ingresado usando marcadores de posición en println.
  // Podemos también hacer cosas como:
  //    println!("{} * {} = {}", 2, 4, 8)
  // Y tendríamos una salida como:
  //    2 * 4 = 8
}