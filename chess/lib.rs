#![allow(dead_code)]
#![allow(dead_code)]

use std;
use std::io;
use std::fmt;
use std::io::prelude::*;

pub struct Game {
    pub matrix: [[Square; 8]; 8],
    player: Team,
    history: Vec<Action>,
    white_king_square:Square,
    black_king_square:Square,

}

impl Game {
    pub fn new() -> Game {
        let init_state: Vec<&str> = ("RB NB BB KB QB BB NB RB
            PB PB PB PB PB PB PB PB
            XX XX XX XX XX XX XX XX
            XX XX XX XX XX XX XX XX
            XX XX XX XX XX XX XX XX
            XX XX XX XX XX XX XX XX
            PW PW PW PW PW PW PW PW
            RW NW BW KW QW BW NW RW")
            .trim()
            .split_whitespace()
            .rev()
            .collect();

        let placeholder_square = Square {
            //TODO fix array initialization
            piece: None,
            coordinate: (-1, -1),
        };
        let mut empty_matrix: [[Square; 8]; 8] = [[placeholder_square; 8]; 8];
        let mut pieces: Vec<Option<Piece>> = vec![];

        for block in init_state {
            let piece = Game::block_to_piece(block);
            pieces.push(piece);
        }
        let mut counter = 0;
        for row in 0..8 {
            for column in 0..8 {
                let square: Square = Square {
                    piece: pieces[counter as usize],
                    coordinate: (column, row),
                };

                empty_matrix[column as usize][row as usize] = square;
                counter += 1;
            }
        }
        let white_king_square=empty_matrix[4][0];
        let black_king_square=empty_matrix[4][7];
        Game {
            history: vec![],
            player: Team::White,
            matrix: empty_matrix,
            white_king_square:white_king_square,
            black_king_square:black_king_square,
        }
    }
    //for testing 
    pub fn new_empty() ->Game{
        let init_state: Vec<&str> = ("XX XX XX KB XX XX XX XX
            XX XX XX XX XX XX XX XX
            XX XX XX XX XX XX XX XX
            XX XX XX XX XX XX XX XX
            XX XX XX XX XX XX XX XX
            XX XX XX XX XX XX XX XX
            XX XX XX XX XX XX XX XX
            XX XX XX KW XX XX XX XX")
            .trim()
            .split_whitespace()
            .rev()
            .collect();

        let placeholder_square = Square {
            //TODO fix array initialization
            piece: None,
            coordinate: (-1, -1),
        };
        let mut empty_matrix: [[Square; 8]; 8] = [[placeholder_square; 8]; 8];
        let mut pieces: Vec<Option<Piece>> = vec![];

        for block in init_state {
            let piece = Game::block_to_piece(block);
            pieces.push(piece);
        }
        let mut counter = 0;
        for row in 0..8 {
            for column in 0..8 {
                let square: Square = Square {
                    piece: pieces[counter as usize],
                    coordinate: (column, row),
                };

                empty_matrix[column as usize][row as usize] = square;
                counter += 1;
            }
        }
        let white_king_square=empty_matrix[4][0];
        let black_king_square=empty_matrix[4][7];
        Game {
            history: vec![],
            player: Team::White,
            matrix: empty_matrix,
            white_king_square:white_king_square,
            black_king_square:black_king_square,
        }
    }

    pub fn main(&mut self){
        let mut error_msg=String::new();
        let mut turns_for_50=0;
        loop {
            println!("{}", self);
            let king_square=match self.player {
                Team::White=>self.white_king_square,
                Team::Black=> self.black_king_square,
            };
            if self.check_square_attacked(king_square){
   
            }

            if !self.is_more_moves(){
                if self.check_square_attacked(king_square){
                    println!("Checkmate")
                }
                else{
                    println!("Stalemate")
                }
            }else{
                if self.check_square_attacked(king_square){
                    println!("Check")
                } 
            }

            if !error_msg.is_empty() {
                println!("");
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
            self.perform_action(moves[input_index]);

            if self.history.last().unwrap().to.piece.is_some(){
                turns_for_50=0;
            }
            else if self.history.last().unwrap().from.piece.is_some(){
                if self.history.last().unwrap().from.piece.unwrap().rank==Rank::Pawn{
                    turns_for_50=0;
                }
            }
            else{
                turns_for_50+=1;
            }
            if turns_for_50/2 == 100 {
                println!("50-rule draw")
            }
            
            error_msg = String::from("");
            print!("\x1B[2J\x1B[1;1H"); // Clears terminal screen
            
            self.player = next_player(self.player)
        }
    }
    
    pub fn perform_action(&mut self, action: Action) {
      
        self.history.push(action);
        let coordinate_from = action.from.coordinate;
        let coordinate_to = action.to.coordinate;

        match action.action_type {
            ActionType::Promotion=> {
                self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize].piece = None;

                let promotion_piece=Piece{
                    rank: Game::prompt_promotion(),
                    team:self.player,
                };
                self.matrix[coordinate_to.0 as usize][coordinate_to.1 as usize].piece =
                    Some(promotion_piece);   
                println!("Promotion")     
            }
            _ => self.make_move(&action),
        }

 
    }

    fn prompt_promotion() -> Rank{

        println!("What unit to you want to promote to");
        let mut input=String::new();
        io::stdin().read_line(&mut input).unwrap();

        let rank=match input.trim() {
            "q" => Rank::Queen,
            "r" => Rank::Rook,
            "b" => Rank::Bishop,
            "kn" => Rank::Knight,
            _ => panic!("There are no such unit")

        };
        return rank
    }

    fn block_to_piece(block: &str) -> Option<Piece> {
        let rank_letter = block.chars().nth(0).expect("not a pair");
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

        let piece = Piece {
            rank: rank,
            team: team,
        };

        Some(piece)
    }

    pub fn move_from_string(&mut self, letter_coordinate: &str) -> Result<Vec<Action>, String> {
        let square = match self.square_from_string(letter_coordinate) {
            Ok(s) => s,
            Err(s) => return Err(s),
        };
        let moveset = match self.generate_moves(square) {
            Ok(a) => a,
            Err(s) => return Err(s),
        };

        let mut index = 0;
        for action in &moveset {
            let letter_coordinate = Game::coordinate_to_string(action.to.coordinate);
            println!("{}. {}", index, letter_coordinate);
            index += 1;
        }
        Ok(moveset)
    }

    pub fn move_string(&mut self, letter_coordinate_from: &str, letter_coordinate_to: &str) {
        let square = self.square_from_string(letter_coordinate_from).unwrap();
        let coordinate_from = square.coordinate;
        let moveset = self.generate_moves(square).unwrap();
        let coordinate_to = Game::coordinate_from_string(letter_coordinate_to).unwrap();
        if moveset.len() > 0 {
            for action in moveset {
                println!("{}", Game::square_to_string(&self, action.to));
                if action.to.coordinate == coordinate_to {
                    self.matrix[coordinate_to.0 as usize][coordinate_to.1 as usize].piece =
                        square.piece;
                    self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize].piece =
                        None;
                    self.history.push(action);
                } else {
                    println!("cant move there")
                }
            }
        } else {
            println!("no moves for this piece")
        }
    }

    fn generate_moves(&mut self, square: Square) -> Result<Vec<Action>, String> {
        let rank = match square.piece {
            Some(p) => p.rank,
            None => return Err(String::from("Tried to move empty square")),
        };
        let team = square.piece.unwrap().team;
        if team != self.player {
            return Err(String::from("Cant move enemy piece"));
        };

        let moveset: Vec<Action> = match rank {
            Rank::Pawn => self.gen_moveset_pawn(square),
            Rank::Rook => self.gen_moveset_rook(square),
            Rank::Knight => self.gen_moveset_knight(square),
            Rank::Bishop => self.gen_moveset_bishop(square),
            Rank::Queen => self.gen_moveset_queen(square),
            Rank::King => self.gen_moveset_king(square),
        };

        let mut legal_moveset:Vec<Action>=vec![];

        for action in &moveset{
            if self.check(action)==false{
                legal_moveset.push(*action);
            }
        }
        Ok(legal_moveset)
    }

    fn check(&mut self,action:&Action)-> bool{
        
        
        self.make_move(action);
        let king_square=match self.player{
            Team::White => self.white_king_square,
            Team::Black => self.black_king_square,
        };
        if self.check_square_attacked(king_square){
            self.undo_move(action);
            return true;
        }
        else{
            self.undo_move(action);
            false
        }
        
    }

    fn is_more_moves(&mut self)->bool{
        let mut all_moves:Vec<Action>=vec![];
        let matrix=self.matrix;
        for row in matrix.iter(){
            for square in row.iter(){
                let gen_moveset=self.generate_moves(*square);
                if gen_moveset.is_ok(){
                    all_moves.append(&mut gen_moveset.unwrap());
                }
            }
        }
        if all_moves.len()==0{
            
            return false
        }
        else {
            
            return true;
        }
    }
    
    fn make_move(&mut self, action:&Action){
        let coordinate_from=action.from.coordinate;
        let coordinate_to=action.to.coordinate;
        let moving_piece =  self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize].piece;
        
        match action.action_type {
            ActionType::Regular=>{
                if action.from.piece.unwrap().rank== Rank::King{
                    match self.player{
                        Team::White => self.white_king_square=self.matrix[coordinate_to.0 as usize][coordinate_to.1 as usize],
                        Team::Black => self.black_king_square=self.matrix[coordinate_to.0 as usize][coordinate_to.1 as usize],
                    }
                }
                self.matrix[coordinate_to.0 as usize][coordinate_to.1 as usize].piece=moving_piece;
                self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize].piece=None;
                
            }
            ActionType::Enpassant =>{
                let team_offset = match self.player {
                    Team::White => 1,
                    Team::Black => -1,
                };
                self.matrix[coordinate_to.0 as usize][coordinate_to.1 as usize].piece =moving_piece;
                self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize].piece = None;
                self.matrix[coordinate_to.0 as usize][(coordinate_to.1 - team_offset) as usize]
                    .piece = None;
            }
            ActionType::Promotion=>{
                self.matrix[coordinate_to.0 as usize][coordinate_to.1 as usize].piece=moving_piece;
                self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize].piece=None;
            }
            ActionType::Castling=>{
                self.matrix[coordinate_to.0 as usize][coordinate_to.1 as usize].piece=moving_piece;
                self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize].piece=None;

                match coordinate_to.0{
                    x if x>coordinate_from.0 =>{
                        
                        self.matrix[(coordinate_from.0+1)as usize][coordinate_from.1 as usize].piece=self.matrix[7][coordinate_from.1 as usize].piece;
                        self.matrix[7][coordinate_from.1 as usize].piece=None;
                            match self.player{
                                Team::White => self.white_king_square=self.matrix[coordinate_to.0 as usize][coordinate_to.1 as usize],
                                Team::Black => self.black_king_square=self.matrix[coordinate_to.0 as usize][coordinate_to.1 as usize],
                            
                        }
                    }
                    x if x<coordinate_from.0 =>{
                        
                        self.matrix[(coordinate_from.0-1)as usize][coordinate_from.1 as usize].piece=self.matrix[0][coordinate_from.1 as usize].piece;
                        self.matrix[0][coordinate_from.1 as usize].piece=None;
                    }
                    _ => panic!("cant castle to there")
                }
            }// already checking for check before adding move
        }
    }
    
    fn undo_move(&mut self, action: &Action){
        let coordinate_from=action.from.coordinate;
        let coordinate_to=action.to.coordinate;
        match action.action_type {
            ActionType::Regular=>{
                
                if action.from.piece.unwrap().rank== Rank::King{
                    match self.player{
                        Team::White => self.white_king_square=self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize],
                        Team::Black => self.black_king_square=self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize],
                    }
                }
                self.matrix[coordinate_to.0 as usize][coordinate_to.1 as usize].piece=action.to.piece;
                self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize].piece=action.from.piece;
     
            }
            ActionType::Enpassant =>{
                let (team_offset,other_player) = match self.player {
                    Team::White => (1,Team::Black),
                    Team::Black => (-1,Team::White),

                };
                self.matrix[coordinate_to.0 as usize][coordinate_to.1 as usize].piece =action.to.piece;
                self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize].piece = action.from.piece;
                self.matrix[coordinate_to.0 as usize][(coordinate_to.1 - team_offset) as usize].piece=Some(Piece{
                    rank:Rank::Pawn,
                    team:other_player,
                })
            }
            ActionType::Castling=>{
                self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize].piece=action.from.piece;
                match self.player{
                    Team::White => self.white_king_square=self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize],
                    Team::Black => self.black_king_square=self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize],
                }

                match coordinate_to.0{
                    x if x>coordinate_from.0 =>{
                        
                        self.matrix[(coordinate_from.0+1)as usize][coordinate_from.1 as usize].piece=None;
                        self.matrix[7][coordinate_from.1 as usize].piece=Some(Piece{rank:Rank::Rook,team:self.player});
                    }
                    x if x<coordinate_from.0 =>{
                        
                        self.matrix[(coordinate_from.0-1)as usize][coordinate_from.1 as usize].piece=None;
                        self.matrix[0][coordinate_from.1 as usize].piece=Some(Piece{rank:Rank::Rook,team:self.player});
                    }
                    _ => panic!("cant castle to kings own position"),
                }
            }
            ActionType::Promotion => {
                self.matrix[coordinate_to.0 as usize][coordinate_to.1 as usize].piece=action.to.piece;
                self.matrix[coordinate_from.0 as usize][coordinate_from.1 as usize].piece=action.from.piece;
            }
            
        }
        
    }

    fn gen_moveset_pawn(&self, start_square: Square) -> Vec<Action> {
        let mut available_moves = Vec::<Action>::new();
        //let coordinate:(usize,usize)=(square.coordinate.0.try_into().unwrap(),square.coordinate.1.try_into().unwrap());
        let(x,y)=(start_square.coordinate.0,start_square.coordinate.1);
        let (offset,promotion_row) = match self.player {
            Team::White => (1,7),
            Team::Black => (-1,0),
        };
        let new_coordinate_y = y + offset;
        let new_square = self.matrix[x as usize][new_coordinate_y as usize];
        if new_square.piece.is_none() {
            let mut action_type=ActionType::Regular;
            if new_coordinate_y==promotion_row{
                action_type=ActionType::Promotion;
            }
            let action = Action {
                from: start_square,
                to: new_square,
                action_type,
            };
            available_moves.push(action);
        }

        if self.unmoved(start_square) {
            let new_coordinate_y = y + 2 * offset;
            if not_out_of_bounds(x, new_coordinate_y) {
                let new_square = self.matrix[x as usize][new_coordinate_y as usize];
                if new_square.piece.is_none() {
                    let action = Action {
                        from: start_square,
                        to: new_square,
                        action_type: ActionType::Regular,
                    };
                    available_moves.push(action);
                }
            }
        }

        available_moves.append(&mut self.gen_pawn_attack_moveset(start_square));
 

        let mut prev_action: &Action;

        //Enpassant
        for dx in (-1..2).step_by(2) {
            if not_out_of_bounds(x + dx, y) {
                let side_square = self.matrix[(x + dx) as usize][y as usize];
                if not_same_team(self.player, side_square) {
                    if side_square.piece.unwrap().rank == Rank::Pawn {
                        if !self.history.is_empty(){
                            prev_action = self.history.last().unwrap();
                            if prev_action.from.piece.unwrap().rank==Rank::Pawn{
                                let enemy_start_y = prev_action.from.coordinate.1;
                                let dy = (y - enemy_start_y).abs();
                                println!("{}", dy);
                                if dy == 2 {
                                    let action = Action {
                                        from: start_square,
                                        to: self.matrix[(x + dx) as usize][(y + offset) as usize],
                                        action_type: ActionType::Enpassant,
                                    };
                                    available_moves.push(action);
                                }
                            }
                            
                        }
                        
                    }
                }
            }
        }

        available_moves
    }

    fn gen_moveset_rook(&self, start_square: Square) -> Vec<Action> {
        let max_one = false;
        let can_jump = false;
        let gen_moveset = self.straight_move(start_square, max_one, can_jump);
        gen_moveset
    }

    fn gen_moveset_bishop(&self, start_square: Square) -> Vec<Action> {
        let max_one = false;
        let can_jump = false;
        let gen_moveset = self.diagonal_move(start_square, max_one, can_jump);
        gen_moveset
    }

    fn gen_moveset_queen(&self, start_square: Square) -> Vec<Action> {
        let max_one = false;
        let can_jump = false;
        let mut gen_moveset = self.diagonal_move(start_square, max_one, can_jump);
        gen_moveset.append(&mut self.straight_move(start_square, max_one, can_jump));
        gen_moveset
    }

    fn gen_moveset_king(&self, start_square: Square) -> Vec<Action> {
        let max_one = true;
        let can_jump = false;
        let mut gen_moveset = self.diagonal_move(start_square, max_one, can_jump);
        gen_moveset.append(&mut self.straight_move(start_square, max_one, can_jump));
        gen_moveset.append(&mut self.castling(start_square));
        gen_moveset
    }

    fn gen_moveset_knight(&self, start_square: Square) -> Vec<Action> {
        let max_one = true;
        let can_jump = true;
        let offsets = vec![(-2, 1), (-1, 2), (1, 2), (2, 1), (2, -1), (1, -2), (-1, -2)];
        let gen_moveset = self.gen_generic_moveset(start_square, offsets, max_one, can_jump);
        gen_moveset
    }

    fn diagonal_move(&self, start_square: Square, max_one: bool, can_jump: bool) -> Vec<Action> {
        let offsets = vec![(-1, 1), (-1, -1), (1, 1), (1, -1)];
        let gen_moveset: Vec<Action> =
            self.gen_generic_moveset(start_square, offsets, max_one, can_jump);
        gen_moveset
    }

    fn straight_move(&self, start_square: Square, max_one: bool, can_jump: bool) -> Vec<Action> {
        let offsets = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        let gen_moveset: Vec<Action> =
            self.gen_generic_moveset(start_square, offsets, max_one, can_jump);
        gen_moveset
    }

    fn gen_pawn_attack_moveset(&self,from_square: Square) -> Vec<Action>{
        let (x,y)=(from_square.coordinate.0,from_square.coordinate.1);
        let (team_offset_y,promotion_row) = match self.player {
            Team::White => (1,7),
            Team::Black => (-1,0)
        };
        let mut gen_moveset:Vec<Action>=vec![];

        let offsets=vec![(1,team_offset_y),(-1,team_offset_y)];
        for offset in offsets{
            if not_out_of_bounds(x+offset.0,y+offset.1){
                let to_square=self.matrix[(x+offset.0)as usize][(y+offset.1) as usize];
                if not_same_team(self.player, to_square){
                    let mut action_type=ActionType::Regular;
                    if y+offset.1==promotion_row{
                        action_type=ActionType::Promotion;
                    }
                    let action= Action{
                        from:from_square,
                        to:to_square,
                        action_type
                    };
                    gen_moveset.push(action);

                }
            }
        }
        gen_moveset
    }

    fn castling(&self,start_square: Square) ->Vec<Action>{
        let (x,y)=(start_square.coordinate.0,start_square.coordinate.1);
        let mut gen_moveset:Vec<Action>=vec![];
        let mut squares_is_safe:bool=false;
        let mut can_castle:bool=false;
         //kingside castling
        if self.unmoved(start_square){
            if self.matrix[7][y as usize].piece.is_some(){
                if self.unmoved(self.matrix[7][y as usize]){
                    for dx in 1..3{
                        if self.matrix[(x+dx)as usize][y as usize].piece.is_none(){
                            can_castle=true;
                        }
                        else{
                            can_castle=false;
                        }
                    }
                    if can_castle{
                        for dx in 1..3{
                            if !self.check_square_attacked(self.matrix[(x+dx)as usize][y as usize]){
                                squares_is_safe=true;
                            }
                            else{
                                squares_is_safe=false;
                            }
                        }
                    }
                }          
            }
            
        }
        if squares_is_safe && can_castle{
            let action=Action{
                from:start_square,
                to:self.matrix[(x+2) as usize][y as usize],
                action_type: ActionType::Castling,
            };
            gen_moveset.push(action);
        }
        squares_is_safe=false;
        can_castle=false;

        if self.unmoved(start_square){
            if self.matrix[0][y as usize].piece.is_some(){
                if self.unmoved(self.matrix[0][y as usize]){
                    for dx in 1..3{
                        if self.matrix[(x-dx)as usize][y as usize].piece.is_none(){
                            can_castle=true;
                        }
                        else{
                            can_castle=false;
                            
                        }
                    }
                    if can_castle{
                        println!("can castle");
                        for dx in 1..3{
                            if !self.check_square_attacked(self.matrix[(x-dx)as usize][y as usize]){
                                squares_is_safe=true;
                            }
                            else{
                                squares_is_safe=false;
                            }
                        }
                    }
                }  
            }
                    
        }
        if squares_is_safe && can_castle{
            let action=Action{
                from:start_square,
                to:self.matrix[(x-2)as usize][y as usize],
                action_type: ActionType::Castling,
            };
            gen_moveset.push(action);
        }

        gen_moveset
    }

    fn check_square_attacked(&self,square: Square) -> bool{
        let mut is_attacked:bool=false;

        for a in self.gen_moveset_bishop(square).iter(){
            if a.to.piece.is_some(){
                if a.to.piece.unwrap().rank==Rank::Bishop{
                    is_attacked=true;
                }
            }
        }
        for a in self.gen_moveset_rook(square).iter(){
            if a.to.piece.is_some(){
                if a.to.piece.unwrap().rank==Rank::Rook{
                    is_attacked=true;
                }
            }
        }
        for a in self.gen_moveset_queen(square).iter(){
            if a.to.piece.is_some(){
                if a.to.piece.unwrap().rank==Rank::Queen{
                    is_attacked=true;
                }
            }
        }
        for a in self.gen_moveset_knight(square).iter(){
            if a.to.piece.is_some(){
                if a.to.piece.unwrap().rank==Rank::Knight{
                    is_attacked=true;
                }
            }
        }
        for a in self.gen_pawn_attack_moveset(square).iter(){
            if a.to.piece.is_some(){
                if a.to.piece.unwrap().rank==Rank::Pawn{
                    is_attacked=true;
                }
            }
        }
        is_attacked
    }

    fn gen_generic_moveset(
        &self,
        start_square: Square,
        offsets: Vec<(isize, isize)>,
        max_one: bool,
        can_jump: bool,
    ) -> Vec<Action> {
        let start_coordinate = start_square.coordinate;
        let start_x = start_coordinate.0;
        let start_y = start_coordinate.1;
        let mut gen_moveset: Vec<Action> = vec![];
        let offsets = offsets;
        for offset in offsets.iter() {
            let mut step = 1;
            loop {
                let new_x = start_x + offset.0 * step;
                let new_y = start_y + offset.1 * step;

                let mut psuedo_valid_moveset = self.psuedo_vaild_move(start_square, new_x, new_y);

                gen_moveset.append(&mut psuedo_valid_moveset.0);

                if psuedo_valid_moveset.1 == true && !can_jump {
                    break;
                }
                step += 1;
                if max_one {
                    break;
                }
            }
        }

        gen_moveset
    }

    fn psuedo_vaild_move(&self, square: Square, x: isize, y: isize) -> (Vec<Action>, bool) {
        let mut gen_moveset: Vec<Action> = vec![];
        let mut collided: bool = false;
        if not_out_of_bounds(x, y) {
            let to_square = self.matrix[x as usize][y as usize];
            if to_square.piece.is_none() {
                let action = Action {
                    from: square,
                    to: to_square,
                    action_type: ActionType::Regular,
                };
                gen_moveset.push(action);
            } else if not_same_team(self.player, to_square) {
                collided = true;
                let action = Action {
                    from: square,
                    to: to_square,
                    action_type: ActionType::Regular,
                };
                gen_moveset.push(action)
            } else {
                collided = true;
            }
        } else {
            collided = true;
        }
        (gen_moveset, collided)
    }

    pub fn square_from_string(&self, letter_coordinate: &str) -> Result<Square, String> {
        let coordinate = match Game::coordinate_from_string(letter_coordinate) {
            Err(e) => return Err(e),
            Ok(c) => c,
        };
        let square = Square {
            coordinate: coordinate,
            piece: self.matrix[coordinate.0 as usize][coordinate.1 as usize].piece,
        };
        Ok(square)
    }

    pub fn square_to_string(&self, square: Square) -> String {
        let coordinate = square.coordinate;
        Game::coordinate_to_string(coordinate)
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

        let string = String::from(column_letter) + &row_letter;
        string
    }
    pub fn coordinate_from_string(letter_coordinate: &str) -> Result<(isize, isize), String> {
        if letter_coordinate.len() != 2 {
            return Result::Err(String::from("Coordinate wasnt in correct format"));
        }
        let column_letter = letter_coordinate
            .chars()
            .nth(0)
            .unwrap()
            .to_ascii_lowercase();
        let row = letter_coordinate
            .chars()
            .nth(1)
            .unwrap()
            .to_digit(10)
            .unwrap() as isize;

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
    pub fn unmoved(&self,from_square: Square) -> bool {
        match from_square.piece.unwrap().rank {
            Rank::Pawn => match from_square.piece.unwrap().team {
                Team::White => {
                    if from_square.coordinate.1 == 1 {
                        return true;
                    } else {
                        return false;
                    }
                }
                Team::Black => {
                    if from_square.coordinate.1 == 6 {
                        return true;
                    } else {
                        return false;
                    }
                }
            },
            Rank::Rook => {
                for action in self.history.iter(){
                    if action.from.coordinate== from_square.coordinate{
                        return false;
                    }
                }
                return true;
            }
            _ => {
                for action in self.history.iter(){
                    if action.from.coordinate== from_square.coordinate{
                        return false;
                    }
                }
                return true;
            }
        }
    }
    
}

