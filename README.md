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

---
#### capitulo 7-packages-crates-modules

- o que vamos ver no capitulo

  - Managing Growing Projects with Packages, Crates, and Modules

  - agrupar funcionalidades relacionadas e separar o código com recursos distintos
    - Funcionalidade
      - É o que o sistema faz para o usuário.
    - Recurso
      - É o meio técnico usado para implementar funcionalidades.
  
  - À medida que um projeto cresce, você deve organizar o código dividindo-o em vários módulos e, em seguida, em vários arquivos. 

  - pacote
    - Um pacote pode conter vários crates binários e, opcionalmente, um crate de biblioteca. Conforme um pacote cresce, você pode extrair partes para crates separados que se tornam dependências externas. 
    - Cargo.toml: estão instruções sobre como compilar, configurar dependências e definir metadados do projeto.
    - Construir significa transformar seu código-fonte em um executável/binário ou biblioteca pronta para uso.
    - usar workspace, para projetos muito grandes, compostos por um conjunto de pacotes inter-relacionados que evoluem juntos

  - escondendo detalhes e criando uma api
    - outros códigos podem chamar seu código por meio de sua interface pública sem precisar saber como a implementação funciona.
  
  - escopo: é o contexto no qual nomes (variáveis, funções, módulos, etc.) estão definidos e podem ser acessados no código. Ele determina onde um item é visível e utilizável, controlando o acesso e evitando conflitos de nomes.
    - Contexto é o conjunto de informações, ambiente ou situação em que algo está inserido, influenciando seu significado, uso ou compreensão.
    - É possível criar escopos e alterar quais nomes estão dentro ou fora do escopo.
    
  - Rust possui uma série de recursos que permitem gerenciar a organização do seu código, incluindo quais detalhes são expostos, quais detalhes são privados e quais nomes pertencem a cada escopo dos seus programas. 
    - Esses recursos, às vezes chamados coletivamente de sistema de módulos , incluem:
      - Packages: Um recurso do Cargo que permite criar, testar e compartilhar crates.
      - Crates: Uma árvore de módulos que produz uma biblioteca ou executável.
      - Modules and use: Permitem controlar a organização, o escopo e a privacidade dos caminhos.
      - Paths: Uma forma de nomear um item, como uma estrutura, função ou módulo.
  
  - Neste capítulo, abordaremos todos esses recursos, discutiremos como eles interagem e explicaremos como usá-los para gerenciar o escopo.

- 7.2 - Packages and Crates
  - crate
    - o que é um crate?
      - Um crate é a menor quantidade de código que o compilador Rust considera por vez.
      - se eu passar um arquivo de código-fonte para rustc ou cargo run isso é um crate
    - tipos de crate
      - crate binario
        - Crates binários são programas que você pode compilar para um executável que pode ser executado, como um programa de linha de comando ou um servidor.
        - tem função main
      - crate de biblioteca
        - não possui função main
        - não são compilados para um executavel
        - definem funcionalidades destinadas a serem compartilhadas com múltiplos projetos
    - crates podem conter modulos
      - modulos podem ser definidos em outros arquivos e são compilados com o crate
    - A raiz do crate é um arquivo fonte a partir do qual o compilador Rust inicia a execução e que constitui o módulo raiz do seu crate
  - package
    - o que é?
      - Um pacote é um conjunto de um ou mais crates que fornece um conjunto de funcionalidades.
    - o pacote contém um arquivo Cargo.toml que descreve como construir esses crates
    - exemplo:
      - o que é o cargo?
      - O Cargo é, na verdade, um pacote que contém o crate binário da ferramenta de linha de comando que você tem usado para construir seu código. O pacote Cargo também contém um crate de biblioteca do qual o crate binário depende.
    - regras:
      - Um pacote pode conter quantos crates binários você quiser, mas no máximo um crate de biblioteca.
      - Um pacote deve conter pelo menos um crate, seja ele de biblioteca ou binário.
      - Um pacote pode ter múltiplos crates binários colocando arquivos no diretório src/bin, cada arquivo será um crate binário separado.
    - $ cargo new my-project
      - convenção cargo
      - !rever melhorar essa parte
      - src/main.rs - crate binario
      - src/lib.rs - crate de lib
  - quiz
    - package > crate > module
    - Um pacote é a unidade organizacional de nível superior, contendo crates. Um crate contém módulos.
