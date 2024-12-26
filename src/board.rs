use crate::shape::Color;
use std::ops::Range;

pub struct Board {
  pub width: u8,
  pub height: u8,
  pub cell_size: u8,
  pub grid: Vec<Vec<Color>>,
}

#[allow(deprecated)]
impl Board {
  pub fn new(width: u8, height: u8) -> Self {
    Board {
      width,
      height,
      cell_size: 30,
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

  pub fn iter_height(&self) -> Range<usize> {
    0..self.height as usize
  }

  pub fn iter_width(&self) -> Range<usize> {
    0..self.width as usize
  }
}
