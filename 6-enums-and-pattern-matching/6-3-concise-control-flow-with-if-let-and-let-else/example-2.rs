// staying on the happy path with let else


//v1 - enum, match, if let, 
// moeda de 25 centavos, Verificando se um estado existia em 1900 usando condicionais aninhadas dentro de um if let.
// o if let tem muito trabalho no corpo da instrução, se a complexidade aumentar pode ficar dificil entender como os ramos superiores(state) se relacionam
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
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
    let coin1 = Coin::Quarter(UsState::Alabama);
    let coin2 = Coin::Quarter(UsState::Alaska);

    if let Some(desc) = describe_state_quarter(coin1) {
        println!("{desc}");
    }
    if let Some(desc) = describe_state_quarter(coin2) {
        println!("{desc}");
    }
}

//v2 - rescrevendo
fn describe_state_quarter_v2(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

//v3 - else sem if let
fn describe_state_quarter_v3(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}