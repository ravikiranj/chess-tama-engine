// Modules
mod engine;

// Bindings
use engine::Board;

fn main() {
    let board = Board::new("Chess-tama");
    println!("Board Name = {}", board.get_name());
}
