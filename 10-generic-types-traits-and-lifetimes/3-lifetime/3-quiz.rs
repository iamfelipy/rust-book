// Pergunta 1
Que tipo de erro de programação um tempo de vida (lifetime) deve prevenir?

Você respondeu:
Usar uma referência a um objeto depois que sua memória foi liberada

Contexto: Os lifetimes ajudam a identificar por quanto tempo um objeto está "vivo" e se referências a esse objeto sobrevivem ao próprio objeto.

// Pergunta 2
Determine se o programa será aprovado pelo compilador. Se for aprovado, escreva a saída esperada do programa caso ele fosse executado.

fn shortest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
  if x.len() < y.len() {
    x
  } else {
    y
  }
}
fn main() {
  println!("{}", shortest("hello", "rust"));
}

resposta:
This program does not compile.

por que?
Se a assinatura de tipo diz que a função deve retornar uma referência com tempo de vida 'a, então seria inválido retornar uma referência com um tempo de vida diferente 'b, isto é, y aqui.