- 7.3 - Defining Modules to Control Scope and Privacy  
Definindo módulos para controlar o escopo e a privacidade.
  - 7.3.1 - Defining Modules to Control Scope and Privacy
    - Nesta seção, falaremos sobre módulos e outras partes do sistema de módulos:
      - namely *paths:* permitem nomear itens;
      - “use”: traz um caminho para o escopo;
      - “pub”: tornar itens públicos.
      - “as”: pacotes externos
      - operador glob.
  - 7.3.2 - Modules Cheat Sheet
    - ao compilar um crate o compilador procura o crate raiz
      - src/main.rs ou src/lib.rs
    - **declaração de modulos**
      - no crate raiz posso declarar modulos
      - como declarar?
        - mod garden {}
      - onde o compilador procura por um modulo?
        - dentro do arquivo atual. ex: src/main.rs -> mod garden {}
        - src/garden.rs
        - src/garnden/mod.rs
    - **declaração de submodulos**
      - não posso declarar submodulos no crate raiz
      - posso colocar dentro de outros modulos
        - src/garden.rs → src/garden/vegetables.ts
        - src/garden.rs → src/garden/vegetables/mod.ts
          - forma menos idiomatica e antiga suportada
          - fica ruim na ide ao editar varios arquivos com o mesmo nome
      - compilador procura o submodulo dentro do diretorio com o mesmo nome do modulo pai
    - **paths to code in modules:**
      - como acessar o modulo dentro do crate?
        - crate::garden::vegetables::Asparagus
        - Uma vez que um módulo faça parte do seu crate, você pode referenciar o código desse módulo de qualquer outro lugar dentro do mesmo crate
        - verificar se as regras de privacidade permitem
    - **private vs public**
      - por padrão um modulo é privado
      - adicionar pub mod para ser publico
    - **use keyword**
      - dentro de um escopo cria um atalho para itens, reduzindo a repetição de caminhos longos
      - use crate::garden::vegetables::Asparagus;
      - agora posso usar assim “Asparagus”
    - **Pergunta: Pro módulo funcionar preciso registrar ele?**
      - Sim, é necessário registrar módulos de topo no arquivo raiz com mod nome_modulo; para incluí-los no crate. Já submódulos devem ser registrados usando mod dentro do módulo pai, não no arquivo raiz.
  - 7.3.3 - Grouping Related Code in Modules
    - utilidade
      - organizar codigo no crate
      - controlar privacidade dos módulos, itens
      - Privacidade: define o que pode ou não ser acessado fora do módulo
    - example resturant
      - cargo new restaurant --lib
    - Os módulos também podem conter definições para outros itens, como structs, enums, constantes, traits
    - Ao usar módulos, podemos agrupar definições relacionadas e nomear o motivo dessa relação.
    - arvore de modulos, crate::
      - main.rs e lib.rs, cada um deles vira um modulo raiz e ganham keyword crate unica para cada dentro deles
      - main.rs e lib.rs são crates diferentes
      - crate:: não significa “acessar outros arquivos do projeto”. Ele significa “acessar o módulo raiz do crate atual”.
      - Rust não organiza código por arquivos, e sim por módulos.
        - crate:: acessa a árvore de módulos
        - arquivos só “alimentam” essa árvore via mod
        - cada módulo pode estar em outro arquivo
      - Observe que toda a árvore de módulos tem sua raiz no módulo implícito chamado crate.
    - Você acessa o lib.rs no main.rs usando o nome do pacote definido no Cargo.toml
- 7.4 - Paths for Referring to an Item in the Module Tree - Caminhos para referenciar um item na árvore de módulos
  - 7.4.1 - introduction
    - como encontrar um item em uma arvore de modulos?
    - as duas formas do caminho
      - Um caminho absoluto é o caminho completo a partir da raiz de um crate;
        - para código de um crate externo, o caminho absoluto começa com o nome do crate
        - e, para código do crate atual, começa com o literal crate.
      - Um caminho relativo começa no módulo atual e usa self, super ou um identificador no módulo corrente.
    - Os caminhos absolutos e relativos são seguidos por um ou mais identificadores separados por dois pontos duplos ( ::).
    - 1-example.rs
      - chamando de duas formas diferentes add_to_waitlist função, na raiz do crate 
    - quando usar relativo ou absoluto?
      - Em geral, preferimos especificar caminhos absolutos, pois é mais provável que queiramos mover as definições de código e as chamadas de itens independentemente umas das outras.
      - 2-example.rs
    - private and public
      - todos os itens (funções, métodos, structs, enums, módulos e constantes) são privados para os módulos pai por padrão. 
      - Os itens em um módulo pai não podem usar os itens privados dentro dos módulos filhos, mas os itens nos módulos filhos podem usar os itens em seus módulos ancestrais.
      - Rust optou por fazer o sistema de módulos funcionar dessa forma para que ocultar detalhes de implementação internos seja o padrão. Assim, você sabe quais partes do código interno pode alterar sem quebrar o código externo.
      - Rust oferece a opção de expor partes internas do código de módulos filhos para módulos ancestrais externos usando a palavra-chave pub para tornar um item público.
  - 7.4.2 - Exposing Paths with the pub Keyword  
    Revelando caminhos com a pub palavra-chave
    - marcar um modulo publico não torna seu conteudo publico
    - marcar um modulo publico permiti que seus modulos ancestrais faça referencia a ele, não que acesse seu codigo interno
    - modulos são containers
    - 3-example.rs
    - Módulos no mesmo nível ("irmãos")  no mesmo nível podem se acessar mesmo sendo privados
      - Não, porque itens privados só podem ser acessados dentro do mesmo módulo ou de seus submódulos, não de fora.
    - crate:: - raiz da arvore de modulos
    - Melhores práticas para pacotes com um binário e uma biblioteca
      - recomenda colocar boa parte da logica em lib.rs e main.rs ser um client
      - usar o nome do pacote para acessar lib.rs apartir de main.rs
  - 7.4.3 - Starting Relative Paths with super  
    Iniciando Caminhos Relativos com super
    - keyword "super": é usada para criar um caminho relativo a apartir do modulo pai
      - isso ajuda se o pai for movido na arvore de modulos, vai continuar funcionando, se não tivesse isso seria caminho absoluto
    - 5-example.rs  
  - 7.4.4 - Making Structs and Enums Public  
    Tornando Structs e Enums Públicos
    - struct
      - colocar somente pub antes da definição da struct
        - torna a struct publica, mas os campos privados
        - posso tornar quaquer campo publico adicionando pub
      - 6-example.rs
    - enum
      - se tornarmos um enum público, todas as suas variantes também serão públicas.
      - 7-example.rs
