use std::collections::LinkedList;

use opengl_graphics::GlGraphics;
use piston::Key;
use piston::input::{RenderArgs, UpdateArgs, Button};
use piston::ButtonArgs;

use crate::direction::Direction;

pub struct Snake {
  pub(crate) body: LinkedList<(i32, i32)>,
  pub(crate) current_dir: Direction
}

impl Snake {
  pub fn input(&mut self, args: &ButtonArgs) {
      let (x, y) = self.body.front().unwrap().clone();

      if x < 0 || x >= 500 {
          return;
      }

      if y < 0 || y >= 500 {
          return;
      }

      match args.button {
          Button::Keyboard(Key::Up) if self.current_dir != Direction::Down => self.current_dir = Direction::Up,
          Button::Keyboard(Key::Down) if self.current_dir != Direction::Up => self.current_dir = Direction::Down,
          Button::Keyboard(Key::Right) if self.current_dir != Direction::Left => self.current_dir = Direction::Right,
          Button::Keyboard(Key::Left) if self.current_dir != Direction::Right => self.current_dir = Direction::Left,
          _ => {}
      }
  }

  pub fn add_new_part(&mut self) {
      let current_head = self.body.front().unwrap().clone();
      let new_part = match self.current_dir {
          Direction::Up => (current_head.0, current_head.1 + 50),
          Direction::Down => (current_head.0, current_head.1 - 50),
          Direction::Right => (current_head.0 - 50, current_head.1),
          Direction::Left => (current_head.0 + 50, current_head.1),
      };

      self.body.push_back(new_part);
  }

  pub fn reset(&mut self) {
      self.body = LinkedList::from_iter((vec![(0,0), (0, 50)]).into_iter());
      self.current_dir = Direction::Right;
  }

  pub fn update(&mut self, args: &UpdateArgs) {
      let mut new_head = (*self.body.front().expect("Snake has no body")).clone();

      match self.current_dir {
          Direction::Left => {
              if new_head.0 - 50 < 0 {
                  new_head.0 = 450;
              } else {
                  new_head.0 -= 50
              }

          },
          Direction::Right => {
              if new_head.0 + 50 > 500 {
                  new_head.0 = 0;
              } else {
                  new_head.0 += 50;
              }
          },
          Direction::Up => {
              if new_head.1 - 50 < 0 {
                  new_head.1 = 450;
              } else {
                  new_head.1 -= 50;
              }
          },
          Direction::Down => {
              if new_head.1 + 50 > 500 {
                  new_head.1 = 0;
              } else {
                  new_head.1 += 50;
              }
          },
      }

      self.body.push_front(new_head);
      self.body.pop_back().unwrap();
  }

  pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
      let red: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
      let test: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

      let squares: Vec<graphics::types::Rectangle> = self.body
          .iter()
          .map(|&(x, y)| graphics::rectangle::square(x as f64, y as f64, 50_f64))
          .collect();

      gl.draw(args.viewport(), |ctx, gl| {
          let transform = ctx.transform;

          for (index, square) in squares.into_iter().enumerate() {
              let color = if index == 0 { test } else { red };
              graphics::rectangle(color, square, transform, gl);
          }
      });
  }
}