/// Pergunta 1:
/// Qual é o nome da variável de ambiente que você deve definir para `1` para ver o backtrace ao ocorrer um panic?
///
/// a) RUST_PANIC
/// b) RUST_BACKTRACE
/// c) PANIC_BACKTRACE
/// d) BACKTRACE_RUST
///
/// Resposta correta: b) RUST_BACKTRACE

// Exemplo de uso:
//
// Para visualizar o backtrace completo de um panic, execute no terminal:
// RUST_BACKTRACE=1 cargo run


--------------------


/// Pergunta 2:
Qual das seguintes opções NÃO é um bom motivo para usar o pânico?
///
/// a) O programa está prestes a realizar uma operação perigosa.
/// b) O programa deve parar de ser executado o mais rápido possível.
/// c) O programa atingiu um estado de erro que deve ser comunicado à função que o chamou.
/// d) O programa atingiu um estado de erro irrecuperável.
///

resposta: O programa atingiu um estado de erro que deve ser comunicado a uma função chamadora

Contexto: Não se deve usar panic para comunicar falhas dentro do programa. A suposição padrão é que funções chamadoras não tentarão capturar panics.