fn not_same_team(team: Team, square: Square) -> bool {
    if square.piece.is_some() {
        if square.piece.unwrap().team != team {
            return true;
        }
    }
    return false;
}

pub fn not_out_of_bounds(x: isize, y: isize) -> bool {
    if x < 0 || x > 7 || y < 0 || y > 7 {
        return false;
    } else {
        return true;
    }
}
fn next_player(team: Team) -> Team {
    if team == Team::White {
        return Team::Black;
    } else {
        return Team::White;
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string_to_print = String::new();
        string_to_print.push_str("A B C D E F G H \n");
        for row in (0..8).rev() {
            for column in 0..8 {
                let square = self.matrix[column][row];
                string_to_print.push_str(&String::from(format!("{} ", square)));
            }
            string_to_print.push_str(&String::from(format!(" {} \n", row + 1)));
        }
        write!(f, "{}", string_to_print)
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Action {
    from: Square,
    to: Square,
    action_type: ActionType,
}

#[derive(Debug, Copy, Clone)]
pub enum ActionType {
    Regular,
    Enpassant,
    Promotion,
    Castling,
}

#[derive(Debug, Copy, Clone)]
pub struct Square {
    piece: Option<Piece>,
    coordinate: (isize, isize),
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
enum Team {
    White,
    Black,
}
#[derive(Debug, Copy, Clone, PartialEq)]
enum Rank {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}
impl fmt::Display for Rank{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f,"{}",self)
    }
}

#[derive(Debug, Copy, Clone)]
struct Piece {
    pub team: Team,
    rank: Rank,
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

#[cfg(test)]
mod tests {
    use super::not_out_of_bounds;
    use super::Game;
    use super::Piece;
    use super::Rank;
    use super::Team;
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
        assert!(not_out_of_bounds(-1, 1) == false);
        assert!(not_out_of_bounds(0, 8) == false);
        assert!(not_out_of_bounds(-1, -1) == false);
        assert!(not_out_of_bounds(8, 0) == false);
        assert!(not_out_of_bounds(1, 1) == true);
        assert!(not_out_of_bounds(9, 0) == false);
    }

    #[test]
    fn test_coordinate_from_string() {
        println!("{:?}", Game::coordinate_from_string("a1").unwrap());
        assert!(Game::coordinate_from_string("a1").unwrap() == (0, 0));
        assert_eq!(
            Game::coordinate_from_string("z1"),
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
        let coordinate = Game::coordinate_from_string("e4").unwrap();
        game.white_king_square=game.matrix[coordinate.0 as usize][coordinate.1 as usize];
        game.matrix[coordinate.0 as usize][coordinate.1 as usize].piece = Some(Piece {
            rank: Rank::King,
            team: Team::White,
        });
        assert_eq!(8, game.move_from_string("e4").unwrap().len());
    }

    #[test]
    fn test_castling(){
        let mut game = Game::new();
        game.matrix[5][0].piece=None;
        game.matrix[6][0].piece=None;
        let moveset=game.castling(game.matrix[4][0]);
        println!("{:?}",moveset);
        assert_eq!(moveset.len(),1);
        game.matrix[1][0].piece=None;
        game.matrix[2][0].piece=None;
        game.matrix[3][0].piece=None;
        println!("");
        println!("{:?}",moveset);
        let moveset=game.castling(game.matrix[4][0]);
        assert_eq!(moveset.len(),2);
        
    }
    #[test]
    fn test_castling_check(){
        let mut game = Game::new();
        game.matrix[5][0].piece=None;
        game.matrix[6][0].piece=None;
        game.matrix[6][1].piece=None;
        game.matrix[6][4].piece=Some(Piece{
            rank:Rank::Rook,
            team:Team::Black,
        });
        let moveset=game.castling(game.matrix[4][0]);
        println!("{:?}",moveset);
        assert_eq!(0, moveset.len());
    }

    #[test]
    fn test_stalemate(){
        let mut game= Game::new_empty();
        game.black_king_square=game.matrix[0][7];
        game.matrix[4][7].piece=None;
        game.matrix[0][7].piece=Some(Piece{rank:Rank::King,team:Team::Black});
        game.player=Team::Black;
        game.matrix[2][6].piece=Some(Piece{rank:Rank::Queen,team:Team::White});
        println!("{}",game);
       // assert_eq!(game.is_more_moves(),false);

        
    }
}