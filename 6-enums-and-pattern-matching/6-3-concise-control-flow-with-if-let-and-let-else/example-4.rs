// full example

enum UsState {
    Alabama,
    Alaska,
    // adicione os outros estados conforme necessário
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
            // Exemplos adicionais:
            UsState::California => year >= 1850,
            UsState::NewYork => year >= 1788,
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

fn main() {
    let c1 = Coin::Quarter(UsState::Alabama);
    let c2 = Coin::Quarter(UsState::Alaska);
    let c3 = Coin::Penny;

    println!("{:?}", describe_state_quarter(c1));
    println!("{:?}", describe_state_quarter(c2));
    println!("{:?}", describe_state_quarter(c3));
}