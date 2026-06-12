// Using an Enum to Store Multiple Types
// os vetores só podem armazenar valores do mesmo tipo
// existem casos de uso em que é necessario armazenar  uma lista de itens de tipos diferentes

// example 1
// planilhas
enum SpreadsheetCell {
  Int(i32),
  Float(f64),
  Text(String),
}

let row = vec![
  SpreadsheetCell::Int(3),
  SpreadsheetCell::Text(String::from("blue")),
  SpreadsheetCell::Float(10.12)
];