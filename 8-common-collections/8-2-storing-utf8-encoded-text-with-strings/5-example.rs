// Indexing into Strings

// exemplo 1: esse codigo não compila
  // acessar um caracter de uma String por indice
  let s1 = String::from("hi");
  let h = s1[0]; 

// exemplo 2: comprimento igual ao das letras
  let hello = String::from("Hola");
  //.len() de hello é 4
  //4 bytes de comprimento
  //cada letra nesse caso ocupa 1 byte quando codificada em utf-8 

// exemplo 3: comprimento diferente da quantidade de letras
  // 3 em ascii ocupa um 1 byte
  // 3 (letra cirílica Ze)  ocupa 2 byte
  let hello = String::from("Здравствуйте");
  // tem 12 letras
  // .len() = 24 
  // cada valor escalar unicode nessa string ocupa 2 bytes de armazenamento
  // Poranto, um índice nos bytes da string nem sempre corresponderá a um valor escalar Unicode válido.

// exemplo 4: tentando acessar com indice 
  let hello = "Здравствуйте";
  let answer = &hello[0];

  esse codigo não compila
  "answer" não sera 3 o digito ascii e sim uma letra cirílica Ze
  "3"=Ze ocupa 2 bytes
  em utf-8 esses são os bytes que representa "3"
  [208, 151]

  não existe acesso por índice
  &hello[0] 

  o rust não compila o codigo, algum dos motivos é evitar:
  acessar bytes como se fossem caracteres
  criar referências quebradas no meio de um caractere

// ideia
ponto sobre o utf-8, há 3 formas de analisar strings da perspectiva do rust
- bytes
- valores escalares
- clusters de grafemas
String não é uma sequência de “letras”é uma sequência de bytes UTF-8