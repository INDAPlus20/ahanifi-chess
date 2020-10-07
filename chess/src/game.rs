#[warn(dead_code)]
use std::io::prelude::*;
use std::{fmt, io};

use crate::moves;
use crate::moves::{Action, ActionType};

#[derive(Debug, Clone)]
pub struct Game {
    gamestate: GameState,
    pub matrix: [[Square; 8]; 8],
    pub player: Team,
    pub history: Vec<Action>,
    pub white_king_square: Square,
    pub black_king_square: Square,
    fifty_turn: i32,
}

impl Game {
    pub fn new() -> Game {
        let init_state = "RB NB BB QB KB BB NB RB
                PB PB PB PB PB PB PB PB
                XX XX XX XX XX XX XX XX
                XX XX XX XX XX XX XX XX
                XX XX XX XX XX XX XX XX
                XX XX XX XX XX XX XX XX
                PW PW PW PW PW PW PW PW
                RW NW BW QW KW BW NW RW";

        Game::blockstate_to_board(init_state)
    }
    //for testing
    pub fn game_from_blockstate(blocks: &str) -> Game {
        Game::blockstate_to_board(blocks)
        
    }

    pub fn main(&mut self) {
        let mut error_msg = String::new();
        let mut turns_for_50 = 0;

        loop {
            println!("{}", self);
            let king_square = match self.player {
                Team::White => self.white_king_square,
                Team::Black => self.black_king_square,
            };

            if !self.is_more_moves() {
                if self.check_square_attacked(king_square) {
                    println!("Checkmate");
                    self.gamestate = GameState::Checkmate;
                    break;
                } else {
                    println!("Stalemate");
                    self.gamestate = GameState::Stalemate;
                    break;
                }
            } else if self.check_square_attacked(king_square) {
                println!("Check")
            }
            
            if !error_msg.is_empty() {
                println!();
                println!("{}", error_msg);
            }
            println!("Generate moves from square: ");
            let input = io::stdin().lock().lines().next().unwrap().unwrap();
            let moves = match self.move_from_string(&input) {
                Ok(a) => a,
                Err(s) => {
                    error_msg = s;
                    continue;
                }
            };
            if moves.is_empty() {
                error_msg = String::from("there are no moves for this piece");
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

            self.perform_action(moves[input_index]);

            if self.history.last().unwrap().to.piece.is_some() {
                turns_for_50 = 0;
            } else if self.history.last().unwrap().from.piece.is_some() {
                if self.history.last().unwrap().from.piece.unwrap().rank == Rank::Pawn {
                    turns_for_50 = 0;
                }
            } else {
                turns_for_50 += 1;
            }
            if turns_for_50 / 2 == 100 {
                println!("50-rule draw");
                self.gamestate = GameState::FiftyRule;
                break;
            }

            error_msg = String::from("");
            print!("\x1B[2J\x1B[1;1H"); // Clears terminal screen
            println!("{:?}", moves[input_index]);
        }
        println!("{:?}", self.gamestate);
    }

    pub fn perform_action(&mut self, action: Action) {
        self.history.push(action);
        let coordinate_from = action.from.coordinate;
        let coordinate_to = action.to.coordinate;

        match action.action_type {
            ActionType::Promotion => {
                self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize].piece = None;

                let promotion_piece = Piece {
                    rank: Game::prompt_promotion(),
                    team: self.player,
                };
                self.matrix[coordinate_to.0 as usize][coordinate_to.1 as usize].piece =
                    Some(promotion_piece);
                println!("Promotion")
            }
            _ => self.make_move(&action),
        }
        self.player = next_player(self.player);
        self.calculate_game_state();
    }

    fn prompt_promotion() -> Rank {
        println!("What unit to you want to promote to");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let rank = match input.trim() {
            "q" => Rank::Queen,
            "r" => Rank::Rook,
            "b" => Rank::Bishop,
            "kn" => Rank::Knight,
            _ => panic!("There are no such unit"),
        };
        rank
    }

