use base64::{decode, encode};
use md5::{compute};
use std::io::{self, Write};
use std::fs;
use rand::Rng;



fn main() {



            println!("                _                       __");
            println!("              /   \\                  /      \\ ");
            println!("             '      \\              /          \\ ");
            println!("            |       |Oo          o|            |");
            println!("            `    \\  |OOOo......oOO|   /        |");
            println!("             `    \\OOOOOOOOOOOOOOO\\//        /");
            println!("               \\ _o\\OOOOOOOOOOOOOOOO//. ___ /");
            println!("           ______OOOOOOOOOOOOOOOOOOOOOOOo.___");
            println!("            --- OO'* `OOOOOOOOOO'*  `OOOOO--");
            println!("                OO.   OOOOOOOOO'    .OOOOO o");
            println!("                `OOOooOOOOOOOOOooooOOOOOO'OOOo");
            println!("              .OO \"OOOOOOOOOOOOOOOOOOOO\"OOOOOOOo");
            println!("          __ OOOOOO`OOOOOOOOOOOOOOOO\"OOOOOOOOOOOOo");
            println!("         ___OOOOOOOO_\"OOOOOOOOOOO\"_OOOOOOOOOOOOOOOO");
            println!("           OOOOO^OOOO0`(____)/\"OOOOOOOOOOOOO^OOOOOO");
            println!("           OOOOO OO000/000000\000000OOOOOOOO OOOOOO");
            println!("           OOOOO O0000000000000000 ppppoooooOOOOOO");
            println!("           `OOOOO 0000000000000000 QQQQ \"OOOOOOO\"");
            println!("            o\"OOOO 000000000000000oooooOOoooooooO'");
            println!("            OOo\"OOOO.00000000000000000000OOOOOOOO'");
            println!("           OOOOOO QQQQ 0000000000000000000OOOOOOO");
            println!("          OOOOOO00eeee00000000000000000000OOOOOOOO.");
            println!("         OOOOOOOO000000000000000000000000OOOOOOOOOO");
            println!("         OOOOOOOOO00000000000000000000000OOOOOOOOOO");
            println!("         `OOOOOOOOO000000000000000000000OOOOOOOOOOO");
            println!("           \"OOOOOOOO0000000000000000000OOOOOOOOOOO'");
            println!("             \"OOOOOOO00000000000000000OOOOOOOOOO\"");
            println!("  .ooooOOOOOOOo\"OOOOOOO000000000000OOOOOOOOOOO\"");
            println!(".OOO\"\"\"\"\"\"\"\"\"\".oOOOOOOOOOOOOOOOOOOOOOOOOOOOOo");
            println!("OOO         QQQQO\"'         0II0         `\"QQQQ");
            println!("OOO                          II");
            println!("`OOo.                        II");
            println!("  `\"OOOOOOOOOOOOoooooooo.    II");
            println!("                             \\/");



    loop {
        println!();
        println!("Input Types:");
        println!("s: String as input");
        println!("f: File as input line by line");
        println!("-----------------------------");
        println!("Available Modules:");
        println!("1: ASCII to Base64");
        println!("2: Base64 to ASCII");
        println!("3: ASCII to Binary");
        println!("4: Binary to ASCII");
        println!("5: ASCII to Hex");
        println!("6: Hex to ASCII");
        println!("7: Binary to Hex");
        println!("8: Hex to Binary");
        println!("MD5: String to MD5");
        println!("Rot: Rotate a-z 1-25 times");
        println!("-----------------------------");
        println!("help: Syntax example");
        println!("exit: Exit");
        println!();

        let mut choice = String::new();
            print!("Enter the input type appended by the desired module:");
            io::stdout().flush().unwrap(); // Ensure "Enter your choice: " is printed before reading input
            io::stdin().read_line(&mut choice).expect("Big dumdum...");


//Start matching
//Start matching
//Start matching
//Start matching


        match choice.trim() {
            "1" => {
                let input = read_input("Enter the ASCII string to encode to Base64: ");
                let encoded = encode(input);
                println_with_padding(&format!("Encoded: {}", encoded));
            }
            "2" => {
                let input = read_input("Enter the Base64 string to decode into ASCII: ");
                match decode(&input) {
                    Ok(bytes) => match String::from_utf8(bytes) {
                        Ok(s) => println_with_padding(&format!("Decoded: {}", s)),
                        Err(_) => println_with_padding("Failed to convert bytes to ASCII string."),
                    },
                    Err(_) => println_with_padding("Failed to decode Base64."),
                }
            }
            "3" => {
                let input = read_input("Enter the ASCII string to encode to Binary: ");
                let binary_encoded = input
                    .bytes()
                    .map(|b| format!("{:08b}", b))
                    .collect::<Vec<String>>()
                    .join(" ");
                println_with_padding(&format!("Encoded to Binary: {}", binary_encoded));
            }
            "4" => {
                let input = read_input("Enter the Binary string to decode into ASCII: ");
                match binary_to_ascii(&input) {
                    Ok(decoded) => println_with_padding(&format!("Decoded to ASCII: {}", decoded)),
                    Err(e) => println_with_padding(&format!("Error: {}", e)),
                }
            }
            "5" => {
                let input = read_input("Enter the ASCII string to encode to Hex: ");
                let hex_encoded = ascii_to_hex(&input);
                println_with_padding(&format!("Encoded to Hex: {}", hex_encoded));
            }
            "6" => {
                let input = read_input("Enter the Hex string to decode into ASCII: ");
                match hex_to_ascii(&input) {
                    Ok(decoded) => println_with_padding(&format!("Decoded to ASCII: {}", decoded)),
                    Err(e) => println_with_padding(&format!("Error: {}", e)),
                }
            }
            "7" => {
                let input = read_input("Enter the Binary string to decode into Hex: ");
                match binary_to_hex(&input) {
                    Ok(hex_decoded) => println_with_padding(&format!("Decoded to Hex: {}", hex_decoded)),
                    Err(e) => println_with_padding(&format!("Error: {}", e)),
    }
}
            "8" => {
                let input = read_input("Enter the Hex string to encode to Binary: ");
                match hex_to_binary(&input) {
                    Ok(binary_encoded) => println_with_padding(&format!("Encoded to Binary: {}", binary_encoded)),
                    Err(e) => println_with_padding(&format!("Error: {}", e)),
    }
}
            "MD5" => {
                let input = read_input("Enter the string to hash with MD5: ");
                let hash = create_md5_hash(&input);
                println_with_padding(&format!("MD5 Hash: {}", hash));
},
            "Rot" => {
                let mut ciphertext = String::new();
                println_with_padding("What's the string?");
                io::stdin()
                .read_line(&mut ciphertext)
                .expect("Failed to read line");
                ciphertext = ciphertext.trim_end().to_string();

                for shift in 1..=25 {
                let decrypted_message = Rotation(&ciphertext, shift);
                println!("Shift {}: {}", shift, decrypted_message);
                }
            }
            "help" => {
                println!("'s1' will convert an ASCII string to Base64");
                println!("'f1' will read lines from a file and convert each ASCII line to Base64");
                println!("'fMD5' will read lines from a file and run them through the MD5 hashing algorithm")
            }
            "exit" => {
                println_with_padding("Exiting...");
                break;
            }
            _ => println_with_padding("Yuh fuckin dummy."),
        }
    }
}




