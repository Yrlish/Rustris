use crate::board::Board;
use crate::piece::Direction::Down;
use crate::piece::Piece;
use crate::shape::get_random_shape;

pub struct GameState {
  pub board: Board,
  pub current_piece: Piece,
  pub game_over: bool,
  pub score: u32,
}

impl GameState {
  pub fn new(board_width: u8, board_height: u8) -> Self {
    let board = Board::new(board_width, board_height);
    let current_shape = get_random_shape();
    let current_piece = Piece {
      x: board_width / 2 - current_shape.width / 2,
      y: 0,
      shape: current_shape,
    };

    GameState {
      board,
      current_piece,
      game_over: false,
      score: 0,
    }
  }

  pub fn tick(&mut self) {
    if self.current_piece.can_move(Down, &self.board) {
      self.current_piece.move_piece(Down, &self.board);
    } else {
      self.merge_current_piece();
      self.spawn_new_piece();
    }
  }

  pub fn merge_current_piece(&mut self) {
    for y in self.current_piece.shape.iter_height() {
      for x in self.current_piece.shape.iter_width() {
        if self.current_piece.shape.cells[y][x] == 1 {
          let board_y = self.current_piece.y as usize + y;
          let board_x = self.current_piece.x as usize + x;

          self.board.grid[board_y][board_x] = self.current_piece.shape.color;
        }
      }
    }

    let lines_cleared = self.board.clear_full_lines();
    self.score += match lines_cleared {
      1 => 100,
      2 => 300,
      3 => 500,
      4 => 800,
      _ => 0,
    }
  }

  pub fn spawn_new_piece(&mut self) {
    let new_shape = get_random_shape();
    let new_piece = Piece {
      x: self.board.width / 2 - new_shape.width / 2,
      y: 0,
      shape: new_shape,
    };

    if !new_piece.can_stay(&self.board) {
      self.game_over = true;
    } else {
      self.current_piece = new_piece;
    }
  }
}
