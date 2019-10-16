pub enum VMSegment {
    Argument,
    Local,
    Static,
    This,
    That,
    Pointer,
    Temp,
}

pub enum VMArithmetic {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

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

    pub fn emit_push(&mut self, segment: &VMSegment, index: u16) {
        let segment = Codegen::segment_to_str(segment);

        self.vm_code
            .push_str(&format!("push {} {}\n", segment, index));
    }

    pub fn emit_pop(&mut self, segment: &VMSegment, index: u16) {
        let segment = Codegen::segment_to_str(segment);

        self.vm_code
            .push_str(&format!("pop {} {}\n", segment, index));
    }

    pub fn emit_arithmetic(&mut self, arithmetic: &VMArithmetic) {
        let command = Codegen::arithmetic_to_str(arithmetic);
        self.vm_code.push_str(&command);
    }

    pub fn emit_label(&mut self, label: &str) {
        self.vm_code.push_str(&format!("label {}", label));
    }

    pub fn emit_goto(&mut self, label: &str) {
        self.vm_code.push_str(&format!("goto {}", label));
    }

    pub fn emit_if_goto(&mut self, label: &str) {
        self.vm_code.push_str(&format!("if-goto {}", label));
    }

    pub fn emit_call(&mut self, name: &str, args_count: u16) {
        self.vm_code
            .push_str(&format!("call {} {}", name, args_count));
    }

    pub fn emit_function(&mut self, name: &str, locals_count: u16) {
        self.vm_code
            .push_str(&format!("function {} {}", name, locals_count));
    }

    pub fn emit_return(&mut self) {
        self.vm_code.push_str("return");
    }

    fn segment_to_str(segment: &VMSegment) -> &str {
        match segment {
            VMSegment::Argument => "argument",
            VMSegment::Local => "local",
            VMSegment::Static => "static",
            VMSegment::This => "this",
            VMSegment::That => "that",
            VMSegment::Pointer => "pointer",
            VMSegment::Temp => "temp",
        }
    }

    fn arithmetic_to_str(arithmetic: &VMArithmetic) -> &str {
        match arithmetic {
            VMArithmetic::Add => "add",
            VMArithmetic::Sub => "sub",
            VMArithmetic::Neg => "neg",
            VMArithmetic::Eq => "eq",
            VMArithmetic::Gt => "gt",
            VMArithmetic::Lt => "lt",
            VMArithmetic::And => "and",
            VMArithmetic::Or => "or",
            VMArithmetic::Not => "not",
        }
    }
}
