// Creating a New Vector
let v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3];

-------------
// Updating a Vector
v.push(5);

-------------
// Reading Elements of Vectors

let v = vec![1, 2, 3, 4, 5];
                  // 1
let third: &i32 = &v[2];
println!("The third element is {third}");
                          // 2
let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}

----
// acessando indice invalido
let v = vec![1, 2, 3, 4, 5];
// da panic
let does_not_exist = &v[100];
// retorna option com none
let does_not_exist = v.get(100);

----

// Lembre-se da regra que afirma que você não pode ter referências mutáveis e imutáveis no mesmo escopo
// nao compila
// evita acessar memoria invalida se o vetor for realocado no push

let mut v = vec![1, 2, 3, 4, 5];
let first = &v[0];
v.push(6);
println!("The first element is: {first}");

----
