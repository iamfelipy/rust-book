// Updating a String

//Appending to a String with push_str and push  
  //exemplo 1:
  // String = struct que contem um ponteiro, ...
  let mut s = String::from("foo");
  s.push_str("bar");
  // resultado: foobar

  //exemplo 2:
  let mut s1 = String::from("foo");
  //"bar" é um literal estático
  // o compilador cria um &'static str
  // você não escreve &, mas ele já nasce como referência
  // lifetime = por quanto tempo um dado é válido na memória
  //'static = válido durante todo o programa
  let s2: &'static str = "bar";
  // Sim, push_str só recebe referência (&str) e copia o conteúdo apontado para dentro da String de destino.
  s1.push_str(s2);
  println!("s2 is {s2}");
  // resultado s2 is foobar

  //exemplo 3:
  let mut s = String::from("lo");
  // metodo .push()
  // O método push serve para adicionar um único caractere ao final de uma String existente. Isso modifica a string original, aumentando seu conteúdo em um caractere.
  s.push('l');
  // resultado: lol

// Concatenation with the + Operator or the format! Macro
  // exemplo 1: concatenando com operator +
  // s1 foi movido e não pode ser usado depois de s3
  let s1 = String::from("Hello, ");
  let s2 = String::from("world!");
  let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used 

  //exemplo 2: concatenando com format
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");

  let s = format!("{s1}-{s2}-{s3}");
