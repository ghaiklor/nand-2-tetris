#[derive(Default)]
pub struct Codegen {
    pub vm_code: String,
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            vm_code: String::new(),
        }
    }
}
