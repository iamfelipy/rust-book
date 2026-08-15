Este trecho aborda problemas e soluções de ownership e empréstimos em Rust, especialmente ao implementar métodos que precisam acessar referências imutáveis e mutáveis de um mesmo struct. Os tópicos incluem erros comuns do compilador relacionados ao borrow checker, a razão para esses erros devido à elisão de lifetimes, raciocínios de segurança de memória, e como ajustar código para que compile corretamente sem perder eficiência, usando padrões como eliminar métodos auxiliares desnecessários ou copiar valores baratos (como Option).

quiz 4
struct TestResult {
    /// Student's scores on a test
    scores: Vec<usize>,

    /// A possible value to curve all scores
    curve: Option<usize>
}
impl TestResult {  
    pub fn get_curve(&self) -> &Option<usize> { 
        // Ao retornar &self.curve, você está emprestando self inteiro de forma imutável, não apenas curve, porque o método recebe &self. Por isso, o borrow checker considera que todo o struct está emprestado enquanto a referência existir.
        &self.curve 
    }

    /// If there is a curve, then increments all 
    /// scores by the curve
    pub fn apply_curve(&mut self) {
        if let Some(curve) = self.get_curve() {
            for score in self.scores.iter_mut() {
                *score += *curve;
            }
        }
    }
}

// Se você tentasse compilar este programa, qual das seguintes descrições melhor descreve o erro do compilador que você obteria?

// Você respondeu:
// em apply_curve, não é possível emprestar self.scores como mutável para iter_mut

/*

Contexto: 
Devido à elisão de tempos de vida, a função get_curve tem a assinatura de tipo get_curve<'a>(&'a self) -> &'a Option<usize>. Isso significa que uma chamada a self.get_curve() estende o empréstimo de self por todo o tempo de vida retornado, não apenas em self.curve. 
Portanto, self fica emprestado imutavelmente dentro do escopo de let Some(curve) = ..., e self.scores.iter_mut() não pode ser chamado.
ia:
O texto diz que, por causa das regras do Rust, quando você chama self.get_curve(), o Rust entende que você está "emprestando" todo o self de forma imutável enquanto usa o valor retornado. Isso impede que você também pegue um empréstimo mutável (por exemplo, para modificar self.scores) dentro do mesmo bloco, pois Rust não permite ter ambos ao mesmo tempo.

*/

-------------------------

quiz 5
struct TestResult {
    /// Student's scores on a test
    scores: Vec<usize>,

    /// A possible value to curve all scores
    curve: Option<usize>
}
impl TestResult {  
    pub fn get_curve(&self) -> &Option<usize> { 
        &self.curve 
    }

    /// If there is a curve, then increments all 
    /// scores by the curve
    pub fn apply_curve(&mut self) {
        if let Some(curve) = self.get_curve() {
            for score in self.scores.iter_mut() {
                *score += *curve;
            }
        }
    }
}

// Normally if you try to compile this function, the compiler returns the following error:

error[E0502]: cannot borrow `self.scores` as mutable because it is also borrowed as immutable
  --> test.rs:17:26
   |
16 |         if let Some(curve) = self.get_curve() {
   |                              ---------------- immutable borrow occurs here
17 |             for score in self.scores.iter_mut() {
   |                          ^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
18 |                 *score += *curve;
   |                           ------ immutable borrow later used here
   
// Suponha que o compilador NÃO rejeitou esta função. Qual (se houver) dos seguintes programas (1) passaria pelo compilador e (2) possivelmente causaria comportamento indefinido se executado? Marque cada programa que satisfaça ambos os critérios, OU marque "Nenhum destes programas" se nenhum satisfizer.

// Você respondeu:
// Nenhum destes programas

// Contexto: Este programa é na verdade seguro como escrito. É uma limitação do verificador de empréstimos não entender que get_curve apenas toma emprestado curve e não afeta scores. No entanto, em teoria, se get_curve fosse alterada para retornar uma referência a algo contendo self.scores, então a segurança de memória poderia potencialmente ser violada.
// ia:
// pelas regras de empréstimo do Rust, que tratam qualquer acesso mutável a um campo como um empréstimo mutável de toda a struct.


--------------------------------------------------

// quiz 6
struct TestResult {
    /// Student's scores on a test
    scores: Vec<usize>,

    /// A possible value to curve all scores
    curve: Option<usize>
}
impl TestResult {  
    pub fn get_curve(&self) -> &Option<usize> { 
        &self.curve 
    }

    /// If there is a curve, then increments all 
    /// scores by the curve
    pub fn apply_curve(&mut self) {
        if let Some(curve) = self.get_curve() {
            for score in self.scores.iter_mut() {
                *score += *curve;
            }
        }
    }
}

Das correções a seguir (destacadas em amarelo), qual correção melhor satisfaz estes três critérios:

A função corrigida passa no compilador Rust,
A função corrigida preserva a intenção do código original, e
A função corrigida não introduz ineficiências desnecessárias

resposta:

pub fn apply_curve(&mut self) {
    // ao usar if let Some(curve) = self.curve, o valor é copiado automaticamente porque Option<usize> implementa Copy.
    if let Some(curve) = self.curve {
        for score in self.scores.iter_mut() {
            *score += curve;
        }
    }
}

Contexto: Ao inserir a definição de get_curve diretamente em apply_curve, o verificador de empréstimos entende que self.curve não é self.scores, permitindo que a função seja compilada. Essa é uma solução comum para esse tipo de limitação do verificador de empréstimos.

Outra opção é aproveitar o fato de que self.curve é barato de copiar e usar Option::copied, o que liberaria o empréstimo de self assim que .copied() for chamado.