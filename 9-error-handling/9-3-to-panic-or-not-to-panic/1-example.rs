// -- 9.3 - To panic! or Not to panic!

// Cases in Which You Have More Information Than the Compiler

/*
  - O método parse() funciona porque o tipo à esquerda (IpAddr) está explícito, então o compilador sabe que deve tentar converter a String para um IpAddr. O Rust usa type inference junto com o trait FromStr implementado para IpAddr, permitindo que "127.0.0.1".parse() saiba para qual tipo converter, mesmo que só tenha uma string.
- parse chama internamente IpAddr::from_str e retorna um IpAddr (ou erro), pois IpAddr implementa o trait FromStr.
*/

use std::net::IpAddr;

let home: IpAddr = "127.0.0.1"
.parse()
.expect("Endereço IP codificado deve ser válido");