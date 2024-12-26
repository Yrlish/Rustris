mod board;
mod game_state;
mod piece;
mod shape;

use crate::game_state::GameState;
use crate::piece::Direction::{Down, Left, Right};
use crate::piece::Piece;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, KeyboardEvent, Window};

#[wasm_bindgen]
pub struct Tetris {
  ctx: CanvasRenderingContext2d,
  game_state: GameState,
}

#[wasm_bindgen]
impl Tetris {
  #[wasm_bindgen(constructor)]
  pub fn new(canvas_id: &str) -> Result<Tetris, JsValue> {
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

    let game_state = GameState::new(10, 20);

    canvas.set_width((game_state.board.width + 7) as u32 * game_state.board.cell_size as u32);
    canvas.set_height((game_state.board.height) as u32 * game_state.board.cell_size as u32);

    Ok(Tetris { ctx, game_state })
  }

  pub fn start_game(self) {
    let game_ref = Rc::new(RefCell::new(self));

    let tick_game_clone = Rc::clone(&game_ref);
    let tick_closure = Closure::wrap(Box::new(move || {
      tick_game_clone.borrow_mut().tick_and_redraw();
    }) as Box<dyn FnMut()>);

    let window = web_sys::window().unwrap();
    window
      .set_interval_with_callback_and_timeout_and_arguments_0(
        tick_closure.as_ref().unchecked_ref(),
        500,
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
        "Shift" => game.hold_piece(),
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
    if self.game_state.game_over {
      self.display_game_over();
      return;
    }

    self.game_state.tick();
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
        self.ctx.canvas().unwrap().height() as f64 / 2.0,
      )
      .unwrap();
  }

  // Move the falling block left
  pub fn move_left(&mut self) {
    self
      .game_state
      .current_piece
      .move_piece(Left, &self.game_state.board);
    self.draw();
  }

  // Move the falling block right
  pub fn move_right(&mut self) {
    self
      .game_state
      .current_piece
      .move_piece(Right, &self.game_state.board);
    self.draw();
  }

  fn move_down(&mut self) {
    self
      .game_state
      .current_piece
      .move_piece(Down, &self.game_state.board);
    self.draw();
  }

  fn hold_piece(&mut self) {
    self.game_state.hold_piece();
    self.draw();
  }

  fn rotate_piece(&mut self) {
    self
      .game_state
      .current_piece
      .rotate_piece(&self.game_state.board);
    self.draw();
  }

  pub fn hard_drop(&mut self) {
    while self
      .game_state
      .current_piece
      .can_move(Down, &self.game_state.board)
    {
      self
        .game_state
        .current_piece
        .move_piece(Down, &self.game_state.board);
    }

    self.game_state.merge_current_piece();
    self.game_state.spawn_new_piece();

    self.draw();
  }

  // Draw the entire board, including the falling block
  #[allow(deprecated)]
  fn draw(&self) {
    let canvas = &self.ctx;
    let board = &self.game_state.board;
    let cell_size = board.cell_size as f64;

    // Clear entire canvas at the start of each frame
    canvas.clear_rect(
      0.0,
      0.0,
      canvas.canvas().unwrap().width() as f64,
      canvas.canvas().unwrap().height() as f64,
    );

    // --- Draw Background ---
    // Main play area background (dark gray)
    canvas.set_fill_style(&JsValue::from_str("#333333")); // Darker for less strain
    canvas.fill_rect(
      0.0,
      0.0,
      board.width as f64 * cell_size,
      canvas.canvas().unwrap().height() as f64,
    );

    // Extra area (Score & Hold) background (slightly lighter gray for distinction)
    canvas.set_fill_style(&JsValue::from_str("#444444"));
    canvas.fill_rect(
      board.width as f64 * cell_size,
      0.0,
      canvas.canvas().unwrap().width() as f64,
      canvas.canvas().unwrap().height() as f64,
    );

    // --- Draw the Tetromino Pieces on the Board ---
    board.draw_pieces(canvas); // Draw fixed pieces
    self.game_state.current_piece.draw_ghost(canvas, board); // Ghost piece
    self.game_state.current_piece.draw(canvas, board); // Active piece

    // --- Draw Gridlines ---
    canvas.set_stroke_style(&JsValue::from_str("#555555")); // Subtle gridlines
    for y in 0..board.height {
      let y_pos = y as f64 * cell_size;
      for x in 0..board.width {
        let x_pos = x as f64 * cell_size;
        canvas.stroke_rect(x_pos, y_pos, cell_size, cell_size);
      }
    }

    // --- Display Score ---
    canvas.set_fill_style(&JsValue::from_str("white")); // Bright text for visibility
    canvas.set_font("20px 'Courier New', monospace"); // Monospace for clarity
    canvas
      .fill_text(
        &format!("Score: {}", self.game_state.score),
        (board.width + 1) as f64 * cell_size,
        cell_size * 1.25,
      )
      .unwrap();

    // --- Display Held Piece ---
    self.draw_hold_piece();
  }

