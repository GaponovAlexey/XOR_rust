use std::io::Write;

fn get_input(query: &str) -> String {
    println!("{query}");
    std::io::stdout().flush().expect("err stdout");
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("err stdin");

    buffer.trim().to_owned()
}
fn main() {
    loop {
        println!("**********XOR**********");
        let input_file_name = get_input("Enter file name");
        let _input_file = match std::fs::File::open(input_file_name) {
            Ok(f) => f,
            Err(e) => {
                println!("can't open file {:?}", e);
                continue;
            }
        };
        let key = match get_input("Enter Key").parse::<u8>() {
            Ok(key) => key,
            Err(e) => {
                match e.kind() {
                    std::num::IntErrorKind::Empty => println!("Key not be nothing "),
                    std::num::IntErrorKind::InvalidDigit => println!("Key not be InvalidDigit "),
                    std::num::IntErrorKind::PosOverflow =>
                        println!("Key not be PosOverflow [0, 255] "),
                    std::num::IntErrorKind::NegOverflow => println!("Key not be NegOverflow "),
                    _ => println!("Err getting key"),
                }
                continue;
            }
        };
    }
}
