mod board;
mod game_renderer;
mod game_state;
mod piece;
mod shape;

use crate::game_renderer::GameRenderer;
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

    canvas.set_width((game_state.board.width + 7) as u32 * game_state.board.cell_size as u32);
    canvas.set_height((game_state.board.height) as u32 * game_state.board.cell_size as u32);

    Ok(Tetris { ctx, game_state })
  }

  pub fn start_game(self) {
    let game_ref = Rc::new(RefCell::new(self));

    let tick_game_clone = Rc::clone(&game_ref);
    let tick_closure = Closure::wrap(Box::new(move || {
      tick_game_clone.borrow_mut().game_tick();
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
  fn game_tick(&mut self) {
    if !self.game_state.game_over {
      self.game_state.state_tick();
    }

    self.render();
  }

  // Move the falling block left
  pub fn move_left(&mut self) {
    self
      .game_state
      .current_piece
      .move_piece(Left, &self.game_state.board);

    self.render();
  }

  // Move the falling block right
  pub fn move_right(&mut self) {
    self
      .game_state
      .current_piece
      .move_piece(Right, &self.game_state.board);

    self.render();
  }

  fn move_down(&mut self) {
    self
      .game_state
      .current_piece
      .move_piece(Down, &self.game_state.board);

    self.render();
  }

  fn hold_piece(&mut self) {
    self.game_state.hold_piece();
    self.render();
  }

  fn rotate_piece(&mut self) {
    self
      .game_state
      .current_piece
      .rotate_piece(&self.game_state.board);

    self.render();
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

    self.render();
  }

  fn render(&self) {
    let canvas = &self.ctx;
    GameRenderer::render(canvas, &self.game_state);
  }
}
