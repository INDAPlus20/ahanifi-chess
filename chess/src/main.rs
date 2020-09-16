use chess;

fn main() {
    let mut game = chess::Game::new();

     game.move_string("a2", "a4");
     game.move_string("a4", "a5");
     game.move_string("e2","e3");
    game.main();
}
