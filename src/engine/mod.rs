// Modules
use std::f64;

// Types
type BoardIndex = u8;

// Constants
const BOARD_INDEX_MAX:u32 = 31;
const MAX_PAWNS:u32 = 4;

// PlayerType
pub enum PlayerType {
    P1,
    P2
}

// PieceType
pub enum PieceType {
    KING,
    PAWN
}

// Position
pub struct Position {
    pub row: u32,
    pub col: u32
}

impl Position {
    pub fn new(&self, row: u32, col: u32) -> Position {
        Position {
            row,
            col
        }
    }
}

// Board
pub struct Board {
    /*
    (0-7) | (8-15) | (16-23) | (24-31)

                 P2
Col     0    1    2    3    4
Row  +----+----+----+----+----+
 0   |  0 |  1 |  2 |  3 |  4 |
     +----+----+----+----+----+
 1   |  5 |  6 |  7 |  8 |  9 |
     +----+----+----+----+----+
 2   | 10 | 11 | 12 | 13 | 14 |
     +----+----+----+----+----+
 3   | 15 | 16 | 17 | 18 | 19 |
     +----+----+----+----+----+
 4   | 20 | 21 | 22 | 23 | 24 |
     +----+----+----+----+----+
                 P1
    */
    // 00 | 00 | 02 | 00
    p1_king: u32,

    // 00 | 00 | 00 | 80
    p1_pawns: u32,

    // 20 | 00 | 00 | 00
    p2_king: u32,

    // D8 | 00 | 00 | 00
    p2_pawns: u32
}

impl Board {
    pub fn new() -> Board {
        Board {
            p1_king: 0x00000200,
            p1_pawns: 0x00000080,
            p2_king: 0x20000000,
            p2_pawns: 0xD8000000
        }
    }

    fn get_king_position(&self, player_type: PlayerType) -> u32 {
        let king_pos:u32;
        if let PlayerType::P1 = player_type {
            king_pos = self.p1_king
        } else {
            king_pos = self.p2_king
        };

        // find right most bit set, N & ~(N-1)
        let pos = (king_pos & !(king_pos - 1)) as f64;
        return BOARD_INDEX_MAX - pos.log2() as u32;
    }

    fn get_pawn_positions(&self, player_type: PlayerType) -> Vec<u32> {
        let mut pawn_pos:u32;
        if let PlayerType::P1 = player_type {
            pawn_pos = self.p1_pawns
        } else {
            pawn_pos = self.p2_pawns
        };

        let mut pawn_positions:Vec<u32> = Vec::new();
        let mut found = 0;
        let mut index = 0;
        while pawn_pos > 0 {
            let is_bit_set = pawn_pos & index;
            if is_bit_set == 1 {
                pawn_positions.push(index);
                found += 1;
            }
            if found == MAX_PAWNS {
                break;
            }

            pawn_pos >>= 1;
            index += 1;
        }

        return pawn_positions;
    }

    /*
    fn get_board_positions(&self, player_type: PlayerType, piece_type: PieceType) -> Vec<Position> {
        let row = pos / 5;
        let col = pos % 5;
        return (row, col);
    }
    */

    pub fn print(&self) {
        println!("P1King = {}, 1D-Pos = {}",
                 self.p1_king,
                 self.get_king_position(PlayerType::P1));

        let pawn_positions = self.get_pawn_positions(PlayerType::P1);
        println!("{:?}", pawn_positions);
    }
}
