/*
  quiz 1
*/

Pergunta 1
Imagine usar uma função de terceiros cuja implementação você não conhece, mas cuja assinatura de tipo é esta:

fn mystery<T>(x: T) -> T {
  // ????
}

Então você chama mystery assim:

let y = mystery(3);

Assumindo que mystery não usa código unsafe, então o valor de y deve ser:

resposta:
3

Contexto: A única função possível (sem código unsafe) que tem a assinatura T -> T é a função identidade:

fn mystery<T>(x: T) -> T {
  x
}

A função poderia, é claro, entrar em pânico (panic) ou imprimir, mas o  valor retornado só pode ser a entrada. mystery não sabe qual é o tipo T, então não há como mystery gerar ou mutar um valor de T. Veja Theorems for free! para mais exemplos dessa ideia.

3 realmente é a resposta correta!

função identidade:
É a função que simplesmente retorna o que recebeu, sem modificar nada.
O nome vem da matemática — "identidade" porque o valor não muda, permanece idêntico ao que entrou.

Um teorema: é uma afirmação que pode ser provada como verdadeira a partir de axiomas e regras lógicas.

Teorema da Parametricidade
Só de olhar a assinatura de uma função genérica, você pode deduzir o que ela faz — sem ver o código.

