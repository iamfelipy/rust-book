// O código da imagem não compila porque retorna referência a variável local (`default`) que não vive o bastante.
// Uma versão correta que preserva a ideia pode ser:

fn make_separator<'a>(user_str: &'a str) -> String {
    if user_str == "" {
        "-".repeat(10)
    } else {
        user_str.to_string()
    }
}

fn main() {
    let inp1 = "";
    let inp2 = "***";
    println!("{}", make_separator(inp1));
    println!("{}", make_separator(inp2));
}