use rand::prelude::SliceRandom;
use rand::thread_rng;

#[derive(Clone)]
pub struct Shape {
  pub cells: Vec<Vec<u8>>, // 2D matrix representation of the shape
  pub width: u8,
  pub height: u8,
  pub color: Color,
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

impl Shape {
  // Rotate the shape clockwise
  pub fn rotate(&self) -> Shape {
    let new_cells = (0..self.width)
      .map(|x| {
        (0..self.height)
          .map(|y| self.cells[(self.height - 1 - y) as usize][x as usize])
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
