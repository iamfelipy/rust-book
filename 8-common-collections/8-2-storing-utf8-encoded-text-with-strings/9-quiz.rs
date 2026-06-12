// quiz 1

Qual das afirmações a seguir melhor explica por que Rust não permite indexação de strings?

resposta: Indexar strings é ambíguo porque strings representam várias granularidades de dados sequenciados

por que: Uma string em UTF-8 pode ser interpretada como uma sequência de bytes, caracteres ou clusters de grafemas. Nenhuma dessas interpretações é necessariamente o modo “padrão” de interpretar uma string, portanto uma operação de indexação padrão não faz sentido.
.chars permite iterar corretamente pelos caracteres Unicode, mas acessar por índice ainda é O(n), pois cada caractere pode ter tamanho variável em bytes. Portanto, não resolve o problema da indexação direta eficiente.

// quiz 2

Qual afirmação descreve melhor a diferença entre os tipos fatia de string &str e fatia de bytes &[u8]?

resposta: &str aponta para bytes que sempre podem ser interpretados como UTF-8, enquanto &[u8] pode ser qualquer sequência de bytes

por que: &str é uma promessa de que a sequência de bytes para a qual aponta será sempre UTF-8 válida. Portanto, um programador que deseja, por exemplo, imprimir um &str nunca precisa verificar se ele é válido, nem se preocupar em interpretar acidentalmente uma string inválida.