// Iterating Over the Values in a Vector

// referencia imutavel
let v = vec![100, 32, 57];
for i in &v {
  // dereferencia * automatica
  println!("{i}");
}

// referencia mutavel
let mut v = vec![100, 32, 57];
for i in &mut v {
  // dereferencia *
  *i += 50;
}