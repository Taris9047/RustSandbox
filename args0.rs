// Cmdline arguments!
fn main() {
    for arg in std::env::args() {
        println!("'{}'", arg);
    }

    // Returning args as String Vec
    // Just like in some other languages like C
    println!("Arguments as Vec<String>");
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() > 0 {
        for arg in args {
            println!("'{}'", arg);
        }
    }
}