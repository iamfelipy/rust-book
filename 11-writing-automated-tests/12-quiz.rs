Pergunta 1
Qual das seguintes opções NÃO é uma boa razão para envolver testes unitários em #[cfg(test)] mod tests { ... }?

Você respondeu:
Isso dá aos seus testes acesso a funções privadas

Contexto: Todos os testes unitários em um determinado arquivo têm acesso às funções privadas desse arquivo, independentemente de estarem em um mod tests ou não.