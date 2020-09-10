pub mod units {
    #[derive(Debug, Copy, Clone)]
    pub struct Piece {
        pub rank: Ranks,
        pub color: Color,
        pub offset: [[i8; 2]; 8],
        pub sliding: bool,
    }
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum Ranks {
        Empty,
        Pawn,
        Rook,
        Knight,
        Bishop,
        Queen,
        King,
    }

    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum Color {
        Empty,
        White,
        Black,
    }

    impl Piece {
        pub fn piece_constructor(rank: Ranks, color: Color) -> Piece {
            let piece = match rank {
                Ranks::King => Piece {
                    rank: rank,
                    color: color,
                    offset: [
                        [-1, 0],
                        [-1, 1],
                        [0, 1],
                        [1, 1],
                        [1, 0],
                        [1, -1],
                        [0, -1],
                        [1, -1],
                    ],
                    sliding: false,
                },
                Ranks::Queen => Piece {
                    rank: rank,
                    color: color,
                    offset: [
                        [-1, 0],
                        [-1, 1],
                        [0, 1],
                        [1, 1],
                        [1, 0],
                        [1, -1],
                        [0, -1],
                        [1, -1],
                    ],
                    sliding: true,
                },
                Ranks::Bishop => Piece {
                    rank: rank,
                    color: color,
                    offset: [
                        [-1, 1],
                        [1, 1],
                        [1, -1],
                        [1, -1],
                        [0, 0],
                        [0, 0],
                        [0, 0],
                        [0, 0],
                    ],
                    sliding: true,
                },
                Ranks::Rook => Piece {
                    rank: rank,
                    color: color,
                    offset: [
                        [-1, 0],
                        [0, 1],
                        [1, 0],
                        [0, -1],
                        [0, 0],
                        [0, 0],
                        [0, 0],
                        [0, 0],
                    ],
                    sliding: true,
                },
                Ranks::Knight => Piece {
                    rank: rank,
                    color: color,
                    offset: [
                        [-2, 1],
                        [-1, 2],
                        [1, 2],
                        [2, 1],
                        [2, -1],
                        [1, -2],
                        [-1, -2],
                        [-2, -1],
                    ],
                    sliding: false,
                },
                Ranks::Pawn => Piece {
                    rank: rank,
                    color: color,
                    offset: [
                        [0, 1],
                        [-1, 1],
                        [1, 1],
                        [0, 0],
                        [0, 0],
                        [0, 0],
                        [0, 0],
                        [0, 0],
                    ],
                    sliding: false,
                },
                Ranks::Empty => Piece {
                    rank: rank,
                    color: color,
                    offset: [
                        [0, 0],
                        [0, 0],
                        [0, 0],
                        [0, 0],
                        [0, 0],
                        [0, 0],
                        [0, 0],
                        [0, 0],
                    ],
                    sliding: false,
                },
            };
            piece
        }
    }
}

pub mod actions {
    use super::board;
    use super::units;

    pub enum Actions {
        Move,
    }

    
    pub fn psuedo_legal_moves(from:(i8,i8), board: board::Board) -> Vec<(i8,i8)> {
        let mut moves: Vec<(i8,i8)> = vec![];
        let piece_on_sq = board.matrix[from.0 as usize][from.1 as usize].piece;
        if piece_on_sq.rank == units::Ranks::Empty {
            return moves;
        }
        let offset = &piece_on_sq.offset;
        let current_pos = &from;

        for vector in offset {
            if vector[0]==0 && vector[1]==0{
                break;
            }
                loop {
                    let new_x_value = (current_pos.0 + &vector[0]);
                    let new_y_value = (current_pos.1+ &vector[1]);
                    println!("{:?}", vector);
                    if new_x_value < 0 || new_x_value > 7 {
                        break;
                    }
                    if new_y_value < 0 || new_y_value > 7 {
                        break;
                    }
                    let to_sq = board.matrix[new_y_value as usize][new_x_value as usize];

                    if to_sq.piece.color == piece_on_sq.color {
                        break;
                    }
                   
                    moves.push((new_x_value,new_y_value));
                    let current_pos= &to_sq.coordinate;
                    if !piece_on_sq.sliding{
                        break;
                    }
                }    
        }

         
        
        moves
    }

    pub fn move_coordinate(from:(i8,i8),to:(i8,i8),board:board::Board) -> Result<Move, () >{
       if  psuedo_legal_moves(from,board).contains(&to){
           let current_move =Move{
                from_sq:from,
                to_sq:to,
           };
        return Ok(current_move);
       }
       Err(())
       
    }
    pub fn validate_moves() {}
    #[derive(Debug)]
    pub struct Move {
        pub from_sq: (i8,i8),
        pub to_sq: (i8,i8)
    }
    

}

pub mod board {
    use super::load_from_file;
    use super::units;
    use super::actions;
    use std::fmt;

    #[derive(Debug, Copy, Clone)]
    pub struct Square {
        pub piece: units::Piece,
        pub coordinate: (i8, i8),
    }

    impl Square {
        fn is_empty(&self) -> bool {
            if let units::Ranks::Empty = self.piece.rank {
                return true;
            }
            false
        }
    }

    impl fmt::Display for Square {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.is_empty() {
                return write!(f, ".");
            }

            write!(f, "{}", self.piece.rank as i32)
        }
    }
    #[derive(Debug, Copy, Clone)]
    pub struct Board {
        pub matrix: [[Square; 8]; 8],
    }



    impl fmt::Display for Board {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut formatted_string = String::new();
            for x in self.matrix.iter() {
                for y in x {
                    formatted_string.push_str(&String::from(format!("{} ", y)));
                }
                formatted_string.push_str(&String::from(format!("\n")));
            }
            write!(f, "{}", formatted_string)
        }
    }

    fn index_to_rank(index: usize) -> units::Ranks {
        match index {
            index if index == units::Ranks::Empty as usize => units::Ranks::Empty,
            index if index == units::Ranks::Pawn as usize => units::Ranks::Pawn,
            index if index == units::Ranks::Rook as usize => units::Ranks::Rook,
            index if index == units::Ranks::Knight as usize => units::Ranks::Knight,
            index if index == units::Ranks::Bishop as usize => units::Ranks::Bishop,
            index if index == units::Ranks::Queen as usize => units::Ranks::Queen,
            index if index == units::Ranks::King as usize => units::Ranks::King,
            _ => panic!("There is no Rank"),
        }
    }

    pub fn new(config_file: &str) -> Board {
        let board = Board {
            matrix: load_from_file::load_from_pgn(config_file),
        };

        board
    }

    pub fn create_empty_square() -> Square {
        let empty_square = Square {
            piece: units::Piece::piece_constructor(units::Ranks::Empty, units::Color::Empty),
            coordinate: (0, 0),
        };
        empty_square
    }

    

   

}

mod load_from_file {
    use super::board;
    use super::units;
    use std::fs;

    pub fn load_from_pgn(path: &str) -> [[board::Square; 8]; 8] {
        let data = fs::read_to_string(path).expect("Failed to read file");
        let split_data: Vec<Vec<&str>> = data
            .lines()
            .map(|line| line.split_whitespace().collect())
            .collect();

        let empty_square = board::create_empty_square();

        let mut matrix: [[board::Square; 8]; 8] = [[empty_square; 8]; 8];

        for (i, line) in split_data.iter().enumerate() {
            for (j, pair) in line.iter().enumerate() {
                matrix[j][7-i] = board::Square {
                    piece: map_letter_to_piece(pair),
                    coordinate: (j as i8, 7-i as i8),
                }
            }
        }
        return matrix;
    }

    fn map_letter_to_piece(letter_pair: &str) -> units::Piece {
        let rank_letter = letter_pair.chars().nth(0).expect("not a pair");
        let color_letter = letter_pair.chars().nth(1).expect("not a pair");

        let color: units::Color;
        let rank: units::Ranks;
        let offset: [u8; 8];
        if color_letter == 'B' {
            color = units::Color::Black;
        } else if color_letter == 'W' {
            color = units::Color::White;
        } else {
            color = units::Color::Empty;
        }

        if rank_letter == 'X' {
            rank = units::Ranks::Empty;
        } else if rank_letter == 'P' {
            rank = units::Ranks::Pawn;
        } else if rank_letter == 'R' {
            rank = units::Ranks::Rook;
        } else if rank_letter == 'N' {
            rank = units::Ranks::Knight;
        } else if rank_letter == 'B' {
            rank = units::Ranks::Bishop;
        } else if rank_letter == 'Q' {
            rank = units::Ranks::Queen;
        } else if rank_letter == 'K' {
            rank = units::Ranks::King;
        } else {
            rank = units::Ranks::Empty;
        }

        units::Piece::piece_constructor(rank, color)
    }
}

mod game {
    use super::board;
    use super::actions;
    use super::units;

    struct Player{
        killed: Vec<units::Piece>,
    }

    
    pub struct Game {
        board: board::Board,
        players: [Player;2],
    }

    impl Game {
        fn new() {}
        fn replace(piece_move:actions::Move){

        }
        fn move_coordinate(&self,from:(i8,i8), to:(i8,i8)){
            actions::move_coordinate(from, to,self.board);
        }

        fn perform_action(&self,action:actions::Actions){
            match action {
                actions::Actions::Move => {
                     replace()
                },
                _ => panic!("No such move"),
            }
        }
        
    }
}

#[cfg(test)]
mod tests {
    use super::board;
    use super::actions;
    use super::units;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn test_config_file() {
        let path = "board_config.txt";
        let board = board::new(path);
        let test_board_print: &str = &format!("{}", board);
        println!("{}", test_board_print);
        let correct_board_print=
        "2 3 4 5 6 4 3 2 1 1 1 1 1 1 1 1 . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 1 1 1 1 1 1 1 1 2 3 4 5 6 4 3 2";
        assert_eq!(
            correct_board_print.trim(),
            test_board_print.trim().replace('\n', "")
        );
    }

    #[test]
    fn test_pawn_movement() {
        let path = "board_config.txt";
        let board = board::new(path);
        let moves = actions::psuedo_legal_moves((4,1), board);

        assert!(moves[0] == (4, 2));
    }
    #[test]
    fn test_bottom_color() {
        let path = "board_config.txt";
        let board = board::new(path);
        assert!(board.matrix[0][0].piece.color == units::Color::White)
    }

    #[test]
    fn test_same_color_collision() {
        let path = "board_config.txt";
        let board = board::new(path);
        let moves = actions::psuedo_legal_moves((3,0), board);

        assert_eq!(moves.len(), 0);
    }


    
}
