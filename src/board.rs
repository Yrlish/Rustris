use crate::shape::Color;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct Board {
  pub width: u8,
  pub height: u8,
  pub grid: Vec<Vec<Color>>,
}

#[allow(deprecated)]
impl Board {
  pub fn new(width: u8, height: u8) -> Self {
    Board {
      width,
      height,
      grid: vec![vec![Color::None; width as usize]; height as usize], // Initialize entire grid as "None"
    }
  }

  pub fn clear_full_lines(&mut self) -> u8 {
    let mut cleared_rows = 0;

    // Retain only rows that are not full and count the cleared rows
    self.grid.retain(|row| {
      if row.iter().all(|&cell| cell != Color::None) {
        cleared_rows += 1;
        false // Remove this row (it's full)
      } else {
        true // Keep this row
      }
    });

    // Add empty rows at the top to maintain the board's size
    for _ in 0..cleared_rows {
      self.grid.insert(0, vec![Color::None; self.width as usize]);
    }

    cleared_rows
  }

  pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
    let cell_width = ctx.canvas().unwrap().width() as f64 / self.width as f64;
    let cell_height = ctx.canvas().unwrap().height() as f64 / self.height as f64;

    for y in 0..self.height {
      for x in 0..self.width {
        let color = self.grid[y as usize][x as usize].to_rgba(1.0);

        ctx.set_fill_style(&JsValue::from_str(&color));
        ctx.fill_rect(
          x as f64 * cell_width,
          y as f64 * cell_height,
          cell_width,
          cell_height,
        );
      }
    }
  }
}
