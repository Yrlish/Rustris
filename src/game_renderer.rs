use crate::game_state::GameState;
use crate::piece::{Direction, Piece};
use web_sys::CanvasRenderingContext2d;

pub struct GameRenderer {}

impl GameRenderer {
  pub fn render(canvas: &CanvasRenderingContext2d, game_state: &GameState) {
    let board_width = game_state.board.width as u16 * game_state.board.cell_size as u16;
    let cell_width = game_state.board.cell_size;

    Self::clear_canvas(canvas);
    Self::render_background(canvas, board_width);

    // Board
    Self::render_board_with_pieces(canvas, &game_state);

    // Score
    Self::render_score(
      canvas,
      game_state.score,
      board_width + cell_width as u16,
      (cell_width as f32 * 1.25) as u16,
    );

    // Held Piece
    Self::render_hold(canvas, &game_state.held_piece, cell_width, board_width);

    // Next Piece
    Self::render_next(canvas, &game_state.next_piece, cell_width, board_width);

    // Game Over screen
    if game_state.game_over {
      Self::render_game_over(canvas);
    }
  }

  fn clear_canvas(canvas: &CanvasRenderingContext2d) {
    canvas.clear_rect(
      0.0,
      0.0,
      canvas.canvas().unwrap().height() as f64,
      canvas.canvas().unwrap().height() as f64,
    );
  }

  fn render_background(canvas: &CanvasRenderingContext2d, board_width: u16) {
    canvas.set_fill_style_str("#333333");
    canvas.fill_rect(
      0.0,
      0.0,
      board_width as f64,
      canvas.canvas().unwrap().height() as f64,
    );

    canvas.set_fill_style_str("#444444");
    canvas.fill_rect(
      board_width as f64,
      0.0,
      canvas.canvas().unwrap().width() as f64,
      canvas.canvas().unwrap().height() as f64,
    );
  }

  fn render_board_with_pieces(canvas: &CanvasRenderingContext2d, game_state: &GameState) {
    let board = &game_state.board;
    let piece = &game_state.current_piece;
    let cell_size = board.cell_size as f64;

    // All pieces in grid
    for y in board.iter_height() {
      for x in board.iter_width() {
        let cell_color = board.grid[y][x].to_rgba(1.0);

        canvas.set_fill_style_str(&cell_color);
        canvas.fill_rect(
          x as f64 * cell_size,
          y as f64 * cell_size,
          cell_size,
          cell_size,
        );
      }
    }

    // The current falling piece
    let piece_color = piece.shape.color.to_rgba(1.0);
    canvas.set_fill_style_str(&piece_color);
    Self::render_a_piece(canvas, cell_size, piece);

    // The ghost of the falling piece
    let mut ghost_piece = piece.clone();

    while ghost_piece.can_move(Direction::Down, board) {
      ghost_piece.y += 1;
    }

    let ghost_color = ghost_piece.shape.color.to_rgba(0.3);
    canvas.set_fill_style_str(&ghost_color);
    Self::render_a_piece(canvas, cell_size, &ghost_piece);

    // Render gridlines
    Self::render_gridlines(canvas, 0, 0, board.width, board.height, board.cell_size);
  }

  fn render_a_piece(canvas: &CanvasRenderingContext2d, cell_size: f64, piece: &Piece) {
    for y in piece.shape.iter_height() {
      for x in piece.shape.iter_width() {
        if piece.shape.cells[y][x] == 1 {
          canvas.fill_rect(
            (piece.x + x as u8) as f64 * cell_size,
            (piece.y + y as u8) as f64 * cell_size,
            cell_size,
            cell_size,
          );
        }
      }
    }
  }

  fn render_gridlines(
    canvas: &CanvasRenderingContext2d,
    start_x: u16,
    start_y: u16,
    cells_x: u8,
    cells_y: u8,
    cell_size: u8,
  ) {
    canvas.set_stroke_style_str("#555555");

    for cell_y in 0..cells_y {
      let pos_y = start_y + (cell_y as u16 * cell_size as u16);
      for cell_x in 0..cells_x {
        let pos_x = start_x + (cell_x as u16 * cell_size as u16);

        canvas.stroke_rect(
          pos_x as f64,
          pos_y as f64,
          cell_size as f64,
          cell_size as f64,
        );
      }
    }
  }