- 7.5 - Bringing Paths into Scope with the use Keyword
  Incorporando Caminhos ao Escopo com a “use” Palavra-chave 
  - introduction
    - keyword "use": serve para criar um atalho para um caminho relativo ou absoluto
    - usar “use” é semelhante a criar um link simbolico em um sistema de arquivos
    - 1-example.rs
      - incluindo um modulo no escopo com use
    - caminho com use tambem passam pela verificação de privacidade
    - 2-example.rs
      - use só é valido no escopo daquele modulo, submodulos criam um novo escopo
      - nesse exemplo o hosting foi acessado em um escopo diferente do use
  - 7.5.2 - Creating Idiomatic use Paths
    - 3-example.rs
    - importação idiomatica
    - use idiomatico: funções
      - trazer o pai da funçaõ
    - use idiomatico: structs enums
      - trazer o caminho completo
      - quando um item tempo mesmo nome em um outros modulo, é melhor importar o modulo
  - 7.5.3 - Re-exporting Names with pub use
    Reexportando nomes com pub use
    - use traz o nome para o escopo, pub + use permiti reexportação de um nome para um codigo externo fora do escopo
    - 4-example.rs
    - Com a reexportação pub use, podemos escrever nosso código com uma estrutura, mas expor uma estrutura diferente.
  - 7.5.4 - Using External Packages
    Utilizando pacotes externos
    - usando pacote rand do do crates.io no escopo
    - precisa estar no cargo.toml se é do crates.io
    - posso pegar itens do pacote std(biblioteca padrão) que já vem no rust por padrão
    - 5-example.rs
  - 7.5.5 - Using Nested Paths to Clean Up Large “use” Lists
    Utilizando caminhos aninhados para limpar “use” listas grandes
    - 6-example.rs
    - caminhos aninhados
    - self
  - 7.5.6 - The Glob Operator
    O Operador Glob
    - importar todos os itens publicos de um caminho
    - 7-example.rs
  - anki - 7.5.7 - quiz
    - 8-quiz.rs
- 7.6 - Separating Modules into Different Files
  Separando módulos em arquivos diferentes
  - example in 7.6*.../restaurant
  - parte 1
    - extraindo os modulos do exemplo de restaurante para arquivos separados
    - keyword "mod": carrega na arvore de modulos
      - carreguei apenas uma vez no crate e é valido pra toda a arvore dele
    - para acessar um modulo carregado na arvore de modules precisa usar o caminho para onde ele foi declarado
      -  mod não é uma operação de “inclusão” como você pode ter visto em outras linguagens de programação.
  - parte 2
    - hosting é um modulo filho de front_of_house.rs
      - o submodulo precisa estar dentro da pasta pai
      - se o crate for pai, o pacote é o pai /src
  - parte 3
    - src/front_of_house.rs vs src/front_of_house/mod.rs
      - pensar apartir do crate lib.rs
    - src/front_of_house/hosting.rs src/front_of_house/hosting/mod.rs
- 7.7 - resumo
  - Rust permite dividir um pacote em vários crates e um crate em módulos para que você possa referenciar itens definidos em um módulo a partir de outro módulo. Você pode fazer isso especificando caminhos absolutos ou relativos. Esses caminhos podem ser trazidos para o escopo com uma declaração use, permitindo que você use um caminho mais curto para múltiplos usos do item nesse escopo. O código do módulo é privado por padrão, mas você pode tornar definições públicas adicionando a palavra-chave pub.