    pub fn move_from_string(&mut self, letter_coordinate: &str) -> Result<Vec<Action>, String> {
        let square = match self.square_from_string(letter_coordinate) {
            Ok(s) => s,
            Err(s) => return Err(s),
        };
        let moveset = match moves::generate_moves(self, square) {
            Ok(a) => a,
            Err(s) => return Err(s),
        };

        for (index, action) in moveset.iter().enumerate() {
            let letter_coordinate = coordinate_to_string(action.to.coordinate);
            println!("{}. {}", index, letter_coordinate);
        }
        Ok(moveset)
    }

    pub fn check(&mut self, action: &Action) -> bool {
        let mut testing_game = self.clone();
        testing_game.make_move(action);
        let king_square = match self.player {
            Team::White => testing_game.white_king_square,
            Team::Black => testing_game.black_king_square,
        };
        if testing_game.check_square_attacked(king_square) {
            true
        } else {
            false
        }
    }

    pub fn all_moves(&mut self) -> Vec<Action> {
        let mut all_moves: Vec<Action> = vec![];
        let matrix = self.matrix;
        for row in matrix.iter() {
            for square in row.iter() {
                let gen_moveset = moves::generate_moves(self, *square);
                if let Ok(mut a) = gen_moveset {
                    all_moves.append(&mut a);
                }
            }
        }
        all_moves
    }

    fn is_more_moves(&mut self) -> bool {
        let all_moves: Vec<Action> = self.all_moves();

        !all_moves.is_empty()
    }

    fn make_move(&mut self, action: &Action) {
        let coordinate_from = action.from.coordinate;
        let coordinate_to = action.to.coordinate;
        let moving_piece =
            self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize].piece;

