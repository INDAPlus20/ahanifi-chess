use chess::board;

fn main() {
    let path = "board_config.txt";
    let my_board = board::new(path);
    println!("{}", my_board)
}
