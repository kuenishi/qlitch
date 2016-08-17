extern crate qlitch;

fn usage() {
    println!("Usage: qlitch [glitch|find] filename [pattern]");
    std::process::exit(-1)
}

fn main() {
    let command = std::env::args().nth(1);
    let filename = std::env::args().nth(2);
    let pattern = std::env::args().nth(3);
    if command.is_none() || filename.is_none() {
        usage()
    }
    
    match command.unwrap().as_str() {
        "glitch" => {
            if pattern.is_none() {
                println!("needs pattern");
                usage()
            }
            qlitch::glitch(filename.unwrap().as_str(), pattern.unwrap().as_str());
        }
        "find" =>  qlitch::find(filename.unwrap().as_str()),
        _ => usage()
    }
}
