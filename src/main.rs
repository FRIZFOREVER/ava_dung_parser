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
use struct_iterable::Iterable;

const KC: [u8; 14] = [0x4B, 0x6E, 0x69, 0x67, 0x68, 0x74, 0x2D, 0x43, 0x61, 0x70, 0x74, 0x61, 0x69, 0x6E];
const BAZI: [u8; 14] = [0x42, 0x61, 0x73, 0x69, 0x6C, 0x69, 0x73, 0x6B, 0x2D, 0x52, 0x69, 0x64, 0x65, 0x72];
const SUIC: [u8; 11] = [0x48, 0x69, 0x67, 0x68, 0x2D, 0x50, 0x72, 0x69, 0x65, 0x73, 0x74];
const DQ: [u8; 9] = [0x41, 0x72, 0x63, 0x68, 0x2D, 0x4D, 0x61, 0x67, 0x65];
const CONS: [u8; 9] = [0x43, 0x6F, 0x6E, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74];
const LAST: [u8; 14] = [0x4C, 0x45, 0x47, 0x45, 0x4E, 0x44, 0x41, 0x52, 0x59, 0x5F, 0x42, 0x4F, 0x53, 0x53];
const FLOOR: [u8; 15] = [0x41, 0x56, 0x41, 0x5F, 0x54, 0x45, 0x4D, 0x50, 0x4C, 0x45, 0x5F, 0x45, 0x58, 0x49, 0x54];

#[derive(Iterable)]
struct Targets {
    kc: (u8, usize, usize),
    bazi: (u8, usize, usize),
    suic: (u8, usize, usize),
    dq: (u8, usize, usize),
    cons: (u8, usize, usize),
    last: (u8, usize, usize),
    floor: (u8, usize, usize),
}

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
    print!("{:?}, {:?}", item_vector, item_vector.len());
    item_vector
}

fn process_find(numbers: Vec<u8>) -> Option<String> {
    // 1. have an item
    // 2. collect matches
    // 3. update matches
    // 4. update answers

    // resulting string
    let mut result = String::new();

    // search tracking
    let mut looking_for: Targets = Targets {
        kc: (KC[0], 0, KC.len()),
        bazi: (BAZI[0], 0, BAZI.len()),
        suic: (SUIC[0], 0, SUIC.len()),
        dq: (DQ[0], 0, DQ.len()),
        cons: (CONS[0], 0, CONS.len()),
        last: (LAST[0], 0, LAST.len()),
        floor: (FLOOR[0], 0, FLOOR.len()),
    };

    // main search loop via iter
    for item in numbers.iter() {
        // comparing per every search query
        for wanted in looking_for.iter() {

        }
    };
    todo!()
}

fn compare(compare_from: &u8, mut boss: &(u8, usize, usize)) -> Option<String> {
    todo!();
}