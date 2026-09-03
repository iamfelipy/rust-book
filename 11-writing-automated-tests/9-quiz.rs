// quiz 1

Ao executar cargo test sem configuração adicional, qual das seguintes ações pode não funcionar corretamente se for feita por vários testes?

Você respondeu:
Gravar texto em um único arquivo

Contexto: Os testes são executados em paralelo por padrão, portanto ações que não são seguras para threads (como gravar em um único arquivo) podem causar uma condição de corrida.


// quiz 2

Considere um programa com o seguinte teste de unidade:

#[test]
fn test_the_logger() { /* ... */ }
#[test]
fn test_the_database() { /* ... */ }
#[test]
fn test_logger_and_database() { /* ... */ }
Qual é a menor string que você pode passar para cargo test <a_string> tal que apenas test_the_logger e test_the_database sejam executados?

Você respondeu:
th
A resposta correta é:
h
Contexto: A menor substring que não está contida em test_logger_and_database mas está contida em test_the_logger e test_the_database é "h" (o caráter do meio de "the").