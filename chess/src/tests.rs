#[cfg(test)]
mod tests {
    use game::{Piece, Rank, Team};

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
        let mut game = Game::new_empty();
        game.black_king_square = game.matrix[0][7];
        game.matrix[4][7].piece = None;
        game.matrix[0][7].piece = Some(Piece {
            rank: Rank::King,
            team: Team::Black,
        });
        game.player = Team::Black;
        game.matrix[2][6].piece = Some(Piece {
            rank: Rank::Queen,
            team: Team::White,
        });
        println!("{}", game);
        // assert_eq!(game.is_more_moves(),false);
    }
}
