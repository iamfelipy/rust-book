//quiz

//quiz1

/*
Question 1

Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.

pub mod a {
    pub mod b {
        pub fn f() { println!("b1"); }
        pub mod c {
            pub fn f() { println!("c1"); }
        }
    }
    pub fn entry() { super::b::c::f(); }
}

pub mod b {
    pub fn f() { println!("b2"); }
    pub mod c {
        pub fn f() { println!("c2"); }
    }
}

fn main() {
    crate::a::entry();
}

Resposta:
This program does compile.
The output of this program will be:
c2

Explicação:
No escopo de a, o caminho super::b::c::f() sobe um nível na hierarquia (ao escopo do módulo pai de a), onde existe outro módulo b com c::f(), então será chamado b::c::f() global, que imprime "c2".
*/

//quiz 2
/*
Question 2

Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.

pub mod point {
    #[derive(Debug)]
    struct Point(i32, i32);
    
    impl Point {
        pub fn origin() -> Self { Point(0, 0) }
    }
}

fn main() {
    let mut p = point::Point::origin();
    p.0 += 1;
    println!("{:?}", p);
}

Resposta:
This program does not compile.
Porque os campos de Point não são públicos, então p.0 não pode ser acessado fora do módulo point.

Explicação:
Embora a estrutura Point e o método estático origin sejam públicos, o campo i32 não está marcado como pub. Portanto, acessar diretamente p.0 fora do módulo point não é permitido. Este programa compilaria se a linha 3 fosse alterada para pub struct Point(pub i32, pub i32).
*/