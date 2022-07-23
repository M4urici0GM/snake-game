
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod snake;
mod direction;
mod fruit;
mod app;

use std::collections::LinkedList;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{ButtonEvent, EventLoop};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston_window::{PistonWindow, Glyphs};
use rand::Rng;

use app::App;
use fruit::Fruit;
use snake::Snake;
use direction::Direction;


fn create_app(opengl: OpenGL) -> App {
    let mut rng = rand::thread_rng();
    App {
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
    }
}

fn create_window(opengl: &OpenGL) -> PistonWindow {
    let mut window: PistonWindow = WindowSettings::new("snake-game", [500, 500])
        .graphics_api(*opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    window.set_lazy(true);
    window
}

fn load_glyphs(window: &mut PistonWindow) -> Glyphs {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let font = assets.join("JetBrainsMono-Regular.ttf");
    window.load_font(font).unwrap()
}

fn start_game_loop(window: &mut PistonWindow, app: &mut App, glyphs: &mut Glyphs) {
    let event_settings = EventSettings::new();
    let mut events = Events::new(event_settings).ups(10);

    while let Some(e) = events.next(window) {
        if let Some(args) = e.button_args() {
            app.input(&args);
        }

        if app.lost {
            window.draw_2d(&e, |c, g, device| {
                use graphics::*;

                let transform = c.transform.trans(100.0, 100.0);
                Text::new_color([1.0, 1.0, 1.0, 1.0], 15).draw(
                    "You lost! Press R to reset.",
                    glyphs,
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


fn main() {
    let opengl = OpenGL::V3_2;
    let mut window = create_window(&opengl);
    let mut glyphs = load_glyphs(&mut window);
    let mut app = create_app(opengl);

    start_game_loop(&mut window, &mut app, &mut glyphs);
}
