#### curso de rust seguindo o livro:
https://rust-book.cs.brown.edu/experiment-intro.html

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