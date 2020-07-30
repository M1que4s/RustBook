// Capitulo 3: Variables y mutabilidad.
// Por defecto, las variables son inmutables en Rust, una vez que hay un valor asignado a
// un nombre, éste no podrá cambiarse. Rust trabaja de esta forma para darnos un "empujón"
// a escribir mejor código. Sin embargo, las variables pueden volverse mutables si así lo
// queremos. Se presenta este primer código en el capítulo, el cual muestra como el
// compilador de Rust nos retorna un error debido a que no podemos cambiar el valor de "x"

fn main() {
  let x = 5;
  println!("Value = {}", x);
  x = 6;
  println!("Value = {}", x);
}