  fn draw_hold_box(&self, canvas: &CanvasRenderingContext2d, cell_size: f64, board_width: u8) {
    let hold_x_offset = (board_width + 1) as f64 * cell_size;
    let hold_y_offset = cell_size * 4.0;
    let box_width = cell_size * 5.0;
    let box_height = cell_size * 5.0;

    // Draw "Hold" background box
    canvas.set_fill_style_str("#222222");
    canvas.fill_rect(hold_x_offset, hold_y_offset, box_width, box_height);

    // Draw "Hold" label
    canvas.set_fill_style_str("white");
    canvas.set_font("20px 'Courier New', monospace");
    canvas
      .fill_text(
        "Hold",
        hold_x_offset + cell_size * 1.7,
        hold_y_offset - 10.0,
      )
      .unwrap();
  }

  fn draw_piece_centered(
    &self,
    canvas: &CanvasRenderingContext2d,
    piece: &Piece,
    hold_x_offset: f64,
    hold_y_offset: f64,
    box_width: f64,
    box_height: f64,
    cell_size: f64,
  ) {
    let piece_width = piece.shape.width as f64 * cell_size;
    let piece_height = piece.shape.height as f64 * cell_size;
    let offset_x = hold_x_offset + (box_width - piece_width) / 2.0;
    let offset_y = hold_y_offset + (box_height - piece_height) / 2.0;

    let color = piece.shape.color.to_rgba(1.0);

    for y in piece.shape.iter_height() {
      for x in piece.shape.iter_width() {
        if piece.shape.cells[y][x] == 1 {
          // Shape's color
          canvas.set_fill_style_str(&color);
          canvas.fill_rect(
            offset_x + x as f64 * cell_size,
            offset_y + y as f64 * cell_size,
            cell_size,
            cell_size,
          );

          // Gridlines
          canvas.set_stroke_style_str("#555555"); // Subtle gridlines
          canvas.stroke_rect(
            offset_x + x as f64 * cell_size,
            offset_y + y as f64 * cell_size,
            cell_size,
            cell_size,
          );
        }
      }
    }
  }

  fn draw_hold_piece(&self) {
    if let Some(ref held_piece) = self.game_state.held_piece {
      let cell_size = self.game_state.board.cell_size as f64;
      let board_width = self.game_state.board.width;

      // Step 1: Draw "Hold" box and label
      self.draw_hold_box(&self.ctx, cell_size, board_width);

      // Step 2: Center and draw the held piece
      let hold_x_offset = (board_width + 1) as f64 * cell_size;
      let hold_y_offset = cell_size * 4.0;
      let box_width = cell_size * 5.0;
      let box_height = cell_size * 5.0;
      self.draw_piece_centered(
        &self.ctx,
        held_piece,
        hold_x_offset,
        hold_y_offset,
        box_width,
        box_height,
        cell_size,
      );
    }
  }
}
