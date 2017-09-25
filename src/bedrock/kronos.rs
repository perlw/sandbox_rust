struct System {
  name: String,
  system: Box<HasSystem>,
  running: bool,
  timing: f64,
  since_update: f64,
}

pub trait HasSystem {
  fn start(&mut self) -> bool;
  fn stop(&mut self) -> bool;
  fn update(&mut self, delta: f64);
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

  pub fn register<T: HasSystem+'static>(&mut self, name: &str, autostart: bool, timing: f64, system: T) {
    self.systems.push(System{
      name: name.to_owned(),
      running: false,
      timing,
      since_update: 0.0,
      system: Box::new(system),
    });

    if autostart {
      self.start_system(name);
    }
  }

  pub fn start_system(&mut self, name: &str) {
    for s in &mut self.systems {
      if !s.running && s.name.as_str() == name {
        if s.system.start() {
          s.running = true;
          println!("System {} started", name);
        } else {
          println!("System {} failed to start", name);
        }
      }
    }
  }

  pub fn stop_system(&mut self, name: &str) {
    for s in &mut self.systems {
      if s.running && s.name.as_str() == name {
        if s.system.stop() {
          s.running = false;
          println!("System {} stopped", name);
        } else {
          println!("System {} stop prevented", name);
        }
      }
    }
  }

  pub fn update(&mut self, delta: f64) {
    for s in &mut self.systems {
      if !s.running {
        continue;
      }

      s.since_update += delta;
      if s.since_update >= s.timing {
        while s.since_update >= s.timing {
          s.since_update -= s.timing;
          s.system.update(if s.timing > 0.0 { s.timing } else { delta });
        }
        s.since_update = 0.0;
      }
    }
  }
}

impl Drop for Kronos {
  fn drop(&mut self) {
    for s in &mut self.systems {
      if s.running {
        s.system.stop();
      }
    }
  }
}
