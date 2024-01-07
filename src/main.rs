extern crate byte_string;

use std::{io::{self, Write}, ops::BitOrAssign, char, fs::OpenOptions};
use io::{BufReader, Read};
use std::fs::File;

struct Player {
    name: String,
    new_encoded_name: [u8; 11],
    money: i32,
    encoded_money: [u8; 6],
}

struct Game {
    //edition: String,
    save_file_path: String,
    save_file_buffer: Vec<u8>,
}

struct Enemy {
    name: String,
    encoded_name: [u8; 11],
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
        money: 0,
        new_encoded_name: [0; 11],
        encoded_money: [0; 6],
    };
    let mut player = Player {
        name: String::new(),
        new_encoded_name: [0; 11],
        money: 0, 
        encoded_money: [0; 6],
    };
    let mut rival = Enemy {
        name: String::new(),
        encoded_name: [0; 11],
    };

    'main_loop: loop{
        match main_menu() {
            1 => { 
                load_save_file(&mut game, &mut original_player, &mut rival);
                player.name  = original_player.name.to_string();
                player.money = original_player.money;
            }
            2 => { 
                println!("Your name is: {}", player.name);
                println!("You have ${}", player.money);
            }
            3 => {
                println!("Your name is: {}", original_player.name);
                println!("You have ${}", original_player.money);
            }
            4 => {
                change_player_name(&mut player);
            }
            5 => {
                change_money_menu(&mut player);
            }
            6 => {
                println!("Your's rival name is: {}", rival.name);
            }
            7 => {
                change_rival_name(&mut rival);
            }
            8 => {
                update_buffer(&mut game, &player, &rival);
                write_changes(&game);
            }
            9 => {
            }
            _ => {
                break 'main_loop;
            }
        };
    }
}
fn update_buffer(game_struct: &mut Game, player_struct: &Player, rival_struct: &Enemy) {
    //write player name!
    let mut index = 0;
    for i in 0x2598..0x2598+0xB{
        game_struct.save_file_buffer[i] = player_struct.new_encoded_name[index];
        index += 1;
    }
    
    //last step
    let checksum = calculate_checksum(&game_struct.save_file_buffer);
    game_struct.save_file_buffer[0x3523] = checksum;
    println!("[DEBUG] Written checksum {}", game_struct.save_file_buffer[0x3523]);
}

fn write_changes(game_struct: &Game) {
    let mut save_file = OpenOptions::new()
        .write(true)
        .open(&game_struct.save_file_path.trim())
        .expect("Failed to open file");

    save_file.write(&game_struct.save_file_buffer).expect("Failed to write to file");
}

fn calculate_checksum(save_buffer: &Vec<u8>) -> u8 {
    let mut checksum: i32 = 0;
    for i in 0x2598..=0x3522{
        checksum += save_buffer[i] as i32;
    }
    checksum = !checksum;
    println!("[DEBUG] checksum: {:#0x}", checksum as u8);
    return checksum as u8;
}

fn change_money_menu(player: &mut Player){
    let mut ammount: String = String::new();
    println!("Right now you have ${}", player.money);
    println!("How much do you want?");
    print!("--> ");
    io::stdout().flush().expect("Failed to flush stdout...");

    io::stdin()
        .read_line(&mut ammount)
        .expect("Failed to read user input");

    change_player_money(player, ammount.trim().parse().unwrap());
}

fn change_player_money(player: &mut Player, ammount: i32){
    let mut stack:[u8; 6] = [0, 0, 0, 0, 0, 0];
    let mut cuantity = ammount.to_string();
    let mut index = 5;

    'get_vals: loop{
        match cuantity.pop(){
            Some(c) => {
                stack[index] = c as u8 - 48;
                index -= 1;
            },
            None => {break 'get_vals},
        };
    }

    println!("[DEBUG]: {:?}", stack);
    player.money = ammount;
    player.encoded_money = stack;
}

