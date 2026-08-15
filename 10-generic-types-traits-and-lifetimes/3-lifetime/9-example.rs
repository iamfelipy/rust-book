/*
  10.3 - Validating References with Lifetimes
*/

/*
  Generic Type Parameters, Trait Bounds, and Lifetimes Together
*/

// example

use std::fmt::Display;

// Função que retorna a string mais longa, com um anúncio.
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}