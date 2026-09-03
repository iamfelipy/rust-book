/*
- Test Organization
    - integration tests
        - The tests Directory
*/

/// commands

- cargo test
    - executa tudo unit, integration
- cargo test --test integration_test
    - executa especifico
- como é a saida?
    - seção 1: teste unitario
        - testes executados
        - resumo
    - seção 2: teste de integração
        - cada teste de integração tem sua propria seção, pois é um crate individual

/// examples

// project adder
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs

//  src/lib.rs

// tests/integration_test.rs
use adder::add_two;

#[test]
fn it_adds_two() {
    let result = add_two(2);
    assert_eq!(result, 4);
}



/*
- Test Organization
    - Integration Tests
        - Submodules in Integration Tests
*/

/// example

├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs

// tests/integration_test.rs
use adder::add_two;

mod common;

#[test]
fn it_adds_two() {
    common::setup();

    let result = add_two(2);
    assert_eq!(result, 4);
}