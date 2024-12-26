use crate::board::Board;
use crate::shape::{get_random_shape, Color, Shape};

#[derive(Clone)]
pub struct Piece {
  pub shape: Shape,
  pub x: u8,
  pub y: u8,
}

#[allow(deprecated)]
impl Piece {
  pub fn random_piece() -> Piece {
    let current_shape = get_random_shape();

    Piece {
      x: 5 - current_shape.width / 2,
      y: 0,
      shape: current_shape,
    }
  }

  pub fn can_move(&self, direction: Direction, board: &Board) -> bool {
    let (dx, dy) = match direction {
      Direction::Left => (-1, 0),
      Direction::Right => (1, 0),
      Direction::Down => (0, 1),
    };

    for y in self.shape.iter_height() {
      for x in self.shape.iter_width() {
        if self.shape.cells[y][x] == 1 {
          // Only check occupied cells in the shape
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
          if board.grid[new_y as usize][new_x as usize] != Color::None {
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

  pub fn rotate_piece(&mut self, board: &Board) {
    let original_shape = self.shape.clone();
    let rotated_shape = self.shape.rotate();
    self.shape = rotated_shape;

    // If out of bounds, shift the piece back onto the board
    if !self.is_within_bounds(board) {
      // Shift to the right if it's out of the left boundary
      while (self.x as i8) < 0 {
        self.x += 1;
      }

      // Shift to the left if it's out of the right boundary
      while self.x + self.shape.width > board.width {
        if self.x > 0 {
          self.x -= 1;
        } else {
          break;
        }
      }
    }

    // After shifting, check if the piece is valid
    if !self.can_stay(board) {
      // Revert if the rotation cannot fit
      self.shape = original_shape;
    }
  }

  // Check if the piece is within the board boundaries
  fn is_within_bounds(&self, board: &Board) -> bool {
    self.x as i8 >= 0 && self.x + self.shape.width <= board.width
  }

  // Check if the piece can stay in its current position
  pub fn can_stay(&self, board: &Board) -> bool {
    for y in self.shape.iter_height() {
      for x in self.shape.iter_width() {
        if self.shape.cells[y][x] == 1 {
          let board_x = self.x as i8 + x as i8;
          let board_y = self.y as i8 + y as i8;

          // Check boundaries
          if board_x < 0 || board_x >= board.width as i8 || board_y >= board.height as i8 {
            return false;
          }

          // Check overlaps
          if board_y >= 0 && board.grid[board_y as usize][board_x as usize] != Color::None {
            return false;
          }
        }
      }
    }
    true
  }
}

#[derive(Clone, Copy)]
pub enum Direction {
  Left,
  Right,
  Down,
}
