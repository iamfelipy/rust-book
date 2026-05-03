// How Matches Interact with Ownership

//v1
let opt: Option<String> = Some(String::from("Hello world!"));

match opt {
    Some(_) => println!("Some!"),
    None => println!("None!"),
}

println!("{:?}", opt);

//v2 - error ownership
let opt: Option<String> = Some(String::from("Hello world!"));

match opt {
    // borrowed: s = &String
    Some(s) => println!("Some: {}", s),
    None => println!("None!"),
}
// 2 some recebe a posse de "hello world", então o println não pode mais usar opt
println!("{:?}", opt);


// v3 - Matching by reference, opt is not moved

let opt: Option<String> = Some(String::from("Hello world!"));

match &opt {
    Some(s) => println!("Some: {}", s),
    None => println!("None!"),
}

println!("{:?}", opt);