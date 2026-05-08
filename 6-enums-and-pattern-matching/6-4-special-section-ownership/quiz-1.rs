//v1
// não compila
// default vive na stack, depois que a função termina default é desalocaddo
// &default apontaria para memoria desalocada
// você não pode retornar uma referência a uma variável local.
// retorna ponteiro invalido
fn make_separator(user_str: &str) -> &str {
    if user_str == "" {
        let default = "=".repeat(10);
        &default
    } else {
        user_str
    }
}

//v2 - solução 1
fn make_separator(user_str: &str) -> String {
    if user_str == "" {
        let default = "=".repeat(10);
        default
    } else {
        user_str.to_string()
    }
}