// String is Unicode!! Now iterate through it!
fn main() {
    let multilingual = "Hi! 안녕! ¡Hola! привет!";
    for ch in multilingual.chars() {
        print!("'{}'", ch);
    }
    println!("");
    println!("len {}", multilingual.len());
    println!("count {}", multilingual.chars().count());

    let maybe = multilingual.find('п');
    if maybe.is_some() {
        let hi = &multilingual[maybe.unwrap()..];
        println!("Hi in Russian: {}", hi);
    }
}