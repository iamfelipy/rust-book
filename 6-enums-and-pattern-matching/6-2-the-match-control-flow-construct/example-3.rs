// Catch-All Patterns and the _ Placeholder

//v1 - other
/*
other = se refere aos outros valores não definidos ou casos possiveis. 
- isso permiti compilar o requisito do match que deve ser exaustivo(ter logica pra todos os casos)
- o padrões dentro do match são avaliados em ordem, por isso o other é o ultimo
*/
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

//v2  _ = generic pattern, wildcard pattern
// nao consigo usar o valor de _
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}

//v3 - () = não executar nada
/*
A tupla sem valores possui um nome especial, `unit` . Esse valor e seu tipo correspondente são ambos escritos () e representam um valor vazio ou um tipo de retorno vazio. Expressões retornam implicitamente o valor `unit` se não retornarem nenhum outro valor.

Aqui, estamos dizendo explicitamente ao Rust que não usaremos nenhum outro valor que não corresponda a um padrão em um braço anterior e que não queremos executar nenhum código neste caso.
*/
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}


