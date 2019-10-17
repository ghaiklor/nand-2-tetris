pub enum VMSegment {
    Argument,
    Constant,
    Local,
    Pointer,
    Static,
    Temp,
    That,
    This,
}

pub enum VMArithmetic {
    Add,
    And,
    Eq,
    Gt,
    Lt,
    Neg,
    Not,
    Or,
    Sub,
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
        self.vm_code.push_str(&format!("{}\n", command));
    }

    pub fn emit_label(&mut self, label: &str) {
        self.vm_code.push_str(&format!("label {}\n", label));
    }

    pub fn emit_goto(&mut self, label: &str) {
        self.vm_code.push_str(&format!("goto {}\n", label));
    }

    pub fn emit_if_goto(&mut self, label: &str) {
        self.vm_code.push_str(&format!("if-goto {}\n", label));
    }

    pub fn emit_call(&mut self, name: &str, args_count: u16) {
        self.vm_code
            .push_str(&format!("call {} {}\n", name, args_count));
    }

    pub fn emit_function(&mut self, name: &str, locals_count: u16) {
        self.vm_code
            .push_str(&format!("function {} {}\n", name, locals_count));
    }

    pub fn emit_return(&mut self) {
        self.vm_code.push_str("return\n");
    }

    fn segment_to_str(segment: &VMSegment) -> &str {
        match segment {
            VMSegment::Argument => "argument",
            VMSegment::Constant => "constant",
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
