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

enum Looking {
    ForLayer(String),
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
    print!("{:?}, {:?}", item_vector, item_vector.len());
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
        String::from("KC"), String::from("Bassi"), String::from("Suic"), String::from("Dancepool"),
        String::from("Cons"), String::from("Last"), String::from("Floor")
    ];
    let mut mode = Looking::ForBoss;

    // main loop starts here
    // We look in boss mode first, then look for layer depending on string value
    // then we append result of layer looking to the string inside and add it to result
    for number in numbers.iter() {
        match mode {
            // We are looking for boss
            Looking::ForBoss => {
                // Iterating through current search query
                for (i, key) in next_char.iter().enumerate() {
                    // If we found a match
                    if number == key {
                        // Add to the counter and look if it's overflow
                        char_counter[i] += 1;
                        if char_counter[i] > max_counter[i] {
                            // if overflow, then reset counter + update mode
                            mode = Looking::ForLayer(names[i].copy());
                            char_counter[i] = 0;
                        }
                    } else { char_counter[i] = 0; }
                    // if match wasn't found, we reset counter to 0

                    // update looking character regardless of what happened
                    // since all changed were looked: found and not over, over and not found

                    // hence I'm too lazy, we do match instead of binding `i` to bosses :p
                    match i {
                        0 => next_char[i] = KC[char_counter[i]],
                        1 => next_char[i] = BAZI[char_counter[i]],
                        2 => next_char[i] = SUIC[char_counter[i]],
                        3 => next_char[i] = DQ[char_counter[i]],
                        4 => next_char[i] = CONS[char_counter[i]],
                        5 => next_char[i] = LAST[char_counter[i]],
                        6 => next_char[i] = FLOOR[char_counter[i]],
                        _ => panic!("we managed to overflow in boss matching :)")
                    }
                }
            }
            // We are looking for chest
            Looking::ForLayer(boss_kind) => {
                todo!("We found a boss, time to look for chest !");
            }
        }
    }
    result;
}