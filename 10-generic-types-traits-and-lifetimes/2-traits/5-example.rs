/*
  - 10.2 - traits - defining shared behavior
  - Características: Definindo o Comportamento Compartilhado
*/

/*
  Traits as Parameters
  Características como parâmetros
*/

/*
  introduction
*/

//example:

pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}

/*
  Trait Bound Syntax
*/

// impl Trait é um sugar sintax para trait bound
// exemplo verboso para o caso anterior
pub fn notify<T: Summary>(item: &T) {
  println!("Breaking news! {}", item.summarize());
}

// implt trait vs trait bound sintax
// nesse caso podem ser de tipos diferentes, mas precisa implementar Summary
pub fn notify(item1: &impl Summary, item2: &impl Summary) {

// nesse caso tem que ser do mesmo tipo e implementar Summary
pub fn notify<T: Summary>(item1: &T, item2: &T) {


/*
  Specifying Multiple Trait Bounds with the + Syntax
  Especificando múltiplos limites de características com a +sintaxe
*/

// impl trait
pub fn notify(item: &(impl Summary + Display)) {
// bound trait syntax
pub fn notify<T: Summary + Display>(item: &T) {

/*
  Clearer Trait Bounds with where Clauses
  Limites de características mais claros com where cláusulas
*/

// com trait bound, fica muito verboso
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// com where clause
// A assinatura desta função é menos confusa: o nome da função, a lista de parâmetros e o tipo de retorno estão próximos, semelhante a uma função sem muitos limites de trait.
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{}
  