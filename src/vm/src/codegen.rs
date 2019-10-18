use crate::opcode::*;
use std::path::Path;

pub struct Codegen {
    filename: String,
    assembly: String,
    label_counter: usize,
}

impl Codegen {
    pub fn new(input_file: &str) -> Codegen {
        Codegen {
            filename: String::from(Path::new(input_file).file_name().unwrap().to_str().unwrap()),
            assembly: String::new(),
            label_counter: 0,
        }
    }

    pub fn codegen(&mut self, opcodes: &[OpCode]) -> String {
        for opcode in opcodes {
            match opcode {
                OpCode::Add => self.emit_2_args_computation("D+M", "add"),
                OpCode::Sub => self.emit_2_args_computation("M-D", "sub"),
                OpCode::Neg => self.emit_1_args_computation("-M", "neg"),
                OpCode::Eq => self.emit_comparable_computation("JEQ", "eq"),
                OpCode::Gt => self.emit_comparable_computation("JGT", "gt"),
                OpCode::Lt => self.emit_comparable_computation("JLT", "lt"),
                OpCode::And => self.emit_2_args_computation("D&M", "and"),
                OpCode::Or => self.emit_2_args_computation("D|M", "or"),
                OpCode::Not => self.emit_1_args_computation("!M", "not"),
                OpCode::Return => self.emit_return(),
                OpCode::Push(opcode) => self.emit_push(opcode),
                OpCode::Pop(opcode) => self.emit_pop(opcode),
                OpCode::Label(opcode) => self.emit_label(opcode.id),
                OpCode::Goto(opcode) => self.emit_goto(opcode.id),
                OpCode::IfGoto(opcode) => self.emit_if_goto(opcode.id),
                OpCode::Function(opcode) => self.emit_function(opcode),
                OpCode::Call(opcode) => self.emit_call(opcode),
            };
        }

        String::from(&self.assembly)
    }

    pub fn emit_entry(&mut self) -> String {
        self.emit_constant_to_d(256);
        self.emit_d_to_address("SP");
        self.emit_call(&CallOpCode {
            id: "Sys.init",
            args_count: 0,
        });

        String::from(&self.assembly)
    }

    fn emit(&mut self, instruction: &str) {
        self.assembly.push_str(instruction);
        self.assembly.push('\n');
    }

    fn emit_comment(&mut self, msg: &str) {
        self.emit(&format!("\n// {}", msg));
    }

    fn emit_sp_dec(&mut self) {
        self.emit("@SP");
        self.emit("M=M-1");
    }

    fn emit_sp_inc(&mut self) {
        self.emit("@SP");
        self.emit("M=M+1");
    }

    fn emit_stack_to_d(&mut self) {
        self.emit("@SP");
        self.emit("A=M");
        self.emit("D=M");
    }

    fn emit_d_to_stack(&mut self) {
        self.emit("@SP");
        self.emit("A=M");
        self.emit("M=D");
    }

    fn emit_d_to_address(&mut self, address: &str) {
        self.emit(&format!("@{}", address));
        self.emit("M=D");
    }

    fn emit_address_to_d(&mut self, address: &str) {
        self.emit(&format!("@{}", address));
        self.emit("D=M");
    }

    fn emit_constant_to_d(&mut self, constant: u16) {
        self.emit(&format!("@{}", constant));
        self.emit("D=A");
    }

    fn emit_label(&mut self, id: &str) {
        self.emit_comment(&format!("label {}", id));
        self.emit(&format!("({})", id));
    }

    fn emit_goto(&mut self, id: &str) {
        self.emit_comment(&format!("goto {}", id));
        self.emit(&format!("@{}", id));
        self.emit("0;JMP");
    }

    fn emit_if_goto(&mut self, id: &str) {
        self.emit_comment(&format!("if-goto {}", id));
        self.emit_sp_dec();
        self.emit_stack_to_d();
        self.emit(&format!("@{}", id));
        self.emit("D;JNE");
    }

    fn emit_1_args_computation(&mut self, computation: &str, comment: &str) {
        self.emit_comment(comment);
        self.emit("@SP");
        self.emit("A=M-1");
        self.emit(&format!("M={}", computation));
    }