fn change_rival_name(rival: &mut Enemy){
    let mut new_name = String::new();
    
    println!("What will your's rival new name be?");
    print!("--> ");
    io::stdout().flush().expect("Failed to flush stdout...");
    new_name.clear();
    io::stdin()
        .read_line(&mut new_name)
        .expect("Failed to read user input");

    rival.encoded_name = ascii_to_first_gen(&new_name);
    rival.name = new_name.trim().to_string();
}

fn get_money(game_struct: &Game) -> i32{
    let mut money: i32 = 0;
    let mut stack: [u8; 6] = [0, 0, 0, 0, 0, 0];

    stack[0].bitor_assign(game_struct.save_file_buffer[0x25f3] >> 4);
    stack[1].bitor_assign(game_struct.save_file_buffer[0x25f3] & 0b00001111);
    stack[2].bitor_assign(game_struct.save_file_buffer[0x25f4] >> 4);
    stack[3].bitor_assign(game_struct.save_file_buffer[0x25f4] & 0b00001111);
    stack[4].bitor_assign(game_struct.save_file_buffer[0x25f5] >> 4);
    stack[5].bitor_assign(game_struct.save_file_buffer[0x25f5] & 0b00001111);

    for i in 0..=5{
        money = money * 10 + stack[i] as i32;
    }

    return money;
}
fn change_player_name(player_struct: &mut Player){
    let mut new_name = String::new();
    let new_encoded_name: [u8; 11];

    print!("What will be your new name? ");
    io::stdout().flush().expect("Failed to flush stdout...");
    io::stdin()
        .read_line(&mut new_name)
        .expect("Failed to read user input");

    new_name.trim().to_string();
    new_encoded_name = ascii_to_first_gen(&new_name);

    println!("Encoded name: {:?}", new_encoded_name);
    println!("ASCII: {:?}", first_gen_to_ascii(new_encoded_name));

    player_struct.name = new_name;
    player_struct.new_encoded_name = new_encoded_name;
}

fn get_enemy_name(file_buffer: &Vec<u8>, enemy: &mut Enemy){
    let mut index=0;
    let mut encoded_name: [u8; 11] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    'get_characters: for c in 0x25F6..0x25f6+0xB{
        if file_buffer[c] == 80{
            break 'get_characters
        } else {
            encoded_name[index] = file_buffer[c];
            index += 1;
        }
    }

    enemy.name = first_gen_to_ascii(encoded_name);
}

fn load_save_file(game_struct: &mut Game, player_struct: &mut Player, enemy: &mut Enemy){
    let mut file_path = String::new();

    print!("Input your file save path: ");
    io::stdout().flush().expect("Failed to flush stdout...");
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read user input");


    game_struct.save_file_path = file_path;
    game_struct.save_file_buffer = create_save_file_buffer(&game_struct.save_file_path);
    player_struct.name = get_player_name(&game_struct.save_file_buffer);
    player_struct.money = get_money(game_struct);

    get_enemy_name(&game_struct.save_file_buffer, enemy);
}

fn main_menu() -> i32 {
    let mut pick = String::new();
    println!("╔════════════════════════════════════════╗");
    println!("║ 1. Load save file                      ║");
    println!("║ 2. Print new stats                     ║");
    println!("║ 3. Print unchanged stats               ║");
    println!("╠════════════════════════════════════════╣");
    println!("║ 4. Change my name                      ║");
    println!("║ 5. Change my money                     ║");
    println!("╠════════════════════════════════════════╣");
    println!("║ 6. Print my rival's name               ║");
    println!("║ 7. Change my rival's name              ║");
    println!("╠════════════════════════════════════════╣");
    println!("║ 8. Write my changes(WIP)               ║");
    println!("║ 9. Write and exit(WIP)                 ║");
    println!("║ 0. Exit                                ║");
    println!("╚════════════════════════════════════════╝");
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
