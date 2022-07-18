use std::{
    fs,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
struct Move {
    move_x: i32,
    mov_y: i32,
}

fn simple_serde() -> Result<(), ()> {
    let var_move = Move {
        move_x: 1,
        mov_y: 3,
    };
    println!("first_print:  {:?}", &var_move);

    let serded_str = serde_json::to_string(&var_move).unwrap();

    fs::write("file", &serded_str).unwrap();

    let x: Move = serde_json::from_str(&fs::read_to_string("file").unwrap()).unwrap();

    println!("second_print:  {:?}", &x);

    Ok(())
}

fn serde_ron() {
    let var_move = Move {
        move_x: 1,
        mov_y: 3,
    };

    println!("first_print:  {:?}", &var_move);

    let serded_str = ron::to_string(&var_move).unwrap();

    println!("first_print:  {:?}", &serded_str);
}

fn mutiple_buffer() {
    let mut moves: Vec<Move> = vec![];
    moves.reserve(1000);
    for i in 0..1000 {
        let t = i as i32;
        moves.push(Move {
            mov_y: t * t,
            move_x: t + 1,
        });
    }

    let mut fi = fs::File::create("temp").unwrap();

    for val in moves {
        let buf = bson::to_vec(&val).unwrap();
        fi.write(&buf).unwrap();
    }

    //let mut buf: Vec<u8> = Vec::new();

    //let len = fi.metadata().unwrap().len();
    //buf.reserve(len as usize);

    //let read_len = fi.read_to_end(&mut buf).unwrap();
    //assert_eq!(read_len as u64,len);

    let move1:Move = bson::from_reader(&fi).unwrap();

    println!("{:?}",move1)
        



}

fn main() {
    mutiple_buffer();
}
