// Methods for Iterating Over Strings

// .chars
for c in "Зд".chars() {
  println!("{c}");
}

resultado
З
д

// ,bytes
for b in "Зд".bytes() {
  println!("{b}");
}
resultado
208
151
208
180

// como obter grafemas?
  lib do crates.io