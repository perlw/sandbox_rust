use super::context;

pub struct Shader {
    pub handle: context::Handle,
}

impl Shader {}

impl Drop for Shader {
    fn drop(&mut self) {}
}
