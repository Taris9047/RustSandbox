// Switch case!?
fn main () {
    let multilingual = "Hi! 안녕! ¡Hola! привет!";
    println!("Greets: {:?}", multilingual);

    match multilingual.find('п') {
        Some(idx) => {
            let hi = &multilingual[idx..];
            println!("Russian hi {}", hi);
        },
        None => println!("Couldn't find greetig in Russian")
    };
}