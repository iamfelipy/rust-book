/*
  10.3 - Validating References with Lifetimes
*/

/*
  Preventing Dangling References with Lifetimes
*/

// esse codigo nao compila
// Uma tentativa de usar uma referência cujo valor saiu do escopo.
fn main() {
  let r;

  {
      let x = 5;
      // &x o tempo de vida de &x é só ate o fechamento das {}
      // depois é desalocado
      r = &x;
  }

  println!("r: {}", r);
}

// erro


$ cargo run
 Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0597]: `x` does not live long enough
--> src/main.rs:6:13
|
5 |         let x = 5;
|             - binding `x` declared here
6 |         r = &x;
|             ^^ borrowed value does not live long enough
7 |     }
|     - `x` dropped here while still borrowed
8 |
9 |     println!("r: {r}");
|                  --- borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error

/*
  10.3 - Validating References with Lifetimes
*/

/*
  The Borrow Checker Ensures Data Outlives Its References
*/

//exemplo 1: Anotações dos tempos de vida de r e x, denominados 'a e 'b, respectivamente

// esse codigo nao compila
fn main() {
  let r;                // ---------+-- 'a
                        //          |
  {                     //          |
      let x = 5;        // -+-- 'b  |
      r = &x;           //  |       |
  }                     // -+       |
                        //          |
  println!("r: {r}");   //          |
}                         // ---------+

//exemplo 2: correção

// codigo que compila
// corrigi a referencia morta
// Em suma: uma referência é válida somente se o dado referenciado viver pelo menos enquanto a referência existir; o compilador compara lifetimes para garantir isso. 
fn main() {
  let x = 5;            // ----------+-- 'b
                        //           |
  let r = &x;           // --+-- 'a  |
                        //   |       |
  println!("r: {r}");   //   |       |
                        // --+       |
}                         // ----------+