        match action.action_type {
            ActionType::Regular => {
                if action.from.piece.unwrap().rank == Rank::King {
                    match self.player {
                        Team::White => {
                            self.white_king_square =
                                self.matrix[coordinate_to.0 as usize][coordinate_to.1 as usize]
                        }
                        Team::Black => {
                            self.black_king_square =
                                self.matrix[coordinate_to.0 as usize][coordinate_to.1 as usize]
                        }
                    }
                }
                self.matrix[coordinate_to.0 as usize][coordinate_to.1 as usize].piece =
                    moving_piece;
                self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize].piece = None;
            }
            ActionType::Enpassant => {
                let team_offset = match self.player {
                    Team::White => 1,
                    Team::Black => -1,
                };
                self.matrix[coordinate_to.0 as usize][coordinate_to.1 as usize].piece =
                    moving_piece;
                self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize].piece = None;
                self.matrix[coordinate_to.0 as usize][(coordinate_to.1 - team_offset) as usize]
                    .piece = None;
            }
            ActionType::Promotion => {
                self.matrix[coordinate_to.0 as usize][coordinate_to.1 as usize].piece =
                    moving_piece;
                self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize].piece = None;
            }
            ActionType::Castling => {
                self.matrix[coordinate_to.0 as usize][coordinate_to.1 as usize].piece =
                    moving_piece;
                self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize].piece = None;

                match coordinate_to.0 {
                    x if x > coordinate_from.0 => {
                        self.matrix[(coordinate_from.0 + 1) as usize][coordinate_from.1 as usize]
                            .piece = self.matrix[7][coordinate_from.1 as usize].piece;
                        self.matrix[7][coordinate_from.1 as usize].piece = None;
                        match self.player {
                            Team::White => {
                                self.white_king_square =
                                    self.matrix[coordinate_to.0 as usize][coordinate_to.1 as usize]
                            }
                            Team::Black => {
                                self.black_king_square =
                                    self.matrix[coordinate_to.0 as usize][coordinate_to.1 as usize]
                            }
                        }
                    }
                    x if x < coordinate_from.0 => {
                        self.matrix[(coordinate_from.0 - 1) as usize][coordinate_from.1 as usize]
                            .piece = self.matrix[0][coordinate_from.1 as usize].piece;
                        self.matrix[0][coordinate_from.1 as usize].piece = None;
                        if action.from.piece.unwrap().rank == Rank::King {
                            match self.player {
                                Team::White => {
                                    self.white_king_square =
                                        self.matrix[coordinate_to.0 as usize][coordinate_to.1 as usize]
                                }
                                Team::Black => {
                                    self.black_king_square =
                                        self.matrix[coordinate_to.0 as usize][coordinate_to.1 as usize]
                                }
                            }
                        }
                    }
                    _ => panic!("cant castle to there"),
                }
            } // already checking for check before adding move
        }
    }
    //there is an error with bishop undo, 
    fn undo_move(&mut self, action: &Action) {
        let coordinate_from = action.from.coordinate;
        let coordinate_to = action.to.coordinate;
        match action.action_type {
            ActionType::Regular => {
                if action.from.piece.unwrap().rank == Rank::King {
                    match self.player {
                        Team::White => {
                            self.white_king_square =
                                self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize]
                        }
                        Team::Black => {
                            self.black_king_square =
                                self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize]
                        }
                    }
                }
                self.matrix[coordinate_to.0 as usize][coordinate_to.1 as usize].piece =
                    action.to.piece;
                self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize].piece =
                    action.from.piece;
            }
            ActionType::Enpassant => {
                let (team_offset, other_player) = match self.player {
                    Team::White => (1, Team::Black),
                    Team::Black => (-1, Team::White),
                };
                self.matrix[coordinate_to.0 as usize][coordinate_to.1 as usize].piece =
                    action.to.piece;
                self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize].piece =
                    action.from.piece;
                self.matrix[coordinate_to.0 as usize][(coordinate_to.1 - team_offset) as usize]
                    .piece = Some(Piece {
                    rank: Rank::Pawn,
                    team: other_player,
                })
            }
            ActionType::Castling => {
                self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize].piece =
                    action.from.piece;
                match self.player {
                    Team::White => {
                        self.white_king_square =
                            self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize]
                    }
                    Team::Black => {
                        self.black_king_square =
                            self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize]
                    }
                }

                match coordinate_to.0 {
                    x if x > coordinate_from.0 => {
                        self.matrix[(coordinate_from.0 + 1) as usize][coordinate_from.1 as usize]
                            .piece = None;
                        self.matrix[(coordinate_from.0 + 2) as usize][coordinate_from.1 as usize]
                            .piece = None;
                        self.matrix[7][coordinate_from.1 as usize].piece = Some(Piece {
                            rank: Rank::Rook,
                            team: self.player,
                        });
                    }
                    x if x < coordinate_from.0 => {
                        self.matrix[(coordinate_from.0 - 1) as usize][coordinate_from.1 as usize]
                            .piece = None;
                        self.matrix[(coordinate_from.0 - 2) as usize][coordinate_from.1 as usize]
                            .piece = None;
                        self.matrix[0][coordinate_from.1 as usize].piece = Some(Piece {
                            rank: Rank::Rook,
                            team: self.player,
                        });
                    }
                    _ => panic!("cant castle to kings own position"),
                }
            }
            ActionType::Promotion => {
                self.matrix[coordinate_to.0 as usize][coordinate_to.1 as usize].piece =
                    action.to.piece;
                self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize].piece =
                    action.from.piece;
            }
        }
    }

    pub fn check_square_attacked(&self, square: Square) -> bool {
        let is_attacked: bool = false;

        for a in moves::gen_moveset_bishop(self, square).iter() {
            if a.to.piece.is_some() && a.to.piece.unwrap().rank == Rank::Bishop {
               
                return true;
            }
        }
        for a in moves::gen_moveset_rook(self, square).iter() {
            if a.to.piece.is_some() && a.to.piece.unwrap().rank == Rank::Rook {
                return true;
            }
        }
        for a in moves::gen_moveset_queen(self, square).iter() {
            if a.to.piece.is_some() && a.to.piece.unwrap().rank == Rank::Queen {
                return true;
            }
        }
        for a in moves::gen_moveset_knight(self, square).iter() {
            if a.to.piece.is_some() && a.to.piece.unwrap().rank == Rank::Knight {
                return true;
            }
        }
        for a in moves::gen_pawn_attack_moveset(self, square).iter() {
            if a.to.piece.is_some() && a.to.piece.unwrap().rank == Rank::Pawn {
                return true;
            }
        }
        is_attacked
    }

    fn block_to_piece(block: &str) -> Option<Piece> {
        let rank_letter = block.chars().next().expect("not a pair");
        let team_letter = block.chars().nth(1).expect("not a pair");

        if block == "XX" {
            return None;
        }
        let rank = match rank_letter {
            'P' => Rank::Pawn,
            'R' => Rank::Rook,
            'N' => Rank::Knight,
            'B' => Rank::Bishop,
            'Q' => Rank::Queen,
            'K' => Rank::King,
            _ => panic!("Piece letter not valid"),
        };
        let team = match team_letter {
            'W' => Team::White,
            'B' => Team::Black,
            _ => panic!("Team letter not valid"),
        };

        let piece = Piece { rank, team };

        Some(piece)
    }

    fn blockstate_to_board(blocks: &str) -> Game {
        let placeholder_square = Square {
            //TODO fix array initialization
            piece: None,
            coordinate: (-1, -1),
        };

        let mut matrix = [[placeholder_square; 8]; 8];

        let blockstates: Vec<&str> = blocks.trim().split_whitespace().collect();
        let mut pieces: Vec<Option<Piece>> = vec![];
        for block in blockstates {
            let piece = Game::block_to_piece(block);
            pieces.push(piece);
        }

        let mut white_king_square: Square = placeholder_square;
        let mut black_king_square: Square = placeholder_square;

        for row in 0..8 {
            for column in 0..8 {
                let current_piece = pieces[8 * (7 - row) + column];
                let this_square: Square = Square {
                    piece: current_piece,
                    coordinate: (column as isize, row as isize),
                };
                if current_piece.is_some() && current_piece.unwrap().rank == Rank::King {
                    match current_piece.unwrap().team {
                        Team::White => white_king_square = this_square,
                        Team::Black => black_king_square = this_square,
                    }
                }
                matrix[column][row] = this_square;
            }
        }
        
        let mut game=Game {
            gamestate: GameState::Active,
            history: vec![],
            player: Team::White,
            matrix,
            black_king_square,
            white_king_square,
            fifty_turn: 0,
        };
        game.calculate_game_state();
        game

    }

    pub fn square_from_string(&self, letter_coordinate: &str) -> Result<Square, String> {
        let coordinate = match coordinate_from_string(letter_coordinate) {
            Err(e) => return Err(e),
            Ok(c) => c,
        };
        let square = Square {
            coordinate,
            piece: self.matrix[coordinate.0 as usize][coordinate.1 as usize].piece,
        };
        Ok(square)
    }

    pub fn square_to_string(&self, square: Square) -> String {
        let coordinate = square.coordinate;
        coordinate_to_string(coordinate)
    }

   pub fn calculate_game_state (&mut self) ->GameState{
        let king_square = match self.player {
            Team::White => self.white_king_square,
            Team::Black => self.black_king_square,
        };

        let is_check=self.check_square_attacked(king_square);

        if !self.is_more_moves() {
            if is_check {
                println!("Checkmate");
                self.gamestate=GameState::Checkmate;
                return GameState::Checkmate;
            } else {
                println!("Stalemate");
                self.gamestate=GameState::Stalemate;
                return GameState::Stalemate;
            }
        }
        if is_check{
            println!("Check");
            self.gamestate=GameState::Check;
            return GameState::Check;
        }

        if  !self.history.is_empty() &&self.history.last().unwrap().to.piece.is_some() {
            self.fifty_turn = 0;
        } else if  !self.history.is_empty()&&self.history.last().unwrap().from.piece.is_some() {
            if self.history.last().unwrap().from.piece.unwrap().rank == Rank::Pawn {
                self.fifty_turn = 0;
            }
        } else {
            self.fifty_turn += 1;
        }
        if self.fifty_turn / 2 == 100 {
            println!("50-rule draw");
            self.gamestate=GameState::FiftyRule;
            return GameState::FiftyRule;
        }
        self.gamestate=GameState::Active;
        GameState::Active
    }
    
    
    pub fn get_game_state(&mut self) -> GameState {
        self.gamestate
    }
        
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string_to_print = String::new();
        string_to_print.push_str("A B C D E F G H \n");
        for row in (0..8).rev() {
            for column in 0..8 {
                let square = self.matrix[column][row];
                string_to_print.push_str(&format!("{} ", square));
            }
            string_to_print.push_str(&format!(" {} \n", row + 1));
        }
        write!(f, "{}", string_to_print)
    }
}
impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

