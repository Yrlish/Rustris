use crate::board::Board;
use crate::shape::Shape;

pub struct Piece {
    pub shape: Shape,
    pub x: u8,
    pub y: u8,
}

impl Piece {
    pub fn can_move(&self, direction: Direction, board: &Board) -> bool {
        let (dx, dy) = match direction {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
        };

        for y in 0..self.shape.height {
            for x in 0..self.shape.width {
                // Only check occupied cells in the shape
                if self.shape.cells[y as usize][x as usize] == 1 {
                    let new_x = self.x as i8 + x as i8 + dx;
                    let new_y = self.y as i8 + y as i8 + dy;

                    // Check horizontal boundaries
                    if new_x < 0 || new_x >= board.width as i8 {
                        return false;
                    }

                    // Check vertical boundaries
                    if new_y < 0 || new_y >= board.height as i8 {
                        return false;
                    }

                    // Check if the space is occupied on the board
                    if board.grid[new_y as usize][new_x as usize] != 0 {
                        return false;
                    }
                }
            }
        }
        true
    }

    // Move the piece in the given direction
    pub fn move_piece(&mut self, direction: Direction, board: &Board) {
        if self.can_move(direction, board) {
            match direction {
                Direction::Left => self.x -= 1,
                Direction::Right => self.x += 1,
                Direction::Down => self.y += 1,
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Down,
}
