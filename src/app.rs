use opengl_graphics::GlGraphics;
use piston::ButtonArgs;

use piston::input::{Button, RenderArgs, UpdateArgs};
use piston::Key;

use crate::{fruit::Fruit, snake::Snake};

pub struct App {
    pub(crate) gl: GlGraphics,
    pub(crate) snake: Snake,
    pub(crate) fruit: Fruit,
    pub(crate) lost: bool,
}

impl App {
    pub(crate) fn input(&mut self, args: &ButtonArgs) {
        if self.lost {
            match args.button {
                Button::Keyboard(Key::R) => {
                    self.snake.reset();
                    self.lost = false;
                }
                _ => {}
            };
        }

        self.snake.input(&args);
    }

    pub(crate) fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |_c, gl| {
            clear([0.0, 0.0, 0.0, 1.0], gl);
        });

        self.snake.render(&mut self.gl, args);
        self.fruit.render(&mut self.gl, args);
    }

    pub(crate) fn update(&mut self, args: &UpdateArgs) {
        self.snake.update(&args);
        let snake_head = self.snake.body.front().unwrap().clone();
        let body_clone: Vec<(i32, i32)> = self.snake.body.clone().into_iter().collect();

        let body_without_head: Vec<(i32, i32)> = body_clone[1..].to_vec();
        let head_intersects_body = body_without_head
            .into_iter()
            .any(|(x, y)| x == snake_head.0 && y == snake_head.1);

        if head_intersects_body {
            self.lost = true;
            return;
        }

        if snake_head.0 == self.fruit.pos_x && snake_head.1 == self.fruit.pos_y {
            self.fruit.update(&body_clone);
            self.snake.add_new_part();
        }
    }
}
