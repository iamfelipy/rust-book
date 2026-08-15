// quiz 1

Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.

trait MakeNoise {
  fn make_noise(&self) {
    println!("(silence)");
  }
}
struct Dog {}
struct Cat {}
impl MakeNoise for Dog {
  fn make_noise(&self) {
    println!("bark");
  }
}
impl MakeNoise for Cat {}
fn main() {
  let dog = Dog {};
  let cat = Cat {};
  dog.make_noise();
  cat.make_noise();
}

---
answer;
bark
(silence)

por que: 
A implementação de MakeNoise fornece uma implementação específica para Dog e usa a implementação padrão para Cat. Portanto, chamar make_noise em cada um imprime "bark" e (silêncio), respectivamente.

// quiz 2

A seguir estão afirmações sobre quais tipos de implementações de traits são permitidos em Rust. Selecione cada afirmação que for verdadeira.

resposta:
Você pode implementar um trait local para um tipo local
Você pode implementar um trait local para um tipo externo
Você pode implementar um trait externo para um tipo local

por que?
A "regra do órfão" exige que você não possa implementar um trait externo para um tipo externo, para garantir que o código não quebre caso dois crates forneçam implementações conflitantes.

Sim, você pegou a ideia! Na hora de juntar tudo, o compilador precisa que cada combinação trait+tipo tenha **uma única implementação**. Se dois crates definissem `impl Display for Vec<String>`, na hora de resolver qual usar, haveria conflito.

A regra do órfão resolve isso **antes** da compilação, na fase de checagem. Ela impede que o conflito sequer exista, em vez de tentar resolver depois.