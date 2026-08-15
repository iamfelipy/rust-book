// quiz 1

If a reference has a lifetime 'static, then this means:

resposta:
Os dados sob a referência nunca são desalocados

por que:
'static significa "vivo durante todo o programa", e portanto os dados sob uma referência estática nunca devem ser desalocados. Embora na prática referências 'static geralmente não sejam mutáveis, o significado do tempo de vida 'static não tem relação essencial com mutabilidade.

//quiz 2

Considere a seguinte assinatura de função sem anotações.

struct Foo<'a> {
  bar: &'a i32
}
fn baz(f: Foo) -> &i32 { /* ... */ }

O Rust aceitará essa assinatura de função? Em caso afirmativo, quais lifetimes ele inferirá?

resposta:
fn baz<'a>(f: Foo<'a>) -> &'a i32

por que:
A struct recebe um único parâmetro de lifetime, e a saída tem um único lifetime, então o Rust assume que são o mesmo.

anotações pessoais:
fn baz<'a>(f: &'a Foo<'a>) -> &'a i32


//quiz 3
Considere a seguinte assinatura de função sem anotações.

struct Foo<'a> {
  bar: &'a i32
}
// Foo changed to &Foo
fn baz(f: &Foo) -> &i32 { /* ... */ }

O Rust aceitará essa assinatura de função? Em caso afirmativo, quais lifetimes ele inferirá?

resposta:
Rust will reject this function signature

por que:
O Rust não compilará este programa, porque é ambíguo se o tempo de vida da saída está ligado ao tempo de vida de &Foo ou à referência Foo.bar.

possivel resposta:
fn baz<'a>(f: &'a Foo<'a>) -> &'a i32



// reflexão sobre o 3
O lifetime serve para garantir que referências não apontem para dados que já foram desalocados, evitando uso de memória inválida (dangling references). No contexto, sem o lifetime, poderíamos ter:

fn main() {
    let r;
    {
        let x = 5;
        let foo = Foo { bar: &x };
        r = baz(&foo); // r referencia x
    } // x é desalocado aqui
    println!("{}", r); // uso de referência inválida
}
O compilador impede isso exigindo que os lifetimes estejam corretos, garantindo segurança da memória.

// reflexão do anterior
// o lifetime retornado por baz será limitado ao lifetime de foo, pois foo pode viver menos tempo que x. Assim, a referência retornada (r) só será válida enquanto foo existir.

fn main() {
  let x = 5;
  let r;
  {
      let foo = Foo { bar: &x };
      r = baz(&foo); // foo vive menos que x
  }
  println!("{}", r);
}