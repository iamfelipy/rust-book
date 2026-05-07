#### aprendendo rust atraves do livro:
https://rust-book.cs.brown.edu/experiment-intro.html

----

##### Rodando o projeto com Docker Compose e acessando o container

O arquivo `docker-compose.yml` já existe na raiz do projeto.

Para iniciar o projeto utilizando Docker Compose, execute:

```sh
docker compose up
```

Para acessar o terminal do container em execução:

```sh
docker exec -it rust-dev bash
```

Esses comandos irão iniciar o serviço e permitir que você entre no container para executar comandos no projeto rust.



##### Criando e executando um novo projeto Rust

```sh
cargo new <nome_do_projeto>
cd <nome_do_projeto>
cargo run
```

----
#### capitulo 5-using-structs-to-structure-related-data

introdução:
- o que é uma struct?
  - Uma struct, ou estrutura, é um tipo de dado personalizado que permite agrupar e nomear múltiplos valores relacionados que formam um conjunto significativo. Se você está familiarizado com uma linguagem orientada a objetos, uma struct é como os atributos de dados de um objeto.
  - structs fornecem uma forma de agrupar campos e dados relacionados, como um Retângulo com sua largura e altura
- o que vou aprender no capitulo?
  - comparação entre struct e tupla, e quando struct é uma forma melhor de comparar dados
  - como definir e instanciar structs
  - como definir funções associadas, metodo
  - Structs e enums (discutidos no Capítulo 6) são os blocos de construção para a criação de novos tipos no domínio do seu programa, permitindo aproveitar ao máximo a verificação de tipos em tempo de compilação do Rust.

conclusao:
- As estruturas permitem criar tipos personalizados que sejam significativos para o seu domínio. Ao usar estruturas, você pode manter dados associados conectados entre si e nomear cada parte para tornar seu código mais claro.
- Em impl blocos, você pode definir funções associadas ao seu tipo, e os métodos são um tipo de função associada que permite especificar o comportamento das instâncias de suas estruturas.

Mas as structs não são a única maneira de criar tipos personalizados: vamos recorrer ao recurso de enumeração do Rust para adicionar mais uma ferramenta à sua caixa de ferramentas.

----
#### capitulo 6-enums-and-pattern-matching

introdução:
- Enumeração: listar itens um por um.
- Listar: colocar itens em sequência, um após o outro, geralmente organizados.
- enum
  - enums permitem declarar que um valor é um dentre um conjunto possível de valores.
  - Por exemplo, podemos querer dizer que Retângulo é uma das formas possíveis que também incluem Círculo e Triângulo. 
  - enumerar variantes possiveis, dai vem o nome enumeração
  - um valor enum só pode ser uma de suas variantes
  - representar explicitamente todas as alternativas possíveis de um valor
  - é um tipo de dado que define, de forma explícita e fechada, um conjunto limitado de variantes possíveis

- Neste capítulo, vamos ver enumerações, também chamadas de enums.
- Enums permitem definir um tipo enumerando suas possíveis variantes.
- Primeiro definiremos e usaremos um enum para mostrar como um enum pode codificar significado junto com dados.
- Em seguida, exploraremos um enum particularmente útil, chamado Option, que expressa que um valor pode ser algo ou nada.
- Depois veremos como a correspondência de padrões na expressão match torna fácil executar código diferente para valores diferentes de um enum.
- Por fim, cobriremos como a construção if let é outra forma conveniente e conciso disponível para lidar com enums no seu código.

- resumo
  - enums criam tipos personalizados
  - o Option é um enum, vem por padrão e pode expressar a ideia de null
  - match e if let permitem extrair o valor de enum que contem dados
  - criar tipos personalizados para um api(função) garante segurança de tipos, o compilador garante que a função receba o tipo espearado
  - Seus programas em Rust podem expressar conceitos em seu domínio