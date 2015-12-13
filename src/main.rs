extern crate sdl2;

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::thread;

struct Particle {
  x: f64,
  y: f64,
  d: f64,
}

impl Particle {
  fn next(&mut self) {
    self.x += self.d.sin() * 5.0;
    self.y += self.d.cos() * 5.0;
  }

  fn reset(&mut self) {
    self.x = 400.0;
    self.y = 300.0;
  }
}

fn main() {
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();

  let window = video_subsystem.window("RAHN DOM", 800, 600)
    .position_centered().opengl()
    .build().unwrap();
  let mut event_pump = sdl_context.event_pump().unwrap();
  let mut renderer = window.renderer().present_vsync().build().unwrap();

  let mut texture = renderer.create_texture_streaming(PixelFormatEnum::RGB24, (16, 16)).unwrap();
  // Create a red-green gradient
  texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
      for y in 0..16 {
          for x in 0..16 {
              let offset = y*pitch + x*3;
              buffer[offset + 0] = 0 as u8;
              buffer[offset + 1] = x as u8 * 16;
              buffer[offset + 2] = y as u8 * 16;
          }
      }
  }).unwrap();

  let mut rotate_degree = 0.0;
  let mut particles: Vec<Particle> = Vec::new();
  for i in 0..63 {
    rotate_degree = (rotate_degree + 0.1) % 360.0;
    particles.push(Particle { x: 400.0, y: 300.0, d: rotate_degree });
  }

  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
          break 'running
        },
        Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
          for particle in particles.iter_mut() {
            particle.reset();
          }
        }
        _ => {}
      }
    };
    renderer.clear();
    // renderer.copy(&texture, None, Some(Rect::new_unwrap(100, 100, 16, 16)));
    for particle in particles.iter_mut() {
      particle.next();
      renderer.copy_ex(&texture, None,
        Some(Rect::new_unwrap(particle.x as i32, particle.y as i32, 16, 16)), particle.d, None, (false, false)
      );
    }
    renderer.present();
  }
}
