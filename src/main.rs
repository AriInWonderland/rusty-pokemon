extern crate byte_string;

use std::{io::{self, Write}, ops::Index};
use io::{BufReader, Read};
use std::fs::File;

struct Player {
    name: String,
    new_encoded_name: [u8; 11],
}

struct Game {
    //edition: String,
    save_file_path: String,
    save_file_buffer: Vec<u8>,
}

//The idea is that you should be able to change this
const UNKNOWN_CHAR:char = '¿';

fn main() {
    let mut game = Game{
        //edition: String::from("Red"),
        save_file_path: String::new(),
        save_file_buffer: Vec::new(),
    };
    let mut original_player = Player {
        name: String::new(),
        new_encoded_name: [0; 11],
    };
    let mut player = Player {
        name: String::new(),
        new_encoded_name: [0; 11],
    };

    'main_loop: loop{
        match main_menu() {
            1 => {
                load_save_file(&mut game, &mut original_player);
                player.name = original_player.name.to_string();
            }
            2 => { 
                println!("Your name is: {}", player.name);
            }
            3 => {
                println!("Your name is: {}", original_player.name);
            }
            4 => {
                change_player_name(&mut player);
            }
            5 => {
            }
            6 => {
            }
            7 => {
            }
            8 => {
            }
            _ => {
                break 'main_loop;
            }
        };
    }
}

fn change_player_name(player_struct: &mut Player){
    let mut new_name = String::new();
    let new_encoded_name: [u8; 11];

    print!("What will be your new name? ");
    io::stdout().flush().expect("Failed to flush stdout...");
    io::stdin()
        .read_line(&mut new_name)
        .expect("Failed to read user input");

    new_encoded_name = ascii_to_first_gen(&new_name);

    println!("Encoded name: {:?}", new_encoded_name);
    println!("ASCII: {:?}", first_gen_to_ascii(new_encoded_name));

    player_struct.name = new_name;
    player_struct.new_encoded_name = new_encoded_name;
}

fn load_save_file(game_struct: &mut Game, player_struct: &mut Player){
    let mut file_path = String::new();

    print!("Input your file save path: ");
    io::stdout().flush().expect("Failed to flush stdout...");
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read user input");

    game_struct.save_file_path = file_path;
    game_struct.save_file_buffer = create_save_file_buffer(&game_struct.save_file_path);
    player_struct.name = get_player_name(&game_struct.save_file_buffer);
}

fn main_menu() -> i32 {
    let mut pick = String::new();
    println!("╔══════════════════════════════════╗");
    println!("║ 1. Load save file                ║");
    println!("║ 2. Print stats (Buffered)        ║");
    println!("║ 3. Print stats (From file)(WIP)  ║");
    println!("╠══════════════════════════════════╣");
    println!("║ 4. Change my name                ║");
    println!("║ 5. Change my money(WIP)          ║");
    println!("║ 6. Change my inventory (not yet) ║");
    println!("╠══════════════════════════════════╣");
    println!("║ 7. Change my rival's name(WIP)   ║");
    println!("╠══════════════════════════════════╣");
    println!("║ 8. Write my changes(WIP)         ║");
    println!("║ 9. Write and exit(WIP)           ║");
    println!("║ 0. Exit                          ║");
    println!("╚══════════════════════════════════╝");
    print!("--> ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut pick)
        .expect("Failed to read user input");
    pick.trim().parse().unwrap()
}

fn get_player_name(file_buffer: &Vec<u8>) -> String {
    let mut encoded_name: [u8; 11] = [0; 11];
    let mut index = 0;

    'get_characters: for c in 0x2598..0x2598+0xB {
        if file_buffer[c] == 80 {
            break 'get_characters;
        } else {
            encoded_name[index] = file_buffer[c];
            index += 1;
        }
    }
    first_gen_to_ascii(encoded_name)
}

fn ascii_to_first_gen(ascii_string: &String) -> [u8; 11]{
    let mut encoded_text: [u8; 11] = [0; 11];
    let mut index = 0;

    for c in ascii_string.chars() {
        if c >= 'A' && c <= 'Z' || c >= 'a' && c <= 'z' {
            encoded_text[index] = ascii_string.as_bytes()[index] + 63;
        } else {
            match c {
                '(' => encoded_text[index] = 0x9A,
                ')' => encoded_text[index] = 0x9B,
                ':' => encoded_text[index] = 0x9C,
                ';' => encoded_text[index] = 0x9D,
                '[' => encoded_text[index] = 0x9E,
                ']' => encoded_text[index] = 0x9F,
                'é' => encoded_text[index] = 0xBA,
                'ď' => encoded_text[index] = 0xBB,
                'ľ' => encoded_text[index] = 0xBC,
                'ś' => encoded_text[index] = 0xBD,
                'ť' => encoded_text[index] = 0xBE,
                'ú' => encoded_text[index] = 0xBF,
                _ => encoded_text[index] = 0x00,
            };
        }
        index += 1;
    }

    return encoded_text;
}

fn first_gen_to_ascii(pokemon_encoded: [u8; 11]) -> String{
    let mut decoded_name = String::new();
    for c in pokemon_encoded {
        if c == 0 {
            continue;
        }
        if c >= 0x9A && c <= 0x9F {
            match c{
                0x9A => decoded_name.push('('),
                0x9B => decoded_name.push(')'),
                0x9C => decoded_name.push(':'),
                0x9D => decoded_name.push(';'),
                0x9E => decoded_name.push('['),
                0x9F => decoded_name.push(']'),
                _ => {println!("Warning: Unknown character found in player name...\nReplacing with {UNKNOWN_CHAR}");decoded_name.push(UNKNOWN_CHAR);},
            }
        } else if c >= 0xBA && c <= 0xBF {
            match c {
                0xBA => decoded_name.push('é'),
                0xBB => decoded_name.push('ď'),
                0xBC => decoded_name.push('ľ'),
                0xBD => decoded_name.push('ś'),
                0xBE => decoded_name.push('ť'),
                0xBF => decoded_name.push('ú'),
                _ => {
                    println!("Warning: Unknown character found in player name...\nReplacing with {UNKNOWN_CHAR}");
                    decoded_name.push(UNKNOWN_CHAR);
                },
            }
        } else {
            decoded_name.push((c-63).into());
        }
    }
    decoded_name
}

fn create_save_file_buffer(file_path: &str) -> Vec<u8> {
    let file = File::open(file_path.trim()).expect("Failed to open file");
    let mut buf_reader = BufReader::new(file);
    let mut save_file_buffer = Vec::new();
    buf_reader.read_to_end(&mut save_file_buffer).expect("Failed to read buffer");

    return save_file_buffer;
}
