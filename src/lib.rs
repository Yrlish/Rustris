mod board;
mod game_state;
mod piece;
mod shape;

use crate::game_state::GameState;
use crate::piece::Direction::{Down, Left, Right};
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

    canvas.set_width((game_state.board.width + 5) as u32 * game_state.board.cell_size as u32);
    canvas.set_height((game_state.board.height + 5) as u32 * game_state.board.cell_size as u32);

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

    canvas.clear_rect(
      0.0,
      0.0,
      canvas.canvas().unwrap().width() as f64,
      canvas.canvas().unwrap().height() as f64,
    );

    board.draw_pieces(canvas);
    self.game_state.current_piece.draw_ghost(canvas, board);
    self.game_state.current_piece.draw(canvas, board);
    board.draw_grid(canvas);

    // Display the current score
    self.ctx.set_fill_style(&JsValue::from_str("black"));
    self.ctx.set_font("20px Arial");
    self
      .ctx
      .fill_text(
        &format!("Score: {}", self.game_state.score),
        board.width as f64 * cell_size + 10.0,
        50.0,
      )
      .unwrap();

    if let Some(ref held_piece) = self.game_state.held_piece {
      // Offset the held piece display area
      let hold_x = (board.width + 1) as f64 * cell_size;
      let hold_y = cell_size * 3.0;

      let color = held_piece.shape.color.to_rgba(1.0);
      canvas.set_fill_style(&JsValue::from_str(&color));

      for y in held_piece.shape.iter_height() {
        for x in held_piece.shape.iter_width() {
          if held_piece.shape.cells[y][x] == 1 {
            canvas.fill_rect(
              hold_x + x as f64 * cell_size,
              hold_y + y as f64 * cell_size,
              cell_size,
              cell_size,
            );
          }
        }
      }

      // Add "Hold" label
      canvas.set_fill_style(&JsValue::from_str("black"));
      canvas
        .fill_text("Hold", hold_x + cell_size, hold_y - 10.0)
        .unwrap();
    }
  }
}
