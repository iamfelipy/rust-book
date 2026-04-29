//v1 - owership e permissões na struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // onwership
    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 0,
        height: 0,
    };

    println!("{}", rect.area());

    let other_rect = Rectangle { width: 1, height: 1 };

                   // passei ownership aqui
    let max_rect = rect.max(other_rect);

    // case 2

    let rect1 = Rectangle {
        width: 0,
        height: 0,
    };

    // vai dar erro, estou tentando mutar a struct sem ter mut 
    rect1.set_width(0)

    // case 3

    let mut rect2 = Rectangle {
        width: 0,
        height: 0,
    };

    rect2.set_width(1);

    // não vai compilar, a refererencia não tem mut
    let rect_ref = &rect2;
    rect_ref.set_width(2);
}

//v2 - mover with self
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // onwership
    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 0,
        height: 0,
    };

    let other_rect = Rectangle {
        width: 1,
        height: 1,
    };

    let max_rect = rect.max(other_rect);
    
    // não vai funcionar pq transferi a posse de rect
    println!("{}", rect.area());
}

//v3 - tentando mover atraves de uma referencia
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // onwership
    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

impl Rectangle {
    fn set_to_max(&mut self, other: Rectangle) {
        *self = self.max(other);
    }
}

fn main() {
    let rect = Rectangle {
        width: 0,
        height: 0,
    };

    let other_rect = Rectangle {
        width: 1,
        height: 1,
    };

    // dentro de set_to_max estou tentando mover a referencia self para max, mas referencia não tem ownership
    let max_rect = rect.set_to_max(other_rect);
}

//v4 - 
//struct nao tem dados na heap, o codigo n compila mas é seguro
// cenario hipotetico que se compilasse não geraria comportamento indefinido
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // onwership
    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

impl Rectangle {
    fn set_to_max(&mut self, other: Rectangle) {
        let max = self.max(other);
        *self = max;
    }
}

fn main() {
    let mut rect = Rectangle { width: 0, height: 1 };
    let other_rect = Rectangle { width: 1, height: 0 };
    rect.set_to_max(other_rect);
}

//v5 - 
//set_to_max compila pq tem essa derive e não tem dados na heap
#[derive(Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // onwership
    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

impl Rectangle {
    fn set_to_max(&mut self, other: Rectangle) {
        // self é copiado para max e não movido, por isso compila
        *self = self.max(other);
    }
}

fn main() {
    let mut rect = Rectangle { width: 0, height: 1 };
    let other_rect = Rectangle { width: 1, height: 0 };
    rect.set_to_max(other_rect);
}

//v6
// copy em struct com String e o risco de compilar
// esse codigo gera dupla liberação de memoria
struct Rectangle {
    width: u32,
    height: u32,
    name: String,
}

impl Rectangle {
    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
            name: if self.name > other.name { self.name } else { other.name },
        }
    }
    fn set_to_max(&mut self, other: Rectangle) {
        let max = self.max(other);
        drop(*self); // this is usually implicit, but added here for clarity
        *self = max;
    }
}

fn main() {
    let mut r1 = Rectangle {
        width: 9,
        height: 9,
        name: String::from("r1"),
    };
    let r2 = Rectangle {
        width: 16,
        height: 16,
        name: String::from("r2"),
    };
    r1.set_to_max(r2);
}