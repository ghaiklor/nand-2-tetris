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
                OpCode::Add => self.emit_add(),
                OpCode::Sub => self.emit_sub(),
                OpCode::Neg => self.emit_neg(),
                OpCode::Eq => self.emit_eq(),
                OpCode::Gt => self.emit_gt(),
                OpCode::Lt => self.emit_lt(),
                OpCode::And => self.emit_and(),
                OpCode::Or => self.emit_or(),
                OpCode::Not => self.emit_not(),
                OpCode::Push(opcode) => self.emit_push(opcode),
                OpCode::Pop(opcode) => self.emit_pop(opcode),
            };
        }

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

    fn emit_d_to_register(&mut self, register: &str) {
        self.emit(&format!("@{}", register));
        self.emit("M=D");
    }

    fn emit_constant_to_d(&mut self, constant: u16) {
        self.emit(&format!("@{}", constant));
        self.emit("D=A");
    }

    fn emit_1_args_computation(&mut self, computation: &str) {
        self.emit_sp_dec();
        self.emit_stack_to_d();
        self.emit(&format!("D={}", computation));
        self.emit_d_to_stack();
        self.emit_sp_inc();
    }

    fn emit_2_args_computation(&mut self, computation: &str) {
        self.emit_sp_dec();
        self.emit_stack_to_d();
        self.emit_sp_dec();
        self.emit("@SP");
        self.emit("A=M");
        self.emit(&format!("D={}", computation));
        self.emit_d_to_stack();
        self.emit_sp_inc();
    }

    fn emit_add(&mut self) {
        self.emit_comment("add");
        self.emit_2_args_computation("M+D");
    }

    fn emit_sub(&mut self) {
        self.emit_comment("sub");
        self.emit_2_args_computation("M-D");
    }

    fn emit_neg(&mut self) {
        self.emit_comment("neg");
        self.emit_1_args_computation("-D");
    }

    fn emit_eq(&mut self) {
        self.emit_comment("eq");
        self.emit_sp_dec();
        self.emit_stack_to_d();
        self.emit_sp_dec();

        // D = D - &SP
        self.emit("@SP");
        self.emit("A=M");
        self.emit("D=M-D");

        // JEQ M-D
        self.emit(&format!("@__EQ_{}_{}", self.filename, self.label_counter));
        self.emit("D;JEQ");
        self.emit("@SP");
        self.emit("A=M");
        self.emit("M=0");
        self.emit(&format!(
            "@__END_EQ_{}_{}",
            self.filename, self.label_counter
        ));
        self.emit("0;JMP");
        self.emit(&format!("(__EQ_{}_{})", self.filename, self.label_counter));
        self.emit("@SP");
        self.emit("A=M");
        self.emit("M=-1");
        self.emit(&format!(
            "(__END_EQ_{}_{})",
            self.filename, self.label_counter
        ));

        self.emit_sp_inc();
        self.label_counter += 1;
    }

    fn emit_gt(&mut self) {
        self.emit_comment("gt");
        self.emit_sp_dec();
        self.emit_stack_to_d();
        self.emit_sp_dec();

        // D = D - &SP
        self.emit("@SP");
        self.emit("A=M");
        self.emit("D=M-D");

        // JGT M-D
        self.emit(&format!("@__GT_{}_{}", self.filename, self.label_counter));
        self.emit("D;JGT");
        self.emit("@SP");
        self.emit("A=M");
        self.emit("M=0");
        self.emit(&format!(
            "@__END_GT_{}_{}",
            self.filename, self.label_counter
        ));
        self.emit("0;JMP");
        self.emit(&format!("(__GT_{}_{})", self.filename, self.label_counter));
        self.emit("@SP");
        self.emit("A=M");
        self.emit("M=-1");
        self.emit(&format!(
            "(__END_GT_{}_{})",
            self.filename, self.label_counter
        ));

        self.emit_sp_inc();
        self.label_counter += 1;
    }

    fn emit_lt(&mut self) {
        self.emit_comment("lt");
        self.emit_sp_dec();
        self.emit_stack_to_d();
        self.emit_sp_dec();

        // D = D - &SP
        self.emit("@SP");
        self.emit("A=M");
        self.emit("D=M-D");

        // JLT M-D
        self.emit(&format!("@__LT_{}_{}", self.filename, self.label_counter));
        self.emit("D;JLT");
        self.emit("@SP");
        self.emit("A=M");
        self.emit("M=0");
        self.emit(&format!(
            "@__END_LT_{}_{}",
            self.filename, self.label_counter
        ));
        self.emit("0;JMP");
        self.emit(&format!("(__LT_{}_{})", self.filename, self.label_counter));
        self.emit("@SP");
        self.emit("A=M");
        self.emit("M=-1");
        self.emit(&format!(
            "(__END_LT_{}_{})",
            self.filename, self.label_counter
        ));

        self.emit_sp_inc();
        self.label_counter += 1;
    }

    fn emit_and(&mut self) {
        self.emit_comment("and");
        self.emit_2_args_computation("M&D");
    }

    fn emit_or(&mut self) {
        self.emit_comment("or");
        self.emit_2_args_computation("M|D");
    }

    fn emit_not(&mut self) {
        self.emit_comment("not");
        self.emit_1_args_computation("!D");
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

                // D = i
                self.emit(&format!("@{}", opcode.i));
                self.emit("D=A");

                // D = &(@segment + i)
                self.emit(&format!("@{}", segment));
                self.emit("A=M+D");
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
                // D = i
                self.emit(&format!("@{}", opcode.i));
                self.emit("D=A");

                // D = &(i + 5)
                self.emit(&format!("@{}", 5));
                self.emit("A=D+A");
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

                // D = THIS/THAT
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

                self.emit_sp_dec();

                // R13 = @segment + i
                self.emit(&format!("@{}", segment));
                self.emit("D=M");
                self.emit(&format!("@{}", opcode.i));
                self.emit("D=D+A");
                self.emit_d_to_register("R13");
                self.emit_stack_to_d();

                // &R13 = D
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
                self.emit_sp_dec();

                // R13 = 5 + i
                self.emit(&format!("@{}", 5));
                self.emit("D=A");
                self.emit(&format!("@{}", opcode.i));
                self.emit("D=D+A");
                self.emit_d_to_register("R13");
                self.emit_stack_to_d();

                // &R13 = D
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

                // THIS/THAT = D
                self.emit(&format!("@{}", offset));
                self.emit("M=D");
            }
            _ => panic!("Unknown segment name: {}", opcode.segment),
        }
    }
}
