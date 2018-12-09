// Modules
mod engine;

use engine::Board;

fn main() {
    let board = Board::new();
    board.print();
}