pub fn coordinate_to_string(coordinate: (isize, isize)) -> String {
    let row_letter: String = (coordinate.1 + 1).to_string();
    let column_number = coordinate.0 + 1;
    let column_letter = match column_number {
        1 => "a",
        2 => "b",
        3 => "c",
        4 => "d",
        5 => "e",
        6 => "f",
        7 => "g",
        8 => "h",
        _ => panic!("there shouldnt be a out of bounds letter here"),
    };

    String::from(column_letter) + &row_letter
}

pub fn coordinate_from_string(letter_coordinate: &str) -> Result<(isize, isize), String> {
    if letter_coordinate.len() != 2 {
        return Result::Err(String::from("Coordinate wasnt in correct format"));
    }
    let column_letter = letter_coordinate
        .chars()
        .next()
        .unwrap()
        .to_ascii_lowercase();
    let row = match letter_coordinate.chars().nth(1).unwrap().to_digit(10) {
        Some(d) => d as isize,
        None => return Err(String::from("row digit wasnt in correct format")),
    };

    let column = match column_letter {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => return Result::Err(String::from("first letter doesnt correspond to a column")),
    };

    if row < 1 || row > 8 {
        return Result::Err(String::from("Row index out of bounds"));
    }
    Ok((column, row - 1))
}

