// safely using iterators
// é seguro, evita panic

// iterando sobre um vetor com iter()
// example 1
let mut v: Vec<i32> = vec![1, 2];
let mut iter = v.iter();
// retorna variante do enum Optional
// next avança o ponteiro
let n1: &i32 = iter.next().unwrap();
let n2: &i32 = iter.next().unwrap();
let end: Option<&i32> = iter.next();



// tentando acessar e mutar um vetor ao mesmo tempo
// não compila
fn dup_in_place(v: &mut Vec<i32>) {
  // ao usar v.push, v.iter vai apontar para memoria desalocada
  for n_ref in v.iter() {
    //  Portanto, para usar iteradores com segurança, o Rust não permite adicionar ou remover elementos do vetor durante a iteração.
    v.push(*n_ref);
  }
}


// gerando iteradores com range(intervalos)
let mut v: Vec<i32> = vec![1, 2];
let mut iter: std::ops::Range<usize> = 0..v.len(); 
let i1: usize = iter.next().unwrap(); 
let n1: &i32 = &v[i1];                