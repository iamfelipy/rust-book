// 6.3.1 - concise control

// A if let sintaxe permite combinar if and let de uma forma menos verbosa para lidar com valores que correspondem a um padrão, ignorando os demais.

//v1 - sem if let
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {max}"),
    _ => (),
}

//v2 - com if let
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
}

//v3 - if let com else

    // match - sem if let else
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }

    // if let com else
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }