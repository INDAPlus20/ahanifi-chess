#[cfg(test)]
mod tests {
    use super::board;
    use super::units;
    use super::readPGN;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn test_config_file(){
        let path="board_config.txt";
        let board=board::new(path);
        let test_board_print:&str=&format!("{}",board);
        println!("{}",test_board_print);
        let correct_board_print=
        "2 3 4 5 6 4 3 2 1 1 1 1 1 1 1 1 . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 1 1 1 1 1 1 1 1 2 3 4 5 6 4 3 2";
        assert_eq!(correct_board_print.trim(),test_board_print.trim().replace('\n',""));
    }
    
    fn init_board(){
        let black_pawn= units::Piece{
            Rank:units::Ranks::Pawn,
            Color:units::Color::Black,
        };
        let mut correct_board:[[board::Square;8];8];

      
    }
}

pub mod units {
    #[derive(Debug, Copy, Clone)]
    pub struct Piece{
        pub Rank:Ranks,
        pub Color:Color
    }
    #[derive(Debug, Copy, Clone)]
    pub enum Ranks {
        Empty,
        Pawn,
        Rook,
        Knight,
        Bishop,
        Queen,
        King,
    }
    #[derive(Debug, Copy, Clone)]
    pub enum Color {
        Empty,
        White,
        Black,
    }
}

pub mod board {
    use super::units;
    use super::readPGN;
    use std::fmt;
    fn init() {}

    #[derive(Debug, Copy, Clone)]
    pub struct Square {
        pub Piece: units::Piece,
  
    }

    impl Square {
        fn is_empty(&self) -> bool {
            if let units::Ranks::Empty = self.Piece.Rank {
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

            write!(f, "{}", self.Piece.Rank as i32)
        }
    }
    #[derive(Debug, Clone, Copy)]
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

    fn IndexToRank(index:usize)-> units::Ranks{
        match index {
            index if index==units::Ranks::Empty as usize => units::Ranks::Empty,
            index if index==units::Ranks::Pawn as usize => units::Ranks::Pawn,
            index if index==units::Ranks::Rook as usize => units::Ranks::Rook,
            index if index==units::Ranks::Knight as usize => units::Ranks::Knight,
            index if index==units::Ranks::Bishop as usize => units::Ranks::Bishop,
            index if index==units::Ranks::Queen as usize => units::Ranks::Queen,
            index if index==units::Ranks::King as usize => units::Ranks::King,
            _ => panic!("There is no Rank")
        }
    }

    pub fn new(config_file:&str) -> Board{
        let mut board=Board{
            matrix:readPGN::load_from_PGN(config_file),
        };
        
        board
    }
        
}

mod readPGN{
    use super::units;
    use super::board;
    use std::fs;
    use std::error::Error;
    pub fn load_from_PGN(path:&str) -> [[board::Square;8];8]{

        let data = fs::read_to_string(path).expect("Failed to read file");     
        let split_data: Vec<Vec<&str>>= data.lines().map(|line|{
            line.split_whitespace().collect()
        }).collect();
        let empty_square= board::Square{
            Piece:units::Piece{ 
                Rank: units::Ranks::Empty,
                Color: units::Color::Empty,
            }
        };
        
        let mut matrix:[[board::Square;8];8]=[[empty_square;8];8];

        for (i,line) in split_data.iter().enumerate(){
            for (j,pair) in line.iter().enumerate(){
                matrix[i][j]=board::Square{
                    Piece: map_pgn_to_Piece(pair)
                }
            }
        }
        return matrix;
    }

    fn map_pgn_to_Piece(letterPair:&str) -> units::Piece{
        let rankLetter= letterPair.chars().next().expect("not a pair");
        let colorLetter=letterPair.chars().next().expect("not a pair");
        let mut color: units::Color;
        let mut rank: units::Ranks;
        if colorLetter== 'B'{
            color=units::Color::Black;
        }
        else if colorLetter == 'W'{
            color= units::Color::White;
        }
        else {
            color= units::Color::Empty;
        }

        if rankLetter =='X'{
            rank=units::Ranks::Empty;
        }
        else if rankLetter == 'P'{
            rank=units::Ranks::Pawn;
        }

        else if rankLetter == 'R'{
            rank=units::Ranks::Rook;
        }    
        else if rankLetter == 'N'{
            rank=units::Ranks::Knight;
        }    
        else if rankLetter == 'B'{
            rank=units::Ranks::Bishop;
        }    
        else if rankLetter == 'Q'{
            rank=units::Ranks::Queen;
        }    
        else if rankLetter == 'K'{
            rank=units::Ranks::King;
        }
        else{
            rank=units::Ranks::Empty;
        }
        
        units::Piece{
            Rank:rank,
            Color: color,
        }
    }


    fn board_from_config(config: String) {
        
    }
}