  fn render_score(canvas: &CanvasRenderingContext2d, score: u32, pos_x: u16, pos_y: u16) {
    canvas.set_fill_style_str("white");
    canvas.set_font("20px 'Courier New', monospace");
    canvas
      .fill_text(&format!("Score: {}", score), pos_x as f64, pos_y as f64)
      .unwrap();
  }

  fn render_hold(
    canvas: &CanvasRenderingContext2d,
    held_piece: &Option<Piece>,
    cell_size: u8,
    board_width: u16,
  ) {
    let box_x = board_width + cell_size as u16;
    let box_y = cell_size as u16 * (4 + 5 + 2);
    let box_width = cell_size as u16 * 5;
    let box_height = cell_size as u16 * 5;

    canvas.set_fill_style_str("#222222");
    canvas.fill_rect(
      box_x as f64,
      box_y as f64,
      box_width as f64,
      box_height as f64,
    );

    canvas.set_fill_style_str("white");
    canvas.set_font("20px 'Courier New', monospace");
    canvas
      .fill_text(
        "Hold",
        box_x as f64 + (cell_size as f64 * 1.7),
        box_y as f64 - 10.0,
      )
      .unwrap();

    if let Some(ref piece) = held_piece {
      let piece_width = cell_size as u16 * piece.shape.width as u16;
      let piece_height = cell_size as u16 * piece.shape.height as u16;
      let piece_x = box_x + (box_width - piece_width) / 2;
      let piece_y = box_y + (box_height - piece_height) / 2;
      let color = piece.shape.color.to_rgba(1.0);

      for y in piece.shape.iter_height() {
        for x in piece.shape.iter_width() {
          if piece.shape.cells[y][x] == 1 {
            canvas.set_fill_style_str(&color);
            let cell_x = piece_x + (x as u16 * cell_size as u16);
            let cell_y = piece_y + (y as u16 * cell_size as u16);

            canvas.fill_rect(
              cell_x as f64,
              cell_y as f64,
              cell_size as f64,
              cell_size as f64,
            );

            Self::render_gridlines(canvas, cell_x, cell_y, 1, 1, cell_size);
          }
        }
      }
    }
  }

  fn render_next(
    canvas: &CanvasRenderingContext2d,
    next_piece: &Piece,
    cell_size: u8,
    board_width: u16,
  ) {
    let box_x = board_width + cell_size as u16;
    let box_y = cell_size as u16 * (4);
    let box_width = cell_size as u16 * 5;
    let box_height = cell_size as u16 * 5;

    canvas.set_fill_style_str("#222222");
    canvas.fill_rect(
      box_x as f64,
      box_y as f64,
      box_width as f64,
      box_height as f64,
    );

    canvas.set_fill_style_str("white");
    canvas.set_font("20px 'Courier New', monospace");
    canvas
      .fill_text(
        "Next",
        box_x as f64 + (cell_size as f64 * 1.7),
        box_y as f64 - 10.0,
      )
      .unwrap();

    let piece_width = cell_size as u16 * next_piece.shape.width as u16;
    let piece_height = cell_size as u16 * next_piece.shape.height as u16;
    let piece_x = box_x + (box_width - piece_width) / 2;
    let piece_y = box_y + (box_height - piece_height) / 2;
    let color = next_piece.shape.color.to_rgba(1.0);

    for y in next_piece.shape.iter_height() {
      for x in next_piece.shape.iter_width() {
        if next_piece.shape.cells[y][x] == 1 {
          canvas.set_fill_style_str(&color);
          let cell_x = piece_x + (x as u16 * cell_size as u16);
          let cell_y = piece_y + (y as u16 * cell_size as u16);

          canvas.fill_rect(
            cell_x as f64,
            cell_y as f64,
            cell_size as f64,
            cell_size as f64,
          );

          Self::render_gridlines(canvas, cell_x, cell_y, 1, 1, cell_size);
        }
      }
    }
  }

  fn render_game_over(canvas: &CanvasRenderingContext2d) {
    canvas.set_fill_style_str("red");
    canvas.set_font("30px Arial");
    canvas
      .fill_text(
        "Game Over",
        (canvas.canvas().unwrap().width() as f64 / 2.0) - 50.0,
        canvas.canvas().unwrap().height() as f64 / 2.0,
      )
      .unwrap();
  }
}
