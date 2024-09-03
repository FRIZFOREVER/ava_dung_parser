slint::slint!{
    import { TextEdit, Button, VerticalBox } from "std-widgets.slint";
    export component App inherits Window {
        out property<string> to_parse;
        callback parse <=> parse_button.clicked;
        height: 400px;
        width:  500px;
        TextEdit { 
            width:  200px;
            height: 380px;
            placeholder-text: "Paste here";
            x: 10px;
            y: 10px;
            edited(text) => {
                to_parse = text;
            }
        }
        parse_button := Button { 
            text: "Parse";
            height: 25px;
            width: 60px;
            x: 275px;
            y: 10px;
        }
        erase_button := Button {
            text: "Erase";
        }
        Text {
            height: 250px;
            width: 250px;
            x: 220px;
            y: 45px; 
            font-family: "Space Mono";
            overflow: TextOverflow.elide;
            text: "Parsing result will be here";
            wrap: TextWrap.word-wrap;
        }
    }
}

use std::{fmt::Error, usize};

use hex;

const KC: [u8; 14] = [0x4B, 0x6E, 0x69, 0x67, 0x68, 0x74, 0x2D, 0x43, 0x61, 0x70, 0x74, 0x61, 0x69, 0x6E];
const BAZI: [u8; 14] = [0x42, 0x61, 0x73, 0x69, 0x6C, 0x69, 0x73, 0x6B, 0x2D, 0x52, 0x69, 0x64, 0x65, 0x72];
const SUIC: [u8; 11] = [0x48, 0x69, 0x67, 0x68, 0x2D, 0x50, 0x72, 0x69, 0x65, 0x73, 0x74];
const DQ: [u8; 9] = [0x41, 0x72, 0x63, 0x68, 0x2D, 0x4D, 0x61, 0x67, 0x65];
const CONS: [u8; 9] = [0x43, 0x6F, 0x6E, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74];
const LAST: [u8; 14] = [0x4C, 0x45, 0x47, 0x45, 0x4E, 0x44, 0x41, 0x52, 0x59, 0x5F, 0x42, 0x4F, 0x53, 0x53];
const FLOOR: [u8; 15] = [0x41, 0x56, 0x41, 0x5F, 0x54, 0x45, 0x4D, 0x50, 0x4C, 0x45, 0x5F, 0x45, 0x58, 0x49, 0x54];
const LAYER: [u8; 6] = [0x4C, 0x61, 0x79, 0x65, 0x72, 0x5F];
enum Looking {
    ForLayer,
    ForBoss,
}

fn main() -> Result<(), slint::PlatformError>{
    let app = App::new()?;
    let weak_app = app.as_weak();
    app.on_parse(move || {
        let app = weak_app.upgrade().unwrap();
        let text = app.get_to_parse();
        let numbers = parse(&text);
        let in_string = process_find(numbers);
    });
    let test_text = "0B 23 08 22";
    parse(&test_text);
    app.run()?;
    Ok(())
}

fn parse(text: &str) -> Vec<u8> {
    let item_vector: Vec<u8> = text.split_whitespace()
        .map(|s| hex::decode(s).unwrap()[0])
        .collect();
    print!("{:?}, {:?}, \n", item_vector, item_vector.len());
    item_vector
}

