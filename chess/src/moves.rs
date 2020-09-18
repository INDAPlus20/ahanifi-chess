use game::{Rank, Team};

use crate::game;
use crate::game::Square;

#[derive(Debug, Copy, Clone)]
pub struct Action {
    pub from: Square,
    pub to: Square,
    pub action_type: ActionType,
}

#[derive(Debug, Copy, Clone)]
pub enum ActionType {
    Regular,
    Enpassant,
    Promotion,
    Castling,
}

pub fn generate_moves(game: &mut game::Game, square: Square) -> Result<Vec<Action>, String> {
    let rank = match square.piece {
        Some(p) => p.rank,
        None => return Err(String::from("Tried to move empty square")),
    };
    let team = square.piece.unwrap().team;
    if team != game.player {
        return Err(String::from("Cant move enemy piece"));
    };

    let moveset: Vec<Action> = match rank {
        Rank::Pawn => gen_moveset_pawn(game, square),
        Rank::Rook => gen_moveset_rook(game, square),
        Rank::Knight => gen_moveset_knight(game, square),
        Rank::Bishop => gen_moveset_bishop(game, square),
        Rank::Queen => gen_moveset_queen(game, square),
        Rank::King => gen_moveset_king(game, square),
    };

    let mut legal_moveset: Vec<Action> = vec![];

    for action in &moveset {
        if !game.check(action) {
            legal_moveset.push(*action);
        }
    }
    Ok(legal_moveset)
}

