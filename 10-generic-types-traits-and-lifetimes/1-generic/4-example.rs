/*
  10.1 - Generic Data Types
*/

/*
  In Enum Definitions
  Em Definições de Enum
*/

// examplo 1: Option
// o Option<T> enum é genérico em relação ao tipo T

enum Option<T> {
  Some(T),
  None,
}


// examplo 2: Result

enum Result<T, E> {
  Ok(T),
  Err(E),
}