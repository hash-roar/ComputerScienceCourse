use std::fs;

use  serde::{Deserialize,Serialize};
#[derive(Deserialize,Serialize,Debug)]
struct  Move{
    move_x: i32,
    mov_y: i32,
}

fn simple_serde()->Result<(),()>
{

    let var_move = Move{
        move_x: 1,
        mov_y:3
    };
    println!("first_print:  {:?}",&var_move);

    let serded_str = serde_json::to_string(&var_move).unwrap();

    fs::write("file",&serded_str ).unwrap();

    let x: Move = serde_json::from_str(&fs::read_to_string("file").unwrap()).unwrap();


    println!("second_print:  {:?}",&x);

    Ok(())
}




fn main() {



}
