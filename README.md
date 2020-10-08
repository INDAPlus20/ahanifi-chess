# ahanifi-chess: Rust-based chess engine with pgn replay functionality

## Documentation (WIP)
### Enumerables
| **Enumerable** | **Values** | **Description** |
|----------------|------------|-----------------|
| `GameState`    | `Active`, `Check`, `Checkmate`,`Stalemate`,`FiftyRule`| Represents the state that a game can have. |
| `Team`       | `White`, `Black` | Represents the colour of a chess piece. |
| `Rank`    | `King`, `Queen`, `Bishop`, `Knight`, `Rook`, `Pawn` | Represents the type of a chess piece. |

### Structure `Game`

| **Function** | **Description** |
|--------------|-----------------|
| `pub fn new() -> Game` | Initialises a new board with default configuration. |
| `pub fn perform_action(&mut self, action:Action)`| Performs a move. |
| `pub fn set_promotion(&mut self, _piece: String) ` | Set the piece type that a peasant becames following a promotion. |
| `pub fn get_game_state(&self) -> GameState` | Get the current game state. |
| `pub fn moves_from_string(&mut self, letter_coordinate: &str) -> Result<Vec<Action>, String>`| The Ok() value returns all legal moves for a given square. The Err() value returns a string describing the error. |
|` pub fn game_from_blockstate(blocks: &str) -> Game`| Initialises a board with the given blockstate configuration.|
Positions are given as strings with the format `"<file><rank>"`.

### Default board as blockstates
```
RB NB BB QB KB BB NB RB
PB PB PB PB PB PB PB PB
XX XX XX XX XX XX XX XX
XX XX XX XX XX XX XX XX
XX XX XX XX XX XX XX XX
XX XX XX XX XX XX XX XX
PW PW PW PW PW PW PW PW
RW NW BW QW KW BW NW RW
```

### Structure `PGN`
| **Function** | **Description** |
|--------------|-----------------|
| `pub fn read_pgn(filepath: &str) ->(Vec<moves::Action>,Vec<game::GameState>) ` | Returns a vector of Action and GameState tuples. Each tuple represents a half turn.|
