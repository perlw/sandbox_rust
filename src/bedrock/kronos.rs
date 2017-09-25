struct System {
  name: String,
  system: Box<HasSystem>,
}

pub trait HasSystem {
  fn start(&mut self);
  fn stop(&mut self);
  fn update(&mut self);
  fn message(&mut self);
}

pub struct Kronos {
  systems: Vec<System>,
}

impl Kronos {
  pub fn new() -> Kronos {
    Kronos {
      systems: Vec::new(),
    }
  }

  pub fn register<T: HasSystem+'static>(&mut self, name: &str, autostart: bool, system: T) {
    self.systems.push(System{
      name: name.to_owned(),
      system: Box::new(system),
    });
  }

  pub fn start_system(&mut self, name: &str) {
    for s in &mut self.systems {
      if s.name.as_str() == name {
        s.system.start();
      }
    }
  }

  pub fn stop_system(&mut self, name: &str) {
    for s in &mut self.systems {
      if s.name.as_str() == name {
        s.system.stop();
      }
    }
  }

  pub fn update(&self, delta: f64) {
  }
}
