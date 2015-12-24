extern crate sdl2;
extern crate rand;

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::thread;
use std::f32::consts::PI;
use rand::Rng;
use std::rc::Rc;

struct Environment {
  particles: Vec<Particle>
}

struct Particle {
  x: f32,
  y: f32,
  d: f32,  //direction
  v: f32,  //velocity
  e: f32,  //energy
  dna: Vec<u32>,
  cp: usize
}

impl Default for Particle {
  fn default () -> Particle {
    Particle {
      x : 400.0,
      y : 300.0,
      d : 0.0,
      v : 5.0,
      e : 10.0,
      dna: vec!(0),
      cp : 0,
    }
  }
}

impl Environment {

  fn generate_particles(&mut self) {
    let mut rotate_degree = 0.0;
    for i in 0..63 {
      rotate_degree = (rotate_degree + 0.1) % 360.0;
      let mut p = Particle { d: rotate_degree, ..Default::default() };
      p.generate();
      self.particles.push(p);
    }
  }

  fn tick(&mut self) {
    for mut particle in self.particles.iter_mut() {
      particle.tock();
    }
  }

}

impl Particle {

  fn tock(&mut self) {
    self.x += self.d.sin() * self.v;
    self.y += self.d.cos() * self.v;
    if self.x > 784.0 || self.x < 0.0 {
      self.d = self.d - (1.5 * PI) % (2.0*PI);
      self.e += 1.0;
    }
    if self.y > 568.0 || self.y < 0.0 {
      self.d = self.d - (1.0 * PI) % (2.0*PI);
      self.e += 1.0;
    }
    self.v *= 0.99;
    self.transcriptase();
  }

  fn generate(&mut self) {
    let mut rng = rand::thread_rng();
    for z in 0..128 {
      self.dna.push(rng.gen::<u32>() % 100);
    }
  }

  fn transcriptase(&mut self) {
    if self.e > 0.0 {
      match self.dna[self.cp] {
        0 => {},  // nothing
        1 => { self.d += 0.25 }, // turn counter-clockwise
        2 => { self.d -= 0.25 }, // turn clockwise
        3 => {
          self.e -= 0.5;
          self.v += 1.0;
        }
        _ => println!("Unknown Code"),
      }
      self.cp = (self.cp + 1) % self.dna.len();
    }
    let mut rng = rand::thread_rng();
    if rng.gen() {
      let mutate_cp = rng.gen::<usize>() % self.dna.len();
      let mutate_dna = rng.gen::<u32>() % 20;
      self.dna[mutate_cp] = mutate_dna;
    }
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
          let fy = y as f32 / 2.0;
          for x in 0..16 {
              let offset = y*pitch + x*3;
              let fx = x as f32 / 8.0;
              println!("fx {}, fy {}", fx, fy);
              buffer[offset + 0] = 0 as u8;
              buffer[offset + 1] = (fx.sin() * 256.0) as u8;
              buffer[offset + 2] = (fy.sin() * 256.0) as u8;
          }
      }
  }).unwrap();

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
    renderer.clear();
    // renderer.copy(&texture, None, Some(Rect::new_unwrap(100, 100, 16, 16)));
    env.tick();
    for particle in env.particles.iter() {
      renderer.copy_ex(&texture, None,
        Some(Rect::new_unwrap(particle.x as i32, particle.y as i32, 16, 16)), particle.d as f64, None, (false, false)
      );
    }
    renderer.present();
  }
}
