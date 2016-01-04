use std::f32::consts::PI;
use rand;

// fn split_owned_vec<T>(mut v: ~[T], index: uint) -> (~[T], ~[T]) {
//     assert!(index <= v.len());

//     let new_len = v.len() - index;
//     let mut new_v = vec::with_capacity(v.len() - index);
//     unsafe {
//         ptr::copy_nonoverlapping_memory(new_v.as_mut_ptr(), v.as_ptr().offset(index as int), new_len);
//         v.set_len(index);
//         new_v.set_len(new_len);
//     }

//     (v, new_v)
// }

pub struct Environment {
  pub particles: Vec<Particle>,
  pub dead_particles: Vec<Particle>,
}

impl Environment {

  pub fn new() -> Environment {
    Environment { particles: vec![], dead_particles: vec![] }
  }

  pub fn generate_particles(&mut self) {
    for i in 0..72 {
      let f = (i * 5) as f32;
      let mut p = Particle::new(f * (PI/180.0));
      p.generate();
      self.particles.push(p);
    }
  }

  pub fn tick(&mut self) {

    // ------ reproduce a random particle
    // let mut new_particle = None;
    // {
    //   let len = self.particles.len();
    //   let random_i = rand::random::<usize>() % len;
    //   let random_particle = self.particles.get(random_i).unwrap();
    //   if !random_particle.dead {
    //     new_particle = Some(Particle {
    //       x: random_particle.x,
    //       y: random_particle.y,
    //       d: random_particle.d,
    //       v: random_particle.v,
    //       ..Default::default()
    //     });
    //   }
    // }
    // match new_particle {
    //   Some(particle) => { self.particles.push(particle) }
    //   None => {}
    // }

    // ------ random particle distance
    // let len = self.particles.len();
    // let random_i = rand::random::<usize>() % len;
    // let random_particle = self.particles.get(random_i).unwrap();
    // for particle in self.particles.iter_mut() {
    //   match particle.neighbor {
    //     Some(neighbor) => {
    //       let a = particle.x - random_particle.x;
    //       let b = particle.y - random_particle.y;
    //       let random_distance = (a.powi(2)+b.powi(2)).sqrt();
    //       let a = particle.x - neighbor.x;
    //       let b = particle.y - neighbor.y;
    //       let neighbor_distance = (a.powi(2)+b.powi(2)).sqrt();
    //       if neighbor_distance > random_distance {
    //         particle.neighbor = Some(&random_particle);
    //       }
    //     }
    //     None => {
    //       particle.neighbor = Some(&random_particle);
    //     }
    //   }
    // }
    // let particles = &mut self.particles;
    for mut particle in self.particles.iter_mut() { particle.tock() };
    // let (dead, live): (Vec<Particle>, Vec<Particle>) = self.particles.into_iter().partition(|p: &Particle| p.dead);
    // return self
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

impl Particle {

  pub fn new (d: f32) -> Particle {
    Particle {
      x : 400.0,
      y : 300.0,
      d : d,
      v : 2.0,
      e : 10.0,
      dna: vec!(0),
      cp : 0,
      dead: false
    }
  }

  fn tock(&mut self) {
    self.x += self.d.cos() * self.v;
    self.y += self.d.sin() * self.v;
    if self.x > 752.0 || self.x < 0.0 {
      self.d = PI - self.d;
      self.v += 0.5;
      self.e -= 0.5;
    }
    if self.y > 552.0 || self.y < 0.0 {
      self.d = self.d * -1.0;
      self.v += 0.5;
      self.e -= 0.5;
    }
    self.v *= 0.99;
    if !self.dead && self.e < 0.1 { self.dead = true }
    if !self.dead { self.transcriptase() };
    self.d = self.d % 6.283185308;
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
