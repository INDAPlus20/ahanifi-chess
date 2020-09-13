use chess;
use std::io;
use std::io::prelude::*;

fn main() {
    let mut game = chess::Game::new();
    let mut error_msg = String::new();
    // game.move_string("a2", "a4");
    // game.move_string("a4", "a5");

    loop {
        println!("{}", game);
        if !error_msg.is_empty() {
            println!("");
            println!("{}", error_msg);
        }
        println!("Generate moves from square: ");
        let input = io::stdin().lock().lines().next().unwrap().unwrap();
        let moves = match game.move_from_string(&input) {
            Ok(a) => a,
            Err(s) => {
                error_msg = s;
                continue;
            }
        };
        if moves.len()==0{
            error_msg=String::from("there are no moves for this piece");
            continue;
        }
        println!("Choose move index: ");
        let input_index = io::stdin()
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        if input_index > moves.len() {
            error_msg = String::from("Please choose a correct index");
            continue;
        }
        game.perform_action(moves[input_index]);

        error_msg = String::from("");
        print!("\x1B[2J\x1B[1;1H"); // Clears terminal screen
    }
}