fn gen_generic_moveset(
    game: &game::Game,
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

            let mut psuedo_valid_moveset = psuedo_vaild_move(game, start_square, new_x, new_y);

            gen_moveset.append(&mut psuedo_valid_moveset.0);

            if psuedo_valid_moveset.1 && !can_jump {
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

fn psuedo_vaild_move(game: &game::Game, square: Square, x: isize, y: isize) -> (Vec<Action>, bool) {
    let mut gen_moveset: Vec<Action> = vec![];
    let mut collided: bool = false;
    if game::not_out_of_bounds(x, y) {
        let to_square = game.matrix[x as usize][y as usize];
        if to_square.piece.is_none() {
            let action = Action {
                from: square,
                to: to_square,
                action_type: ActionType::Regular,
            };
            gen_moveset.push(action);
        } else if game::not_same_team(game.player, to_square) {
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

pub fn gen_moveset_pawn(game: &game::Game, start_square: Square) -> Vec<Action> {
    let mut available_moves = Vec::<Action>::new();
    //let coordinate:(usize,usize)=(square.coordinate.0.try_into().unwrap(),square.coordinate.1.try_into().unwrap());
    let (x, y) = (start_square.coordinate.0, start_square.coordinate.1);
    let (offset, promotion_row) = match game.player {
        Team::White => (1, 7),
        Team::Black => (-1, 0),
    };
    let new_coordinate_y = y + offset;
    let new_square = game.matrix[x as usize][new_coordinate_y as usize];
    if new_square.piece.is_none() {
        let mut action_type = ActionType::Regular;
        if new_coordinate_y == promotion_row {
            action_type = ActionType::Promotion;
        }
        let action = Action {
            from: start_square,
            to: new_square,
            action_type,
        };
        available_moves.push(action);
    }

    if game::unmoved(game, start_square) {
        let new_coordinate_y = y + 2 * offset;
        if game::not_out_of_bounds(x, new_coordinate_y) {
            let new_square = game.matrix[x as usize][new_coordinate_y as usize];
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

    available_moves.append(&mut gen_pawn_attack_moveset(game, start_square));

    let mut prev_action: &Action;

    //Enpassant
    for dx in (-1..2).step_by(2) {
        if game::not_out_of_bounds(x + dx, y) {
            let side_square = game.matrix[(x + dx) as usize][y as usize];
            if game::not_same_team(game.player, side_square)
                && side_square.piece.unwrap().rank == Rank::Pawn
                && !game.history.is_empty()
            {
                prev_action = game.history.last().unwrap();
                if prev_action.from.piece.unwrap().rank == Rank::Pawn {
                    let enemy_start_y = prev_action.from.coordinate.1;
                    let dy = (y - enemy_start_y).abs();
                    println!("{}", dy);
                    if dy == 2 {
                        let action = Action {
                            from: start_square,
                            to: game.matrix[(x + dx) as usize][(y + offset) as usize],
                            action_type: ActionType::Enpassant,
                        };
                        available_moves.push(action);
                    }
                }
            }
        }
    }

    available_moves
}

pub fn gen_moveset_rook(game: &game::Game, start_square: Square) -> Vec<Action> {
    let max_one = false;
    let can_jump = false;
    straight_move(game, start_square, max_one, can_jump)
}

pub fn gen_moveset_bishop(game: &game::Game, start_square: Square) -> Vec<Action> {
    let max_one = false;
    let can_jump = false;
    diagonal_move(game, start_square, max_one, can_jump)
}

pub fn gen_moveset_queen(game: &game::Game, start_square: Square) -> Vec<Action> {
    let max_one = false;
    let can_jump = false;
    let mut gen_moveset = diagonal_move(game, start_square, max_one, can_jump);
    gen_moveset.append(&mut straight_move(game, start_square, max_one, can_jump));
    gen_moveset
}

pub fn gen_moveset_king(game: &game::Game, start_square: Square) -> Vec<Action> {
    let max_one = true;
    let can_jump = false;
    let mut gen_moveset = diagonal_move(game, start_square, max_one, can_jump);
    gen_moveset.append(&mut straight_move(game, start_square, max_one, can_jump));
    gen_moveset.append(&mut castling(game, start_square));
    gen_moveset
}

pub fn gen_moveset_knight(game: &game::Game, start_square: Square) -> Vec<Action> {
    let max_one = true;
    let can_jump = true;
    let offsets = vec![(-2, 1), (-1, 2), (1, 2), (2, 1), (2, -1), (1, -2), (-1, -2)];
    gen_generic_moveset(game, start_square, offsets, max_one, can_jump)
}

fn diagonal_move(
    game: &game::Game,
    start_square: Square,
    max_one: bool,
    can_jump: bool,
) -> Vec<Action> {
    let offsets = vec![(-1, 1), (-1, -1), (1, 1), (1, -1)];
    gen_generic_moveset(game, start_square, offsets, max_one, can_jump)
}

fn straight_move(
    game: &game::Game,
    start_square: Square,
    max_one: bool,
    can_jump: bool,
) -> Vec<Action> {
    let offsets = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    gen_generic_moveset(game, start_square, offsets, max_one, can_jump)
}

pub fn gen_pawn_attack_moveset(game: &game::Game, from_square: Square) -> Vec<Action> {
    let (x, y) = (from_square.coordinate.0, from_square.coordinate.1);
    let (team_offset_y, promotion_row) = match game.player {
        Team::White => (1, 7),
        Team::Black => (-1, 0),
    };
    let mut gen_moveset: Vec<Action> = vec![];

    let offsets = vec![(1, team_offset_y), (-1, team_offset_y)];
    for offset in offsets {
        if game::not_out_of_bounds(x + offset.0, y + offset.1) {
            let to_square = game.matrix[(x + offset.0) as usize][(y + offset.1) as usize];
            if game::not_same_team(game.player, to_square) {
                let mut action_type = ActionType::Regular;
                if y + offset.1 == promotion_row {
                    action_type = ActionType::Promotion;
                }
                let action = Action {
                    from: from_square,
                    to: to_square,
                    action_type,
                };
                gen_moveset.push(action);
            }
        }
    }
    gen_moveset
}

pub fn castling(game: &game::Game, start_square: Square) -> Vec<Action> {
    let (x, y) = (start_square.coordinate.0, start_square.coordinate.1);
    let mut gen_moveset: Vec<Action> = vec![];
    let mut squares_is_safe: bool = false;
    let mut can_castle: bool = false;
    //kingside castling
    if game::unmoved(game, start_square)
        && game.matrix[7][y as usize].piece.is_some()
        && game::unmoved(game, game.matrix[7][y as usize])
    {
        for dx in 1..3 {
            if game.matrix[(x + dx) as usize][y as usize].piece.is_none() {
                can_castle = true;
            } else {
                can_castle = false;
            }
        }
        if can_castle {
            for dx in 1..3 {
                if !game.check_square_attacked(game.matrix[(x + dx) as usize][y as usize]) {
                    squares_is_safe = true;
                } else {
                    squares_is_safe = false;
                }
            }
        }
    }
    if squares_is_safe && can_castle {
        let action = Action {
            from: start_square,
            to: game.matrix[(x + 2) as usize][y as usize],
            action_type: ActionType::Castling,
        };
        gen_moveset.push(action);
    }
    squares_is_safe = false;
    can_castle = false;

    if game::unmoved(game, start_square)
        && game.matrix[0][y as usize].piece.is_some()
        && game::unmoved(game, start_square)
        && game.matrix[0][y as usize].piece.is_some()
        && game::unmoved(game, game.matrix[0][y as usize])
    {
        for dx in 1..3 {
            if game.matrix[(x - dx) as usize][y as usize].piece.is_none() {
                can_castle = true;
            } else {
                can_castle = false;
            }
        }
        if can_castle {
            println!("can castle");
            for dx in 1..3 {
                if !game.check_square_attacked(game.matrix[(x - dx) as usize][y as usize]) {
                    squares_is_safe = true;
                } else {
                    squares_is_safe = false;
                }
            }
        }
    }
    if squares_is_safe && can_castle {
        let action = Action {
            from: start_square,
            to: game.matrix[(x - 2) as usize][y as usize],
            action_type: ActionType::Castling,
        };
        gen_moveset.push(action);
    }

    gen_moveset
}
