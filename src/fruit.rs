
use opengl_graphics::GlGraphics;
use piston::RenderArgs;
use rand::Rng;

pub struct Fruit {
  pub(crate) pos_x: i32,
  pub(crate) pos_y: i32,
}

impl Fruit {
  pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
      let blue: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
      gl.draw(args.viewport(), |ctx, gl| {
          let transform = ctx.transform;
          let square=  graphics::rectangle::square(self.pos_x as f64, self.pos_y as f64, 50_f64);

          graphics::rectangle(blue, square, transform, gl);
      })
  }

  pub fn update(&mut self, snake_body: &Vec<(i32, i32)>) {
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