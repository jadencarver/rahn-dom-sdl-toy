extern crate rand;
extern crate sdl2;
extern crate sdl2_image;

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::path::Path;
use sdl2::rect::Rect;
use sdl2_image::{LoadTexture, INIT_PNG, INIT_JPG};
use std::f32::consts::PI;

mod environment;
use environment::Environment;


fn main() {
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  sdl2_image::init(INIT_PNG | INIT_JPG);

  let window = video_subsystem.window("RAHN DOM", 800, 600)
    .position_centered().opengl()
    .build().unwrap();
    
  let mut event_pump = sdl_context.event_pump().unwrap();
  let mut renderer = window.renderer().present_vsync().build().unwrap();

  let live_texture = renderer.load_texture(Path::new("images/bug.png")).unwrap();
  let dead_texture = renderer.load_texture(Path::new("images/bug_dead.png")).unwrap();

  // for i in 0..360 {
  //   let f = i as f32;
  //   let r = f * (PI/180.0);
  //   println!("Angle {} => Radians {} : Sine: {} - Cosine: {}", f, r, r.sin(), r.cos());
  // }

  let mut env = Environment { particles: vec!() };
  env.generate_particles();

  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
          break 'running
        },
        _ => {}
      }
    };
    renderer.set_draw_color(Color::RGB(22,91,38));
    renderer.clear();
    env.tick();
    for particle in env.particles.iter() {
      let rect = Some(Rect::new_unwrap(particle.x as i32, particle.y as i32, 48, 48));
      let r = (particle.d * (180.0/PI)) as f64;
      if particle.dead {
        renderer.copy_ex(&dead_texture, None, rect, r, None, (false, false));
      } else {
        renderer.copy_ex(&live_texture, None, rect, r, None, (false, false));
      }
    }
    renderer.present();
  }
}
