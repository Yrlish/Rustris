use crate::board::Board;
use crate::shape::Shape;

pub struct Piece {
    pub shape: Shape,
    pub x: u8,
    pub y: u8,
}

impl Piece {
    fn can_move(&self, dx: u8, dy: u8, board: &Board) -> bool {
        let new_x = self.x + self.shape.width + dx;
        let new_y = self.y + self.shape.height + dy;

        // Check if the new position is out of horizontal bounds
        if new_x < 0 || new_x >= board.width {
            return false;
        }

        // Check if the new position is out of vertical bounds
        if new_y < 0 || new_y >= board.height {
            return false;
        }

        // Check if the space is already occupied
        if board.grid[new_y as usize][new_x as usize] != 0 {
            return false;
        }

        true
    }

    fn move_piece(&mut self, dx: u8, dy: u8, board: &Board) {
        if self.can_move(dx, dy, board) {
            self.x += dx;
            self.y += dy;
        }
    }
}
