#[warn(dead_code)]

fn main() {
    let blockstate = 
   "XX XX XX XX XX XX XX XX
    PB XX XX XX XX KB BB XX
    XX PB XX PB BB PB XX XX
    XX XX XX XX PB XX XX XX
    XX XX XX XX PW XX XX XX
    XX XX RW XX XX XX XX XX
    PW PW PW XX BW XX XX XX
    XX XX KW XX XX PW XX XX";
    let mut game = chess::game::Game::game_from_blockstate(blockstate);
    //let mut game = chess::game::Game::new();

    let pgn_filepath = "pgn_files/PGN6.txt";
    chess::pgn::read_pgn(pgn_filepath);

    //game.main();
}