    fn emit_2_args_computation(&mut self, computation: &str, comment: &str) {
        self.emit_comment(comment);
        self.emit("@SP");
        self.emit("AM=M-1");
        self.emit("D=M");
        self.emit("A=A-1");
        self.emit(&format!("M={}", computation));
    }

    fn emit_comparable_computation(&mut self, comparator: &str, comment: &str) {
        self.emit_2_args_computation("M-D", comment);
        self.emit("D=M");

        self.emit(&format!(
            "@__{}_{}_{}",
            comparator, self.filename, self.label_counter
        ));
        self.emit(&format!("D;{}", comparator));
        self.emit("@SP");
        self.emit("A=M-1");
        self.emit("M=0");
        self.emit(&format!(
            "@__END_{}_{}_{}",
            comparator, self.filename, self.label_counter
        ));
        self.emit("0;JMP");
        self.emit(&format!(
            "(__{}_{}_{})",
            comparator, self.filename, self.label_counter
        ));
        self.emit("@SP");
        self.emit("A=M-1");
        self.emit("M=-1");
        self.emit(&format!(
            "(__END_{}_{}_{})",
            comparator, self.filename, self.label_counter
        ));

        self.label_counter += 1;
    }

    fn emit_push(&mut self, opcode: &PushOpCode) {
        self.emit_comment(&format!("push {} {}", opcode.segment, opcode.i));

        match opcode.segment {
            "local" | "argument" | "this" | "that" => {
                let segment = match opcode.segment {
                    "local" => "LCL",
                    "argument" => "ARG",
                    "this" => "THIS",
                    "that" => "THAT",
                    _ => panic!("Unsupported segment name: {}", opcode.segment),
                };

                // D = &(@segment + i)
                self.emit(&format!("@{}", segment));
                self.emit("D=M");
                self.emit(&format!("@{}", opcode.i));
                self.emit("A=D+A");
                self.emit("D=M");
                self.emit_d_to_stack();
                self.emit_sp_inc();
            }
            "constant" => {
                self.emit_constant_to_d(opcode.i);
                self.emit_d_to_stack();
                self.emit_sp_inc();
            }
            "static" => {
                // D = @<filename>.<i>
                self.emit(&format!("@{}.{}", &self.filename, opcode.i));
                self.emit("D=M");
                self.emit_d_to_stack();
                self.emit_sp_inc();
            }
            "temp" => {
                // D = &(i + 5)
                self.emit(&format!("@{}", opcode.i + 5));
                self.emit("D=M");
                self.emit_d_to_stack();
                self.emit_sp_inc();
            }
            "pointer" => {
                let offset = match opcode.i {
                    0 => "THIS",
                    1 => "THAT",
                    _ => panic!("Unknown pointer offset: {}", opcode.i),
                };

                self.emit(&format!("@{}", offset));
                self.emit("D=M");
                self.emit_d_to_stack();
                self.emit_sp_inc();
            }
            _ => panic!("Unknown segment name: {}", opcode.segment),
        }
    }

    fn emit_pop(&mut self, opcode: &PopOpCode) {
        self.emit_comment(&format!("pop {} {}", opcode.segment, opcode.i));

        match opcode.segment {
            "local" | "argument" | "this" | "that" => {
                let segment = match opcode.segment {
                    "local" => "LCL",
                    "argument" => "ARG",
                    "this" => "THIS",
                    "that" => "THAT",
                    _ => panic!("Unsupported segment name: {}", opcode.segment),
                };

                // R13 = @segment + i
                self.emit_address_to_d(segment);
                self.emit(&format!("@{}", opcode.i));
                self.emit("D=D+A");
                self.emit_d_to_address("R13");

                // &R13 = D
                self.emit_sp_dec();
                self.emit_stack_to_d();
                self.emit("@R13");
                self.emit("A=M");
                self.emit("M=D");
            }
            "constant" => {
                panic!("pop constant is not supported");
            }
            "static" => {
                self.emit_sp_dec();
                self.emit_stack_to_d();

                // @<filename>.<i> = D
                self.emit(&format!("@{}.{}", &self.filename, opcode.i));
                self.emit("M=D");
            }
            "temp" => {
                // R13 = 5 + i
                self.emit_constant_to_d(5);
                self.emit(&format!("@{}", opcode.i));
                self.emit("D=D+A");
                self.emit_d_to_address("R13");

                // &R13 = D
                self.emit_sp_dec();
                self.emit_stack_to_d();
                self.emit("@R13");
                self.emit("A=M");
                self.emit("M=D");
            }
            "pointer" => {
                let offset = match opcode.i {
                    0 => "THIS",
                    1 => "THAT",
                    _ => panic!("Unknown pointer offset: {}", opcode.i),
                };

                self.emit_sp_dec();
                self.emit_stack_to_d();
                self.emit(&format!("@{}", offset));
                self.emit("M=D");
            }
            _ => panic!("Unknown segment name: {}", opcode.segment),
        }
    }

