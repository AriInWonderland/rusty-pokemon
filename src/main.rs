extern crate byte_string;

use std::io::{self, Write};
use io::{BufReader, Read};
use std::fs::File;

struct Player {
    name: String,
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
    let mut player = Player {
        name: String::new(),
    };

    'main_loop: loop{
        match main_menu() {
            1 => {
                load_save_file(&mut game, &mut player);
            }
            2 => { 
                println!("Your name is: {}", player.name);
            }
            3 => {
            }
            4 => {
            }
            5 => {
            }
            6 => {
            }
            7 => {
            }
            _ => {
                break 'main_loop;
            }
        };
    }
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
    println!("║ 3. Print stats (From file)       ║");
    println!("╠══════════════════════════════════╣");
    println!("║ 4. Change my name                ║");
    println!("║ 5. Change my money               ║");
    println!("║ 6. Change my inventory (not yet) ║");
    println!("╠══════════════════════════════════╣");
    println!("║ 7. Change my rival's name        ║");
    println!("╠══════════════════════════════════╣");
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
