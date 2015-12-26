use std::f32::consts::PI;
use rand;

pub struct Environment {
  pub particles: Vec<Particle>
}

impl Environment {

  pub fn generate_particles(&mut self) {
    for i in 0..72 {
      let f = (i * 5) as f32;
      let mut p = Particle { d: f * (PI/180.0), ..Default::default() };
      p.generate();
      self.particles.push(p);
    }
  }

  pub fn tick(&mut self) {
    // let len = self.particles.len();
    // let random_i = rand::random::<usize>() % len;
    // let particles = self.particles;
    // let random_particle = self.particles.get_mut(random_i).unwrap();
    // for particle in self.particles.iter_mut() {
    //   match particle.neighbor {
    //     Some(neighbor_i) => {
    //       let a = particle.x - random_particle.x;
    //       let b = particle.y - random_particle.y;
    //       let random_distance = (a.powi(2)+b.powi(2)).sqrt();
    //       let neighbor = self.particles.get(neighbor_i).unwrap();
    //       let a = particle.x - neighbor.x;
    //       let b = particle.y - neighbor.y;
    //       let neighbor_distance = (a.powi(2)+b.powi(2)).sqrt();
    //       if neighbor_distance > random_distance {
    //         particle.neighbor = Some(random_i);
    //       }
    //     }
    //     None => {
    //       particle.neighbor = Some(random_i);
    //     }
    //   }
    // }
    for mut particle in self.particles.iter_mut() {
      particle.tock();
      if particle.e < 0.1 { particle.dead = true; }
    }
  }

}

pub struct Particle {
  pub x: f32,
  pub y: f32,
  pub d: f32,  //direction
  pub v: f32,  //velocity
  pub e: f32,  //energy
  pub dead: bool,
  pub dna: Vec<u32>,
  pub cp: usize
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
      dead: false
    }
  }
}

impl Particle {

  fn tock(&mut self) {
    self.x += self.d.cos() * self.v;
    self.y += self.d.sin() * self.v;
    if self.x > 784.0 || self.x < 0.0 {
      self.d = PI - self.d;
      self.v += self.v.sqrt();
      self.e -= 0.5;
    }
    if self.y > 568.0 || self.y < 0.0 {
      self.d = self.d * -1.0;
      self.v += self.v.sqrt();
      self.e -= 0.5;
    }
    self.v *= 0.99;
    if !self.dead { self.transcriptase() }
    self.d = self.d % 52.0;
  }

  fn generate(&mut self) {
    for _ in 0..128 {
      self.dna.push(rand::random::<u32>() % 100);
    }
  }

  fn transcriptase(&mut self) {
    if self.e > 0.0 {
      match self.dna[self.cp] {
        0 => {},  // nothing
        1 => { self.d += 0.25 }, // turn counter-clockwise
        2 => { self.d -= 0.25 }, // turn clockwise
        3 => { // moves
          self.v += 1.0;
          self.e -= 0.5;
        }
        _ => {},
      }
      self.cp = (self.cp + 1) % self.dna.len();
    }
    self.mutate();
  }

  fn mutate(&mut self) {
    if rand::random::<bool>() {
      let mutate_cp = rand::random::<usize>() % self.dna.len();
      let mutate_dna = rand::random::<u32>() % 20;
      self.dna[mutate_cp] = mutate_dna;
    }
  }

}