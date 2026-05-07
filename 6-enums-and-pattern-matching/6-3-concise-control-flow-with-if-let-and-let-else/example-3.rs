//v4 quiz 1: print_range_max - idiomático com if let
enum Location {
    Point(i32),
    Range(i32, i32),
}

fn print_range_max(loc: &Location) {
    if let Location::Range(_, max) = loc {
        println!("{max}");
    }
}

//v5 quiz 2
enum Location {
    Point(i32),
    Range(i32, i32),
}

fn get_start(loc: &Location) -> i32 {
    match loc {
        Location::Point(n) => *n,
        Location::Range(start, _) => *start,
    }
}