fn process_find(numbers: Vec<u8>) -> Option<String> {

    // resulting string
    let mut result = String::new();

    /* 
    0 - KC
    1 - Bazi
    2 - Suic
    3 - DQ
    4 - Cons
    5 - Last
    6 - Floor
    */

    let mut next_char: [u8; 7] = [
            KC[0], BAZI[0], SUIC[0], DQ[0], 
            CONS[0], LAST[0], FLOOR[0]
        ];
    let mut char_counter: [usize; 7] = [0; 7];
    let max_counter: [usize; 7] = [
            KC.len(), BAZI.len(), SUIC.len(), DQ.len(),
            CONS.len(), LAST.len(), FLOOR.len()
    ];
    let names: [String; 7] = [
        String::from("KC"), String::from("Bassi"), String::from("Suic"),
        String::from("DQ"),
        String::from("Cons"), String::from("Last"), String::from("Floor")
    ];

    let mut layer: usize = 0;

    let mut boss_kind: String = String::from("Unknown_Boss");
    let mut mode = Looking::ForBoss;
    let mut first_digit: char = '0';
    // main loop starts here
    // We look in boss mode first, then look for layer depending on string value
    // then we append result of layer looking to the string inside and add it to result
    for number in numbers.iter() {
        match mode {
            // We are looking for boss
            Looking::ForBoss => {
                // Iterating through current search query
                println!("New char: {number}, ~~{}~~", char::from(*number));
                for (i, key) in next_char.iter().enumerate() {
                    // If we found a match
                    if number == key {
                        println!("Match found: {:?}, {:?}, boss: {i}", &number, &key);
                        println!("New stage is: {}", char_counter[i]+1);
                        // Add to the counter and look if it's overflow
                        char_counter[i] += 1;
                        if char_counter[i] == max_counter[i] {
                            println!("Overflow on {i}");
                            // if overflow, then reset counter + update mode
                            mode = Looking::ForLayer;
                            boss_kind = names[i].clone();
                            println!("~~~~~~~ We found a boss ~~~~~~~");
                            // if overflow on Floor => append and end
                            if i == 6 {
                                result += "Floor";
                                return Some(result)
                            }
                            char_counter[i] = 0;
                        }
                    } else { char_counter[i] = 0; }
                    // if match wasn't found, we reset counter to 0
                }
                // after we iterated through every char, we need to update it accordingly
                    next_char[0] = KC[char_counter[0]];
                    next_char[1] = BAZI[char_counter[1]];
                    next_char[2] = SUIC[char_counter[2]];
                    next_char[3] = DQ[char_counter[3]];
                    next_char[4] = CONS[char_counter[4]];
                    next_char[5] = LAST[char_counter[5]];
                    next_char[6] = FLOOR[char_counter[6]];
            },
            // We are looking for chest
            Looking::ForLayer => {
                println!("New char: {number}, ~~{}~~", char::from(*number));
                // todo!("We found a boss, time to look for chest !");
                // we need to look for "layer_x" pattern
                // where x is the number, that we will get and parse according to boss kind
                let second_digit: char;
                if layer == 7 {
                    second_digit = char::from(*number);
                    let chest_found = chest_confirm(
                        first_digit,
                        second_digit,
                        &boss_kind
                    );
                    println!("{chest_found}");
                    result += &format!("{chest_found}\n");
                    layer = 0;
                    mode = Looking::ForBoss;
                    if boss_kind == "Last" {
                        println!("{result}");
                        return Some(result)
                    }
                }
                if layer == 6 {
                    layer +=1;
                }
                else if *number == LAYER[layer] {
                    layer += 1;
                }
                else {
                    layer = 0;
                }
                first_digit = char::from(*number);

            } // end of last match arm
        } //end of match
    } // end of mainloop
    println!("{result}");
    Some(result)
}

fn chest_confirm (first_digit: char, second_digit: char, boss_kind: &String) -> String {
    println!("{first_digit}, {second_digit}");
    match boss_kind.as_str() {
        "DQ" | "Suic" | "KC" | "Bassi" => {
            match (first_digit, second_digit) {
                ('0', '8') => format!("{boss_kind} - 2 Gold\n"),
                ('0', '9') => format!("{boss_kind} - 1 Gold\n"),
                ('1', '0') => format!("{boss_kind} - 2 Purpl\n"),
                ('1', '1') => format!("{boss_kind} - 1 Purpl\n"),
                _ => panic!("DQ / Suic and 2 more: wrong layer found")
            }
        },
        "Cons" => {
            match (first_digit, second_digit) {
                ('0', '6') => format!("{boss_kind} - 2 Gold\n"),
                ('0', '7') => format!("{boss_kind} - 1 Gold\n"),
                ('0', '8') => format!("{boss_kind} - 2 Purpl\n"),
                ('0', '9') => format!("{boss_kind} - 1 Purpl\n"),
                _ => panic!("Cons: wrong layer found")
            }
        },
        "Last" => {
            match (first_digit, second_digit) {
                ('0', '2') => format!("{boss_kind} - 2 Gold\n"), 
                ('0', '4') => format!("{boss_kind} - 1 Gold\n"),
                ('0', '5') => format!("{boss_kind} - 2 Purpl\n"),
                _ => panic!("Last: wrong layer found")
            }
        },
        _ => panic!("unknown boss"),
    }
}