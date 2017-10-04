use super::context;

pub struct Shader {
}

impl Shader {
    pub fn create() -> Self {
        Shader{}
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
    }
}
