// En Rust también disponemos de constantes, las cuales son muy diferentes de las variables:
//    Las constantes no se pueden hacer mutables con el keyword mut como se hace con las
//    variables. Se debe indicar siempre el tipo de dato. Y su valor debe ser constante,
//    es decir, no puede ser algo que se calcúle en tiempo de ejecución o llamadas a una
//    función, etc. Adicionalmente, se recomienda usar nombres en mayúsculas para éstas.

fn main() {
  const MAX_POINTS: u32 = 100_000;
  // Rust permite usar "underscores" en valores literales numéricos para una mejor legibilidad
}