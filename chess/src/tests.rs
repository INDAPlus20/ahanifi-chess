#[cfg(test)]
mod tests {
    use super::*;
    use game::{Piece, Rank, Team};
    use crate::pgn;
    use crate::game;
    use crate::{game::Game, moves};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_init() {
        let game: Game = Game::new();
        assert!("♔" == game.matrix[4][7].piece.unwrap().unicode());
        assert!("♖" == game.matrix[0][7].piece.unwrap().unicode());
    }

    #[test]
    fn test_out_of_bounds() {
        assert!(game::not_out_of_bounds(-1, 1) == false);
        assert!(game::not_out_of_bounds(0, 8) == false);
        assert!(game::not_out_of_bounds(-1, -1) == false);
        assert!(game::not_out_of_bounds(8, 0) == false);
        assert!(game::not_out_of_bounds(1, 1) == true);
        assert!(game::not_out_of_bounds(9, 0) == false);
    }

    #[test]
    fn test_coordinate_from_string() {
        println!("{:?}", game::coordinate_from_string("a1").unwrap());
        assert!(game::coordinate_from_string("a1").unwrap() == (0, 0));
        assert_eq!(
            game::coordinate_from_string("z1"),
            Err(String::from("first letter doesnt correspond to a column"))
        );
    }

    #[test]
    fn test_pawn_basic_move() {
        let mut game = Game::new();
        //singlestep move and doublestep move
        assert_eq!(2, game.move_from_string("a2").unwrap().len());
    }
    #[test]
    fn test_king_move() {
        let mut game = Game::new();
        let coordinate = game::coordinate_from_string("e4").unwrap();
        game.white_king_square = game.matrix[coordinate.0 as usize][coordinate.1 as usize];
        game.matrix[coordinate.0 as usize][coordinate.1 as usize].piece = Some(Piece {
            rank: Rank::King,
            team: Team::White,
        });
        assert_eq!(8, game.move_from_string("e4").unwrap().len());
    }
    #[test]
    fn test_queen_move() {
        let init_state = "RB NB BB QB KB BB NB RB
                PB PB PB PB PB PB PB PB
                XX XX XX XX XX XX XX XX
                XX XX XX XX XX XX XX XX
                XX XX XX PW XX XX XX XX
                XX XX XX XX XX XX XX XX
                PW PW XX XX XX PW PW PW
                RW NW BW QW KW BW NW RW";
        let mut game = Game::game_from_blockstate(init_state);
        assert_eq!(3 + 2 + 4, game.move_from_string("d1").unwrap().len())
    }

    #[test]
    fn test_castling() {
        let mut game = Game::new();
        game.matrix[5][0].piece = None;
        game.matrix[6][0].piece = None;
        let moveset = moves::castling(&game, game.matrix[4][0]);
        println!("{:?}", moveset);
        assert_eq!(moveset.len(), 1);
        game.matrix[1][0].piece = None;
        game.matrix[2][0].piece = None;
        game.matrix[3][0].piece = None;
        println!();
        println!("{:?}", moveset);
        let moveset = moves::castling(&game, game.matrix[4][0]);
        assert_eq!(moveset.len(), 2);
    }
    #[test]
    fn test_castling_check() {
        let mut game = Game::new();
        game.matrix[5][0].piece = None;
        game.matrix[6][0].piece = None;
        game.matrix[6][1].piece = None;
        game.matrix[6][4].piece = Some(Piece {
            rank: Rank::Rook,
            team: Team::Black,
        });
        let moveset = moves::castling(&game, game.matrix[4][0]);
        println!("{:?}", moveset);
        assert_eq!(0, moveset.len());
    }

    #[test]
    fn test_stalemate() {
        let init_state: &str = "KB XX XX XX XX XX XX XX
         XX XX QW XX XX XX XX XX
         XX XX XX XX XX XX XX XX
         XX XX XX XX XX XX XX XX
         XX XX XX XX XX XX XX XX
         XX XX XX XX XX XX XX XX
         XX XX XX XX XX XX XX XX
         XX XX XX XX KW XX XX XX";
        let mut game = Game::game_from_blockstate(init_state);
        game.player = Team::Black;
        game.calculate_game_state();
        assert_eq!(game::GameState::Stalemate, game.get_game_state());
    }

    #[test]
    fn test_checkmate() {
        let init_state: &str = "KB XX XX XX XX XX XX XX
         XX XX QW XX XX XX XX XX
         XX XX XX XX XX XX XX XX
         XX XX XX XX XX XX XX XX
         XX XX XX XX XX XX XX XX
         XX XX XX XX XX XX XX XX
         XX XX XX XX XX XX XX XX
         RW XX XX XX KW XX XX XX";
        let mut game = Game::game_from_blockstate(init_state);
        game.player = Team::Black;
        game.calculate_game_state();
        assert_eq!(game::GameState::Checkmate, game.get_game_state());
    }

    #[test]
    fn test_check() {
        let init_state: &str = "KB XX XX XX XX XX XX XX
         XX XX XX XX XX XX XX XX
         XX XX XX XX XX XX XX XX
         XX XX XX XX XX XX XX XX
         XX XX XX XX XX XX XX XX
         XX XX XX XX XX XX XX XX
         XX XX XX XX XX XX XX XX
         RW XX XX XX KW XX XX XX";
        let mut game = Game::game_from_blockstate(init_state);
        game.player = Team::Black;
       
        assert_eq!(game::GameState::Check, game.calculate_game_state());
    }

    #[test]
    fn test_with_pgn(){
           let pgn_filepaths=vec!["pgn_files/PGN1.txt","pgn_files/PGN2.txt","pgn_files/PGN3.txt","pgn_files/PGN4.txt","pgn_files/PGN5.txt","pgn_files/PGN6.txt"];
           for filepath in pgn_filepaths {
               compare_pgn_and_game(filepath)
           }
    }

    fn compare_pgn_and_game(pgn_filepath:&str){
        let mut game=game::Game::new();
        let (actions,gamestates)=pgn::read_pgn(pgn_filepath);
        for (i,action) in actions.iter().enumerate(){
            game.perform_action(*action);
            assert_eq!(game.get_game_state(),gamestates[i]);
            
        }
    }
}
