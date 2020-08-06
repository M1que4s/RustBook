// Al igual que en el capítulo 2, podemos cambiar este comportamiento usando el keyword mut

fn main() {
  let mut x = 5;
  println!("Value = {}", x);
  x = 6;
  println!("Value = {}", x);
}