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

  pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
    let cell_width = ctx.canvas().unwrap().width() as f64 / self.width as f64;
    let cell_height = ctx.canvas().unwrap().height() as f64 / self.height as f64;

    for y in 0..self.height {
      for x in 0..self.width {
        let color = match self.grid[y as usize][x as usize] {
          Color::None => "lightgray",
          Color::Cyan => "cyan",
          Color::Yellow => "yellow",
          Color::Purple => "purple",
          Color::Green => "green",
          Color::Red => "red",
          Color::Blue => "blue",
          Color::Orange => "orange",
        };

        ctx.set_fill_style(&JsValue::from_str(color));
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
