// use art::kinds::PrimaryColor;
// use art::utils::mix;
// beneficios ao usar pub sub no lib.rs e melhora na docs
use art::PrimaryColor;
use art::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}