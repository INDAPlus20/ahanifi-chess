#[allow(dead_code)]
use crate::game;
use crate::moves;
use std::fs;
pub fn read_pgn(filepath: &str) ->(Vec<moves::Action>,Vec<game::GameState>) {
    let half_turns = file_to_half_turn(filepath);
    let mut actions: Vec<moves::Action> = vec![];
    let mut gamestates: Vec<game::GameState> = vec![];
    let mut game = game::Game::new();
    let max_move = 10000;

    for (i, half_turn) in half_turns.iter().enumerate() {
        println!("{}", game);
        if max_move == ((i + 2) / 2) {
            break;
        }
        println!("Move number {}", (i + 2) / 2);
        let index = half_turn.rfind(|c: char| c.is_digit(10));
        if index.is_none() {
            match half_turn.len() {
                5 => {
                    println!("Queenside Castling");
                    for action in game.all_moves() {
                        if action.action_type == moves::ActionType::Castling {
                            if action.to.coordinate.0 == 2 {
                                println!("{:?}\n", action);
                                actions.push(action);
                                game.perform_action(action);
                                break;
                            }
                        }
                    }
                }
                3 => {
                    println!("Kingside Castling");
                    for action in game.all_moves() {
                        if action.action_type == moves::ActionType::Castling {
                            if action.to.coordinate.0 == 6 {
                                println!("{:?}\n", action);
                                actions.push(action);
                                game.perform_action(action);
                                break;
                            }
                        }
                    }
                }
                _ => panic!("invalid input"),
            }
        } else {
            println!("{}", half_turn);
            let index = index.unwrap();
            let letter_coordinate = &half_turn[index - 1..index + 1];
            let coordinate = game::coordinate_from_string(letter_coordinate).unwrap();

            let rank = match half_turn.find(|c: char| c.is_uppercase()) {
                Some(i) => match half_turn.chars().nth(i).unwrap() {
                    'P' => game::Rank::Pawn,
                    'R' => game::Rank::Rook,
                    'B' => game::Rank::Bishop,
                    'Q' => game::Rank::Queen,
                    'N' => game::Rank::Knight,
                    'K' => game::Rank::King,
                    _ => panic!("Piece letter not valid"),
                },
                None => game::Rank::Pawn,
            };

            let mut possible_actions: Vec<moves::Action> = vec![];
            let all_moves = game.all_moves();

            for action in all_moves.iter() {
                if action.to.coordinate == coordinate {
                    if action.from.piece.unwrap().rank == rank {
                        possible_actions.push(*action);
                    }
                }
            }

            if possible_actions.len() == 1 {
                let this_action = possible_actions[0];
                println!("{:?}\n", this_action);
                actions.push(this_action);
                game.perform_action(this_action);
            } else {
                let column: isize;
                let row: isize;
                let mut offset: usize = 0;
                if half_turn.chars().nth(0).unwrap().is_uppercase() {
                    offset = 1;
                }
                if half_turn.chars().nth(0 + offset).unwrap().is_numeric() {
                    row = half_turn
                        .chars()
                        .nth(0 + offset)
                        .unwrap()
                        .to_digit(10)
                        .unwrap() as isize;
                    for action in possible_actions {
                        if action.from.coordinate.1 == row {
                            actions.push(action);
                            game.perform_action(action);
                            break;
                        }
                    }
                } else {
                    println!("{:?}", possible_actions);
                    column = char_to_column(half_turn.chars().nth(0 + offset).unwrap());
                    if half_turn.chars().nth(2).unwrap().is_numeric() {
                        row = half_turn
                            .chars()
                            .nth(1 + offset)
                            .unwrap()
                            .to_digit(10)
                            .unwrap() as isize;
                        for action in possible_actions {
                            if action.from.coordinate.1 == row && action.from.coordinate.0 == column
                            {
                                actions.push(action);
                                game.perform_action(action);
                                break;
                            }
                        }
                    } else {
                        for action in possible_actions {
                            if action.from.coordinate.0 == column {
                                actions.push(action);
                                game.perform_action(action);
                                break;
                            }
                        }
                    }
                }
            }
        }
        if half_turn.chars().last().unwrap() == '+' {
            gamestates.push(game::GameState::Check);
        } else if half_turn.chars().last().unwrap() == '#' {
            gamestates.push(game::GameState::Checkmate);
        } else {
            gamestates.push(game::GameState::Active);
        }
    }

    println!("{}", game);
    println!("{:?}", game.get_game_state());
    (actions,gamestates)
}

fn char_to_column(c: char) -> isize {
    match c {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => panic!("wrong char"),
    }
}

fn file_to_half_turn(filepath: &str) -> Vec<String> {
    let pgn_file = fs::read_to_string(filepath).unwrap();
    let mut content = String::new();
    let mut comment_flag: bool = false;
    for c in pgn_file.chars() {
        if c == '{' {
            comment_flag = true;
        }
        if comment_flag {
            if c == '}' {
                comment_flag = false;
            }
            continue;
        }
        content.push(c);
    }

    let mut full_turns: Vec<String> = content
        .split(|c| c == '.')
        .map(|s| {
            s.trim()
                .split_whitespace()
                .enumerate()
                .filter(|&(i, _)| i < 2)
                .map(|(_, e)| e)
                .collect::<Vec<&str>>()
                .join(" ")
        })
        .collect();

    full_turns.remove(0);

    let mut half_turns: Vec<String> = full_turns
        .iter()
        .flat_map(|s| s.split_whitespace())
        .map(|s| s.to_string())
        .collect();

    if half_turns.last().unwrap().contains("-") {
        half_turns.remove(half_turns.len() - 1);
    }
    half_turns
}
