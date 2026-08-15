/*
  10.3 - Validating References with Lifetimes
*/

/*
  The Static Lifetime
*/

// existe durante toda a duraçao do programa
// armazenado diretamente no binario do programa
let s: &'static str = "I have a static lifetime.";