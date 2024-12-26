use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::ops::Range;

#[derive(Clone)]
pub struct Shape {
  pub cells: Vec<Vec<u8>>, // 2D matrix representation of the shape
  pub width: u8,
  pub height: u8,
  pub color: Color,
}

impl Shape {
  // Rotate the shape clockwise
  pub fn rotate(&self) -> Shape {
    let new_cells = self
      .iter_width()
      .map(|x| {
        self
          .iter_height()
          .map(|y| self.cells[(self.height - 1) as usize - y][x])
          .collect()
      })
      .collect();

    Shape {
      cells: new_cells,
      width: self.height,
      height: self.width,
      color: self.color,
    }
  }

  pub fn iter_height(&self) -> Range<usize> {
    0..self.height as usize
  }

  pub fn iter_width(&self) -> Range<usize> {
    0..self.width as usize
  }
}

pub fn get_random_shape<'a>() -> Shape {
  let mut rng = thread_rng();

  if let Some(new_shape) = get_tetris_shapes().choose(&mut rng) {
    new_shape.clone()
  } else {
    panic!("No shapes!")
  }
}

pub fn get_tetris_shapes() -> Vec<Shape> {
  vec![
    // O shape
    Shape {
      cells: vec![vec![1, 1], vec![1, 1]],
      width: 2,
      height: 2,
      color: Color::Yellow,
    },
    // I shape
    Shape {
      cells: vec![vec![1, 1, 1, 1]],
      width: 4,
      height: 1,
      color: Color::Cyan,
    },
    // T shape
    Shape {
      cells: vec![vec![0, 1, 0], vec![1, 1, 1]],
      width: 3,
      height: 2,
      color: Color::Purple,
    },
    // S shape
    Shape {
      cells: vec![vec![0, 1, 1], vec![1, 1, 0]],
      width: 3,
      height: 2,
      color: Color::Green,
    },
    // Z shape
    Shape {
      cells: vec![vec![1, 1, 0], vec![0, 1, 1]],
      width: 3,
      height: 2,
      color: Color::Red,
    },
    // J shape
    Shape {
      cells: vec![vec![1, 0, 0], vec![1, 1, 1]],
      width: 3,
      height: 2,
      color: Color::Blue,
    },
    // L shape
    Shape {
      cells: vec![vec![0, 0, 1], vec![1, 1, 1]],
      width: 3,
      height: 2,
      color: Color::Orange,
    },
  ]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
  Cyan,
  Yellow,
  Purple,
  Green,
  Red,
  Blue,
  Orange,
  None, // Represents an empty cell in the grid
}

impl Color {
  pub fn to_rgba(&self, opacity: f64) -> String {
    match self {
      Color::Cyan => format!("rgba(0, 255, 255, {})", opacity), // Cyan
      Color::Yellow => format!("rgba(255, 255, 0, {})", opacity), // Yellow
      Color::Purple => format!("rgba(128, 0, 128, {})", opacity), // Purple
      Color::Green => format!("rgba(0, 255, 0, {})", opacity),  // Green
      Color::Red => format!("rgba(255, 0, 0, {})", opacity),    // Red
      Color::Blue => format!("rgba(0, 0, 255, {})", opacity),   // Blue
      Color::Orange => format!("rgba(255, 165, 0, {})", opacity), // Orange
      Color::None => format!("rgba(200, 200, 200, {})", opacity), // Default for empty
    }
  }
}
