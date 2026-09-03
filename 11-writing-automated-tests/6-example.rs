/*
  - how to write tests
    - Using Result<T, E> in Tests
*/

/// example 1: result inves de panic

#[test]
fn it_works() -> Result<(), String> {
    let result = add(2, 2);

    if result == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}

// exemplos ia
// ? → se der Err, o teste FALHA
#[test]
fn teste_sucesso() -> Result<(), String> {
    let resultado = dividir(10, 2)?; // se Err, teste falha
    assert_eq!(resultado, 5);
    Ok(())
}

// is_err → se der Err, o teste PASSA
#[test]
fn teste_erro_esperado() {
    let resultado = dividir(10, 0);
    assert!(resultado.is_err()); // Err é o que queremos
}
