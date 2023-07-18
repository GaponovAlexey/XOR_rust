use std::io::{ Write, BufReader, Read, BufWriter };

fn get_input(query: &str) -> String {
    println!("{query}");
    std::io::stdout().flush().expect("err stdout");
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("err stdin");

    buffer.trim().to_owned()
}

fn process_file_data(data: &Vec<u8>, key: u8) -> Vec<u8> {
    let mut pro_data = Vec::with_capacity(data.len());

    for byte in data {
        pro_data.push(byte ^ key);
    }

    pro_data
}

fn main() {
    loop {
        println!("**********XOR**********");
        let input_file_name = get_input("Enter file name");
        let _input_file = match std::fs::File::open(&input_file_name) {
            Ok(f) => f,
            Err(e) => {
                println!("can't open file {:?}", e);
                continue;
            }
        };
        let key = match get_input("Enter Key").parse::<u8>() {
            Ok(key) => { key }
            Err(e) => {
                match e.kind() {
                    std::num::IntErrorKind::Empty => println!("key mustn't be empty"),
                    std::num::IntErrorKind::InvalidDigit => println!("Key not be InvalidDigit "),
                    std::num::IntErrorKind::PosOverflow =>
                        println!("key must be in rangy [0, 255] "),
                    _ => println!("Err getting key"),
                }
                continue;
            }
        };

        let mut reader = BufReader::new(_input_file);

        let mut input_data = Vec::new();

        if let Err(err) = reader.read_to_end(&mut input_data) {
            println!("Error reading file{:?}", err);
            continue;
        }

        let processed_data = process_file_data(&mut input_data, key);

        let output_file_name = get_input("Enter file name to output");
        let output_name = match std::fs::File::create(&output_file_name) {
            Ok(f) => f,
            Err(e) => {
                println!("can't create file {output_file_name} {:?}", e);
                continue;
            }
        };
        let mut writer = BufWriter::new(output_name);

        if let Err(err) = writer.write_all(&processed_data) {
            println!("Error writing to out file {err}\n");
            continue;
        }

        println!("\n");
    }
}
