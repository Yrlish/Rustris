mod shape;
mod piece;
mod board;

use crate::shape::Shape;
use rand::seq::SliceRandom;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d, Document, HtmlCanvasElement, KeyboardEvent, Window,
};
use crate::board::Board;
use crate::piece::Direction::{Down, Left, Right};
use crate::piece::Piece;

#[wasm_bindgen]
pub struct Tetris {
    board: Board,
    ctx: CanvasRenderingContext2d,
    current_piece: Piece,
    shapes: Vec<Shape>,
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

        let board = Board {
            width: 10,
            height: 20,
            grid: vec![vec![0; 10]; 20]
        };

        let shapes = shape::get_tetris_shapes();
        let current_shape = shapes.first().expect("No shapes").clone();
        let current_piece = Piece {
            shape: current_shape.clone(),
            x: board.width / 2 - current_shape.width / 2,
            y: 0,
        };

        // Create an empty board
        Ok(Tetris {
            board,
            ctx,
            current_piece,
            shapes,
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
        if self.current_piece.can_move(Down, &self.board) {
            self.move_down();
        } else {
            self.merge_shape_into_board();
            self.spawn_new_block();
        }

        self.draw();
    }

    fn merge_shape_into_board(&mut self) {
        for y in 0..self.current_piece.shape.height {
            for x in 0..self.current_piece.shape.width {
                if self.current_piece.shape.cells[y as usize][x as usize] == 1 {
                    self.board.grid[(self.current_piece.y + y) as usize][(self.current_piece.x + x) as usize] = 1;
                }
            }
        }
    }

    // Spawn a new block at the top of the board
    fn spawn_new_block(&mut self) {
        use rand::thread_rng;

        let mut rng = thread_rng();

        if let Some(new_shape) = self.shapes.choose(&mut rng) {
            self.current_piece = Piece {
                shape: new_shape.clone(),
                x: self.board.width / 2 - new_shape.width / 2,
                y: 0,
            }
        } else {
            panic!("No shapes!")
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
        self.current_piece.shape = self.current_piece.shape.rotate();
        self.draw();
    }

    // Draw the entire board, including the falling block
    #[allow(deprecated)]
    fn draw(&self) {
        let cell_width = self.ctx.canvas().unwrap().width() as f64 / self.board.width as f64;
        let cell_height = self.ctx.canvas().unwrap().height() as f64 / self.board.height as f64;

        // Clear the canvas
        self.ctx.set_fill_style(&JsValue::from_str("white"));
        self.ctx.fill_rect(
            0.0,
            0.0,
            self.ctx.canvas().unwrap().width() as f64,
            self.ctx.canvas().unwrap().height() as f64,
        );

        // Render the board and active blocks
        for y in 0..self.board.height {
            for x in 0..self.board.width {
                if self.board.grid[y as usize][x as usize] == 1 {
                    // Draw filled block
                    self.ctx.set_fill_style(&JsValue::from_str("blue"));
                } else {
                    // Grid-style empty cells
                    self.ctx.set_fill_style(&JsValue::from_str("lightgray"));
                }
                self.ctx.fill_rect(
                    x as f64 * cell_width,
                    y as f64 * cell_height,
                    cell_width,
                    cell_height,
                );

                // Draw border for clarity
                self.ctx.set_stroke_style(&JsValue::from_str("black"));
                self.ctx.stroke_rect(
                    x as f64 * cell_width,
                    y as f64 * cell_height,
                    cell_width,
                    cell_height,
                );
            }
        }

        for y in 0..self.current_piece.shape.height {
            for x in 0..self.current_piece.shape.width {
                if self.current_piece.shape.cells[y as usize][x as usize] == 1 {
                    self.ctx.set_fill_style(&JsValue::from_str("green"));
                    self.ctx.fill_rect(
                        (self.current_piece.x + x) as f64 * cell_width,
                        (self.current_piece.y + y) as f64 * cell_height,
                        cell_width,
                        cell_height,
                    );
                }
            }
        }
    }
}
