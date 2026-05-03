// v1 - quiz
enum Location {
    Point(i32),
    Range(i32, i32),
}

fn main() {
    let l: Location = Location::Range(0, 5);
    let n = match l {
        Location::Point(_) => -1,
        Location::Range(_, 5) => 5,
        Location::Range(0, _) => 0,
        _ => -2,
    };
    // qual o valor de n?
    println!("{n}");
}

// v2 - quiz 
// exmeplo de implementação real
impl<T> Option<T> {
    fn unwrap_or(self, other: T) -> T {
        match self {
            Some(t) => t,
            None => other,
        }
    }
}

// v3 - quiz
// nao compila, x passa ownership para match, value não pode ser usado depois
#[derive(Debug)]
enum Either {
    Left(usize),
    Right(String),
}

fn main() {
    let x = Either::Right(String::from("Hello world"));
    let value = match x {
        Either::Left(n) => n,
        Either::Right(s) => s.len(),
    };
    println!("{x:?} {value}");
}

//v4 - quiz
fn decr_twice_v1(n: u32) -> Option<u32> {
    match n {
        0 => None,
        1 => None,
        n2 => Some(n2 - 2),
    }
}

fn decr_twice_v2(n: u32) -> Option<u32> {
    if n == 0 {
        None
    } else if n == 1 {
        None
    } else {
        Some(n - 2)
    }
}