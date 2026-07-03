// Exemplo 1: Acessando um índice inválido em um array (causa panic!)
let v = vec![1, 2, 3];
println!("O quarto elemento é: {}", v[3]);

// Exemplo 2: Chamando panic! explicitamente
panic!("Este é um panic! explícito");

--------------------

// Para visualizar um backtrace detalhado ao ocorrer um panic!,
// execute seu programa com a variável de ambiente RUST_BACKTRACE ativada:
//
//     RUST_BACKTRACE=1 cargo run
// RUST_BACKTRACE=1 mostra apenas um backtrace resumido.
// RUST_BACKTRACE=full mostra o backtrace completo, com todas as funções da pilha, facilitando depuração detalhada.
//
// Isso mostrará a cadeia completa de chamadas que levou ao erro, facilitando a depuração.

--------------------

// Exemplo 3: Abortando o processo ao ativar panic=abort

// Para ver o efeito de panic=abort, adicione as seguintes configurações no arquivo Cargo.toml:
//
// [profile.dev]
// panic = "abort"
//
// [profile.release]
// panic = "abort"
//
// Com panic=abort, ao ocorrer um panic o processo será finalizado imediatamente, 
// sem executar o backtrace ou a liberação adequada de recursos.

// A linha abaixo ainda causa um panic, mas o comportamento será diferente.
panic!("Este panic abortará imediatamente o processo, sem mostrar backtrace!");


------
// exemplo de backtrace com panic ativado por codigo externo
// acessar indice invalido
fn main() {
  let v = vec![1, 2, 3];

  v[99];
}
// $ RUST_BACKTRACE=1 cargo run
// thread 'main' panicked at src/main.rs:4:6:
// index out of bounds: the len is 3 but the index is 99
// stack backtrace:
//    0: rust_begin_unwind
//              at /rustc/4d91ded48198da2e3341fedcd9d2cc0c46688/library/std/src/panicking.rs:595
//    1: core::panicking::panic_fmt
//              at /rustc/4d91ded48198da2e3341fedcd9d2cc0c46688/library/core/src/panicking.rs:67
//    2: core::panicking::panic_bounds_check
//              at /rustc/4d91ded48198da2e3341fedcd9d2cc0c46688/library/core/src/panicking.rs:137
//    3: <usize as core::slice::index::SliceIndex<[T]>>::index
//              at /file/home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/slice/index.rs:263
//    4: core::slice::<impl core::ops::index::Index<I> for [T]>::index
//              at /file/home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/slice/mod.rs:3154
//    5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
//              at /file/home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2727
//    6: main
//              at ./src/main.rs:4:6
//    7: core::ops::function::FnOnce::call_once
//              at /file/home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/ops/function.rs:250
// note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.