// quizes
// quiz-after-7.4.2

/*
Question 1
What is the keyword you use at the start of an absolute path to an item in the current crate?

You answered:
✓ Correct  crate

Context: For example, an absolute path to item b in module a would be crate::a::b.

*/

// =====================

// Pergunta:
// Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
//
// pub mod foo {
//     fn a() { println!("a"); }
//     mod bar {
//         pub fn b() { println!("b"); }
//     }
// }
//
// fn main() {
//     foo::bar::b();
// }
//
// Resposta:
// This program does not compile.
//
// Contexto:
// b is not accessible to main because the module bar is not marked as pub.

// =====================

// Pergunta:
// Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
//
// pub mod foo {
//     pub mod bar {
//         pub fn b() { println!("b"); }
//     }
//     pub fn a() { bar::b(); }
// }
//
// fn main() {
//     foo::a();
// }
//
// Resposta:
// This program does compile.
// The output of this program will be:
// b
//
// Contexto:
// It is valid within foo to use a relative path to refer to items within bar.