//start of functions
//start of functions
//start of functions
//start of functions
//start of functions
//start of functions
//start of functions





fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn read_from_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn println_with_padding(content: &str) {
    println!("\n{}\n", content);
}

fn binary_to_ascii(s: &str) -> Result<String, &'static str> {
    s.split(' ')
        .map(|byte_str| u8::from_str_radix(byte_str, 2))
        .collect::<Result<Vec<u8>, _>>()
        .map_err(|_| "Failed to parse binary to ASCII")
        .and_then(|bytes| String::from_utf8(bytes).map_err(|_| "Failed to convert bytes to ASCII"))
}

fn hex_to_ascii(input: &str) -> Result<String, &'static str> {
    (0..input.len())
        .step_by(3) // Considering there's a space between each hex byte representation.
        .map(|i| u8::from_str_radix(&input[i..i + 2], 16))
        .collect::<Result<Vec<u8>, _>>()
        .map_err(|_| "Failed to parse hex to ASCII")
        .and_then(|bytes| String::from_utf8(bytes).map_err(|_| "Failed to convert bytes to ASCII"))
}

fn binary_to_hex(input: &str) -> Result<String, &'static str> {
    input.split(' ')
         .map(|byte_str| u8::from_str_radix(byte_str, 2))
         .collect::<Result<Vec<u8>, _>>()
         .map_err(|_| "Invalid binary digit")
         .map(|bytes| bytes.iter().map(|&b| format!("{:02x}", b)).collect::<Vec<String>>().join(" "))
}

fn hex_to_binary(input: &str) -> Result<String, &'static str> {
    (0..input.len())
        .step_by(3) // Considering there's a space between each hex byte representation.
        .map(|i| u8::from_str_radix(&input[i..i+2], 16))
        .collect::<Result<Vec<u8>, _>>()
        .map_err(|_| "Invalid hex digit")
        .map(|bytes| bytes.iter().map(|&b| format!("{:08b}", b)).collect::<Vec<String>>().join(" "))
}

fn ascii_to_hex(input: &str) -> String {
    input
        .as_bytes()
        .iter()
        .map(|&b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join(" ")
}

fn create_md5_hash(input: &str) -> String {
    let digest = md5::compute(input);
    format!("{:x}", digest)
}

fn Rotation(ciphertext: &str, shift: u8) -> String {
    ciphertext
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() { 'a' } else { 'A' } as u8;
                let offset = c as u8 - first;
                let new_offset = (offset + (26 - shift) % 26) % 26;
                (first + new_offset) as char
            } else {
                c
            }
        })
        .collect()
}
fn random_char_between_a_and_e() -> char {
    let characters = ['a', 'b', 'c', 'd', 'e'];
    let mut rng = rand::thread_rng();
    characters[rng.gen_range(0..characters.len())]
}