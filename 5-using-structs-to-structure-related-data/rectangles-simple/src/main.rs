// A area função deveria calcular a área de um retângulo, mas a função que escrevemos tem dois parâmetros, e não está claro em nenhum lugar do nosso programa que os parâmetros estão relacionados. Seria mais legível e mais fácil de gerenciar agrupar a largura e a altura. Já discutimos uma maneira de fazer isso na seção “O Tipo Tupla” do Capítulo 3: usando tuplas.

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}