    fn emit_function(&mut self, opcode: &FunctionOpCode) {
        self.emit_comment(&format!("function {} {}", opcode.id, opcode.vars_count));
        self.emit_label(&format!("__CALL__{}__", opcode.id));

        let mut vars_count = opcode.vars_count;
        while vars_count > 0 {
            self.emit_constant_to_d(0);
            self.emit_d_to_stack();
            self.emit_sp_inc();

            vars_count -= 1;
        }
    }

    fn emit_call(&mut self, opcode: &CallOpCode) {
        self.emit_comment(&format!("call {} {}", opcode.id, opcode.args_count));

        // store return address
        self.emit(&format!(
            "@__CALL__{}__{}__RET",
            opcode.id, self.label_counter
        ));
        self.emit("D=A");
        self.emit_d_to_stack();
        self.emit_sp_inc();

        // store call frame
        self.emit_address_to_d("LCL");
        self.emit_d_to_stack();
        self.emit_sp_inc();
        self.emit_address_to_d("ARG");
        self.emit_d_to_stack();
        self.emit_sp_inc();
        self.emit_address_to_d("THIS");
        self.emit_d_to_stack();
        self.emit_sp_inc();
        self.emit_address_to_d("THAT");
        self.emit_d_to_stack();
        self.emit_sp_inc();

        // calculate ARG
        self.emit_address_to_d("SP");
        self.emit("@5");
        self.emit("D=D-A");
        self.emit(&format!("@{}", opcode.args_count));
        self.emit("D=D-A");
        self.emit_d_to_address("ARG");

        // reposition LCL
        self.emit_address_to_d("SP");
        self.emit_d_to_address("LCL");

        // finally, jump to the function
        self.emit(&format!("@__CALL__{}__", opcode.id));
        self.emit("0;JMP");

        self.emit_label(&format!(
            "__CALL__{}__{}__RET",
            opcode.id, self.label_counter
        ));

        self.label_counter += 1;
    }

    fn emit_return(&mut self) {
        self.emit_comment("return");

        // store the address of end frame and return address
        self.emit_address_to_d("LCL");
        self.emit_d_to_address("R13");
        self.emit("@5");
        self.emit("A=D-A");
        self.emit("D=M");
        self.emit_d_to_address("R14");

        // pop the return value to start frame
        self.emit_sp_dec();
        self.emit_stack_to_d();
        self.emit("@ARG");
        self.emit("A=M");
        self.emit("M=D");

        // reposition SP
        self.emit("@ARG");
        self.emit("D=M+1");
        self.emit_d_to_address("SP");

        // restore the call frame
        self.emit_address_to_d("R13");
        self.emit("@1");
        self.emit("D=D-A");
        self.emit("A=D");
        self.emit("D=M");
        self.emit("@THAT");
        self.emit("M=D");

        self.emit_address_to_d("R13");
        self.emit("@2");
        self.emit("D=D-A");
        self.emit("A=D");
        self.emit("D=M");
        self.emit("@THIS");
        self.emit("M=D");

        self.emit_address_to_d("R13");
        self.emit("@3");
        self.emit("D=D-A");
        self.emit("A=D");
        self.emit("D=M");
        self.emit("@ARG");
        self.emit("M=D");

        self.emit_address_to_d("R13");
        self.emit("@4");
        self.emit("D=D-A");
        self.emit("A=D");
        self.emit("D=M");
        self.emit("@LCL");
        self.emit("M=D");

        // jump to return address
        self.emit_address_to_d("R14");
        self.emit("A=D");
        self.emit("0;JMP");
    }
}
