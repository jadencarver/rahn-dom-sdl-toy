mod particle;

pub struct Environment {
  particles: Vec<particle.Particle>
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
      if particle.e < 0.0 { particle.dead = true }
    }
  }

}