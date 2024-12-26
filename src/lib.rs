mod board;
mod piece;
mod shape;

use crate::board::Board;
use crate::piece::Direction::{Down, Left, Right};
use crate::piece::Piece;
use rand::seq::SliceRandom;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, KeyboardEvent, Window};

#[wasm_bindgen]
pub struct Tetris {
  board: Board,
  ctx: CanvasRenderingContext2d,
  current_piece: Piece,
  game_over: bool,
  score: u32,
}

#[wasm_bindgen]
impl Tetris {
  #[wasm_bindgen(constructor)]
  pub fn new(canvas_id: &str) -> Result<Tetris, JsValue> {
    // Access the HTML canvas
    let window: Window = web_sys::window().unwrap();
    let document: Document = window.document().unwrap();
    let canvas = document
      .get_element_by_id(canvas_id)
      .ok_or("Canvas not found")?
      .dyn_into::<HtmlCanvasElement>()?;
    let ctx = canvas
      .get_context("2d")?
      .ok_or("Failed to get canvas context")?
      .dyn_into::<CanvasRenderingContext2d>()?;

    let board = Board::new(10, 20);

    let current_shape = shape::get_random_shape();
    let current_piece = Piece {
      x: board.width / 2 - current_shape.width / 2,
      y: 0,
      shape: current_shape,
    };

    // Create an empty board
    Ok(Tetris {
      board,
      ctx,
      current_piece,
      game_over: false,
      score: 0,
    })
  }

  pub fn start_game(self) {
    let game_ref = Rc::new(RefCell::new(self));

    let tick_game_clone = Rc::clone(&game_ref);
    let tick_closure = Closure::wrap(Box::new(move || {
      tick_game_clone.borrow_mut().tick_and_redraw();
    }) as Box<dyn FnMut()>);

    // Use `setInterval` to call the Rust function periodically
    let window = web_sys::window().unwrap();
    window
      .set_interval_with_callback_and_timeout_and_arguments_0(
        tick_closure.as_ref().unchecked_ref(),
        500, // Interval in milliseconds (blocks fall every 500ms)
      )
      .expect("Failed to set interval");

    // Prevent the closure from being dropped
    tick_closure.forget();

    Self::attach_input_listeners(game_ref);
  }

  fn attach_input_listeners(game_ref: Rc<RefCell<Tetris>>) {
    let game_clone = Rc::clone(&game_ref);
    let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
      let mut game = game_clone.borrow_mut();
      match event.key().as_str() {
        "ArrowLeft" => game.move_left(),
        "ArrowRight" => game.move_right(),
        "ArrowUp" => game.rotate_piece(),
        "ArrowDown" => game.move_down(),
        " " => game.hard_drop(),
        _ => {}
      }
    }) as Box<dyn FnMut(_)>);

    web_sys::window()
      .unwrap()
      .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
      .expect("Failed to add keydown listener");

    closure.forget();
  }

  // Game tick: Advance the block down one cell and redraw
  fn tick_and_redraw(&mut self) {
    if self.game_over {
      self.display_game_over();
      return;
    }

    if self.current_piece.can_move(Down, &self.board) {
      self.move_down();
    } else {
      self.merge_shape_into_board();
      self.spawn_new_block();
    }

    self.draw();
  }

  #[allow(deprecated)]
  fn display_game_over(&self) {
    self.ctx.set_fill_style(&JsValue::from_str("red"));
    self.ctx.set_font("30px Arial");
    self
      .ctx
      .fill_text(
        "Game Over",
        (self.ctx.canvas().unwrap().width() as f64 / 2.0) - 50.0,
        (self.ctx.canvas().unwrap().height() as f64 / 2.0),
      )
      .unwrap();
  }

  fn merge_shape_into_board(&mut self) {
    for y in 0..self.current_piece.shape.height {
      for x in 0..self.current_piece.shape.width {
        if self.current_piece.shape.cells[y as usize][x as usize] == 1 {
          let board_y = (self.current_piece.y + y) as usize;
          let board_x = (self.current_piece.x + x) as usize;

          self.board.grid[board_y][board_x] = self.current_piece.shape.color;
        }
      }
    }

    // Clear full lines and update the score
    let lines_cleared = self.board.clear_full_lines();

    // Score points based on the number of cleared lines
    self.score += match lines_cleared {
      1 => 100, // 1 line cleared: 100 points
      2 => 300, // 2 lines cleared: 300 points
      3 => 500, // 3 lines cleared: 500 points
      4 => 800, // 4 lines cleared (Tetris): 800 points
      _ => 0,   // No points for clearing 0 lines
    };
  }

  // Spawn a new block at the top of the board
  fn spawn_new_block(&mut self) {
    let new_shape = shape::get_random_shape();
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

  // Move the falling block left
  pub fn move_left(&mut self) {
    self.current_piece.move_piece(Left, &self.board);
    self.draw();
  }

  // Move the falling block right
  pub fn move_right(&mut self) {
    self.current_piece.move_piece(Right, &self.board);
    self.draw();
  }

  fn move_down(&mut self) {
    self.current_piece.move_piece(Down, &self.board);
    self.draw();
  }

  fn rotate_piece(&mut self) {
    self.current_piece.rotate_piece(&self.board);
    self.draw();
  }

  pub fn hard_drop(&mut self) {
    while self.current_piece.can_move(Down, &self.board) {
      self.current_piece.move_piece(Down, &self.board);
    }

    self.merge_shape_into_board();
    self.spawn_new_block();

    self.draw();
  }

  // Draw the entire board, including the falling block
  #[allow(deprecated)]
  fn draw(&self) {
    let canvas = &self.ctx;
    self.board.draw(canvas);
    self.current_piece.draw(canvas, &self.board);

    let cell_width = self.ctx.canvas().unwrap().width() as f64 / self.board.width as f64;
    let cell_height = self.ctx.canvas().unwrap().height() as f64 / self.board.height as f64;

    for y in 0..self.board.height {
      for x in 0..self.board.width {
        // Draw grid borders for clarity
        self.ctx.set_stroke_style(&JsValue::from_str("black"));
        self.ctx.stroke_rect(
          x as f64 * cell_width,
          y as f64 * cell_height,
          cell_width,
          cell_height,
        );
      }
    }

    // Display the current score
    self.ctx.set_fill_style(&JsValue::from_str("black"));
    self.ctx.set_font("20px Arial");
    self
      .ctx
      .fill_text(
        &format!("Score: {}", self.score),
        10.0,
        25.0, // Display score in the top-left corner
      )
      .unwrap();
  }
}
