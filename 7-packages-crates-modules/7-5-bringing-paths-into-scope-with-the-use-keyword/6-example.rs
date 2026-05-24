// 1: Especificando um caminho aninhado para trazer vários itens com o mesmo prefixo para o escopo.
before
use std::cmp::Ordering;
use std::io;
after
use std::{cmp::Ordering, io};


// 2: usando self, para subcaminhos
before
use std::io;
use std::io::Write;
after
use std::io::{self, Write};