pub fn unmoved(game: &Game, from_square: Square) -> bool {
    match from_square.piece.unwrap().rank {
        Rank::Pawn => match from_square.piece.unwrap().team {
            Team::White => from_square.coordinate.1 == 1,
            Team::Black => from_square.coordinate.1 == 6,
        },
        Rank::Rook => {
            for action in game.history.iter() {
                if action.from.coordinate == from_square.coordinate {
                    return false;
                }
            }
            true
        }
        _ => {
            for action in game.history.iter() {
                if action.from.coordinate == from_square.coordinate {
                    return false;
                }
            }
            true
        }
    }
}

pub fn not_same_team(team: Team, square: Square) -> bool {
    if square.piece.is_some() && square.piece.unwrap().team != team {
        return true;
    }
    false
}

pub fn not_out_of_bounds(x: isize, y: isize) -> bool {
    !(x < 0 || x > 7 || y < 0 || y > 7)
}
fn next_player(team: Team) -> Team {
    if team == Team::White {
        Team::Black
    } else {
        Team::White
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Square {
    pub piece: Option<Piece>,
    pub coordinate: (isize, isize),
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.piece {
            None => write!(f, "."),
            Some(p) => write!(f, "{}", p.unicode()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Team {
    White,
    Black,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Rank {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GameState {
    Active,
    Check,
    Checkmate,
    Stalemate,
    FiftyRule,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Piece {
    pub team: Team,
    pub rank: Rank,
}

impl Piece {
    pub fn unicode(&self) -> &str {
        let team_index: usize = self.team_to_int() - 1;
        match self.rank {
            Rank::Pawn => ["♟︎", "♙"][team_index],
            Rank::Rook => ["♜", "♖"][team_index],
            Rank::Knight => ["♞", "♘"][team_index],
            Rank::Bishop => ["♝", "♗"][team_index],
            Rank::Queen => ["♛", "♕"][team_index],
            Rank::King => ["♚", "♔"][team_index],
        }
    }
    fn team_to_int(&self) -> usize {
        match self.team {
            Team::White => 1,
            Team::Black => 2,
        }
    }
}
