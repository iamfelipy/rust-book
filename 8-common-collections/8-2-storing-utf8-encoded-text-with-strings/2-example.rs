// Creating a New String

// exemplo 1: criando String vazia
let mut s = String::new();


// exemplo 2: criando um objeto String apartir de um literal string com o to_string()
let data = "initial contents";
let s = data.to_string();
  // The method also works on a literal directly:
let s = "initial contents".to_string();

// exemplo 3: criando o objeto String apartir de um literal string com o ::from
let s = String::from("initial contents");

// exemplo 4: mostrando o beneficio do utf-8, saudações em diferentes idiomas
let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hello");
let hello = String::from("שלום");
let hello = String::from("नमस्ते");
let hello = String::from("こんにちは");
let hello = String::from("안녕하세요");
let hello = String::from("你好");
let hello = String::from("Olá");
let hello = String::from("Здравствуйте");
let hello = String::from("Hola");