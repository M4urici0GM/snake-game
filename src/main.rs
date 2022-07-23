
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::collections::LinkedList;
use glutin_window::GlutinWindow as Window;
use graphics::Transformed;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{ButtonArgs, ButtonEvent, EventLoop, Key};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, Button};
use piston::window::WindowSettings;
use piston_window::PistonWindow;
use rand::Rng;

pub struct App {
    gl: GlGraphics,
    snake: Snake,
    fruit: Fruit,
    lost: bool,
}

#[derive(Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}


pub struct Snake {
    body: LinkedList<(i32, i32)>,
    current_dir: Direction
}

pub struct Fruit {
    pos_x: i32,
    pos_y: i32,
}

impl App {
    pub(crate) fn input(&mut self, args: &ButtonArgs) {
        match args.button {
            Button::Keyboard(Key::R) => {
                self.snake.reset();
                self.lost = false;
            },
            _ => {}
        };
        self.snake.input(&args);
    }
}


impl Fruit {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        let blue: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        gl.draw(args.viewport(), |ctx, gl| {
            let transform = ctx.transform;
            let square=  graphics::rectangle::square(self.pos_x as f64, self.pos_y as f64, 50_f64);

            graphics::rectangle(blue, square, transform, gl);
        })
    }

    fn update(&mut self, snake_body: &Vec<(i32, i32)>) {
        let mut rng = rand::thread_rng();
        let mut y = rng.gen_range(0..10) * 50;
        let mut x = rng.gen_range(0..10) * 50;

        while snake_body.into_iter().any(|(cur_x, cur_y)| *cur_x == x && *cur_y == y) {
            y = rng.gen_range(0..10) * 50;
            x = rng.gen_range(0..10) * 50;
        }

        self.pos_x = x;
        self.pos_y = y;
    }
}

impl Snake {
    fn input(&mut self, args: &ButtonArgs) {
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

    fn add_new_part(&mut self) {
        let current_head = self.body.front().unwrap().clone();
        let new_part = match self.current_dir {
            Direction::Up => (current_head.0, current_head.1 + 50),
            Direction::Down => (current_head.0, current_head.1 - 50),
            Direction::Right => (current_head.0 - 50, current_head.1),
            Direction::Left => (current_head.0 + 50, current_head.1),
        };

        self.body.push_back(new_part);
    }

    fn reset(&mut self) {
        self.body = LinkedList::from_iter((vec![(0,0), (0, 50)]).into_iter());
        self.current_dir = Direction::Right;
    }

    fn update(&mut self, args: &UpdateArgs) {
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

    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
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

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |_c, gl| {
            clear([0.0, 0.0, 0.0, 1.0], gl);
        });

        self.snake.render(&mut self.gl, args);
        self.fruit.render(&mut self.gl, args);
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.snake.update(&args);
        let snake_head = self.snake.body.front().unwrap().clone();
        let body_clone: Vec<(i32, i32)> = self.snake.body.clone().into_iter().collect();

        let body_without_head: Vec<(i32, i32)> = body_clone[1..].to_vec();
        if body_without_head.into_iter().any(|(x, y)| x == snake_head.0 && y == snake_head.1) {
            self.lost = true;
            return;
        }

        if snake_head.0 == self.fruit.pos_x && snake_head.1 == self.fruit.pos_y {
            self.fruit.update(&body_clone);
            self.snake.add_new_part();
        }
    }
}

fn main() {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let ref font = assets.join("JetBrainsMono-Regular.ttf");

    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("spinning-square", [500, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    window.set_lazy(true);
    let mut glyphs = window.load_font(font).unwrap();


    let mut rng = rand::thread_rng();
    let mut app = App {
        gl: GlGraphics::new(opengl),
        lost: false,
        fruit: Fruit {
            pos_x: rng.gen_range(0..10) * 50,
            pos_y: rng.gen_range(0..10) * 50,
        },
        snake: Snake {
            body: LinkedList::from_iter((vec![(0,0), (0, 50)]).into_iter()),
            current_dir: Direction::Right,
        },
    };

    let mut events = Events::new(EventSettings::new()).ups(10);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.button_args() {
            app.input(&args);
        }

        if app.lost {
            window.draw_2d(&e, |c, g, device| {
                use graphics::*;

                let transform = c.transform.trans(100.0, 100.0);
                Text::new_color([1.0, 1.0, 1.0, 1.0], 15).draw(
                    "You lost! Press R to reset.",
                    &mut glyphs,
                    &c.draw_state,
                    transform,
                    g
                ).unwrap();
                // Update glyphs before rendering.
                glyphs.factory.encoder.flush(device);
            });
            continue;
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.render_args() {
            app.render(&args);
        }
    }
}
