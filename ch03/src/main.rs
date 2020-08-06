// Algo que se pudo ver en el capítulo 2, es una característica de las variables llamada
// "shadowing", que es básicamente re-declarar y asignar una variable con el mismo nombre
// de otra usando su valor anterior. Por ejemplo:

fn main() {
  let x = 5;
  let x = x + 1;
  let x = x * 2;
  println!("Value of x: {}", x);
  // Esto es muy diferente de usar mut, ya que si no usamos el keyword let, tendremos un error
  // Otra diferencia es que literalmente estamos otra variable de nuevo, por lo tanto, podemos
  // cambiar su tipo y mantenemos el mismo nombre. Por ejemplo:
  let spaces = "    "; // => String
  let spaces = spaces.len(); // => Number
  println!("spaces = {}", spaces);
  // Lo anterior no es válido con mut, porque primero se declara la variable como string y
  // luego estaríamos intentado asignarle un valor numérico:
  //    let mut spaces = "    ";
  //    spaces = spaces.len();
}