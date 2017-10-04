struct SystemState<T> {
    name: String,
    system: Box<System<T>>,
    running: bool,
    timing: f64,
    since_update: f64,
}

pub trait System<T> {
    fn start(&mut self) -> bool;
    fn stop(&mut self) -> bool;
    fn update(&mut self, delta: f64);
    fn message(&mut self, msg: &T);
}

pub struct Kronos<T> {
    systems: Vec<SystemState<T>>,
}

#[allow(dead_code)]
impl<T> Kronos<T> {
    pub fn new() -> Kronos<T> {
        Kronos::<T> {
            systems: Vec::new(),
        }
    }

    pub fn register<S: System<T> + 'static>(
        &mut self,
        name: &str,
        autostart: bool,
        timing: f64,
        system: S,
    ) {
        self.systems.push(SystemState {
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
            if !s.running && s.name == name {
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
            if s.running && s.name == name {
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
                    s.system
                        .update(if s.timing > 0.0 { s.timing } else { delta });
                }
                s.since_update = 0.0;
            }
        }
    }

    pub fn post_message(&mut self, system: &str, msg: &T) {
        // Temp solution, should queue first then send
        for s in &mut self.systems {
            if s.running && s.name == system {
                println!("System {} receiving message", s.name);
                s.system.message(msg);
            }
        }
    }

    pub fn emit_message(&mut self, msg: &T) {
        // Temp solution, should queue first then send
        for s in &mut self.systems {
            println!("System {} receiving message", s.name);
            s.system.message(msg);
        }
    }
}

impl<T> Drop for Kronos<T> {
    fn drop(&mut self) {
        for s in &mut self.systems {
            if s.running {
                s.system.stop();
            }
        }
    }
}
