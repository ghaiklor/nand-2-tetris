use crate::opcode::*;

pub fn codegen(opcodes: &[OpCode], filename: &str) -> String {
    let mut assembly_code = String::new();

    for opcode in opcodes {
        match opcode {
            OpCode::Add => emit_add(&mut assembly_code),
            OpCode::Sub => emit_sub(&mut assembly_code),
            OpCode::Neg => emit_neg(&mut assembly_code),
            OpCode::Eq => emit_eq(&mut assembly_code),
            OpCode::Gt => emit_gt(&mut assembly_code),
            OpCode::Lt => emit_lt(&mut assembly_code),
            OpCode::And => emit_and(&mut assembly_code),
            OpCode::Or => emit_or(&mut assembly_code),
            OpCode::Not => emit_not(&mut assembly_code),
            OpCode::Push(opcode) => emit_push(opcode, &mut assembly_code, filename),
            OpCode::Pop(opcode) => emit_pop(opcode, &mut assembly_code),
        }
    }

    assembly_code
}

fn emit(instruction: &str, out: &mut String) {
    out.push_str(instruction);
    out.push('\n');
}

fn emit_comment(msg: &str, out: &mut String) {
    emit(&format!("\n// {}", msg), out);
}

fn emit_add(out: &mut String) {
    emit_comment("add", out);

    // SP--
    emit("@SP", out);
    emit("M=M-1", out);

    // pop -> D
    emit("@SP", out);
    emit("A=M", out);
    emit("D=M", out);

    // SP--
    emit("@SP", out);
    emit("M=M-1", out);

    // A = SP
    emit("@SP", out);
    emit("A=M", out);

    // x + y -> D
    emit("D=M+D", out);

    // D -> push
    emit("@SP", out);
    emit("A=M", out);
    emit("M=D", out);

    // SP++
    emit("@SP", out);
    emit("M=M+1", out);
}

fn emit_sub(out: &mut String) {
    emit_comment("sub", out);

    // SP--
    emit("@SP", out);
    emit("M=M-1", out);

    // pop -> D
    emit("@SP", out);
    emit("A=M", out);
    emit("D=M", out);

    // SP--
    emit("@SP", out);
    emit("M=M-1", out);

    // A = SP
    emit("@SP", out);
    emit("A=M", out);

    // x - y -> D
    emit("D=M-D", out);

    // D -> push
    emit("@SP", out);
    emit("A=M", out);
    emit("M=D", out);

    // SP++
    emit("@SP", out);
    emit("M=M+1", out);
}

fn emit_neg(out: &mut String) {
    emit_comment("neg", out);

    // SP--
    emit("@SP", out);
    emit("M=M-1", out);

    // pop -> D
    emit("@SP", out);
    emit("A=M", out);
    emit("D=M", out);

    // -x -> D
    emit("D=-D", out);

    // D -> push
    emit("@SP", out);
    emit("A=M", out);
    emit("M=D", out);

    // SP++
    emit("@SP", out);
    emit("M=M+1", out);
}

fn emit_eq(out: &mut String) {
    emit_comment("eq", out);
}

fn emit_gt(out: &mut String) {
    emit_comment("gt", out);
}

fn emit_lt(out: &mut String) {
    emit_comment("lt", out);
}

fn emit_and(out: &mut String) {
    emit_comment("and", out);

    // SP--
    emit("@SP", out);
    emit("M=M-1", out);

    // pop -> D
    emit("@SP", out);
    emit("A=M", out);
    emit("D=M", out);

    // SP--
    emit("@SP", out);
    emit("M=M-1", out);

    // A = SP
    emit("@SP", out);
    emit("A=M", out);

    // x & y -> D
    emit("D=M&D", out);

    // D -> push
    emit("@SP", out);
    emit("A=M", out);
    emit("M=D", out);

    // SP++
    emit("@SP", out);
    emit("M=M+1", out);
}

fn emit_or(out: &mut String) {
    emit_comment("or", out);

    // SP--
    emit("@SP", out);
    emit("M=M-1", out);

    // pop -> D
    emit("@SP", out);
    emit("A=M", out);
    emit("D=M", out);

    // SP--
    emit("@SP", out);
    emit("M=M-1", out);

    // A = SP
    emit("@SP", out);
    emit("A=M", out);

    // x | y -> D
    emit("D=M|D", out);

    // D -> push
    emit("@SP", out);
    emit("A=M", out);
    emit("M=D", out);

    // SP++
    emit("@SP", out);
    emit("M=M+1", out);
}

fn emit_not(out: &mut String) {
    emit_comment("not", out);

    // SP--
    emit("@SP", out);
    emit("M=M-1", out);

    // pop -> D
    emit("@SP", out);
    emit("A=M", out);
    emit("D=M", out);

    // !x -> D
    emit("D=!D", out);

    // D -> push
    emit("@SP", out);
    emit("A=M", out);
    emit("M=D", out);

    // SP++
    emit("@SP", out);
    emit("M=M+1", out);
}

fn emit_push(opcode: &PushOpCode, out: &mut String, filename: &str) {
    emit_comment(&format!("push {} {}", opcode.segment, opcode.i), out);

    match opcode.segment {
        "local" | "argument" | "this" | "that" => {
            // D = i
            emit(&format!("A={}", opcode.i), out);
            emit("D=A", out);

            let segment = match opcode.segment {
                "local" => "LCL",
                "argument" => "ARG",
                "this" => "THIS",
                "that" => "THAT",
                _ => panic!("Unsupported segment name: {}", opcode.segment),
            };

            // D = &(@segment + i)
            emit(&format!("@{}", segment), out);
            emit("A=M+D", out);
            emit("D=M", out);

            // D -> push
            emit("@SP", out);
            emit("A=M", out);
            emit("M=D", out);

            // SP++
            emit("@SP", out);
            emit("M=M+1", out);
        }
        "constant" => {
            // D = i
            emit(&format!("@{}", opcode.i), out);
            emit("D=A", out);

            // D -> push
            emit("@SP", out);
            emit("A=M", out);
            emit("M=D", out);

            // SP++
            emit("@SP", out);
            emit("M=M+1", out);
        }
        "static" => {
            // D = @<filename>.<i>
            emit(&format!("@{}.{}", filename, opcode.i), out);
            emit("D=M", out);

            // D -> push
            emit("@SP", out);
            emit("A=M", out);
            emit("M=D", out);

            // SP++
            emit("@SP", out);
            emit("M=M+1", out);
        }
        "temp" => {
            // D = i
            emit(&format!("A={}", opcode.i), out);
            emit("D=A", out);

            // D = &(i + 5)
            emit("A=5", out);
            emit("A=D+A", out);
            emit("D=M", out);

            // D -> push
            emit("@SP", out);
            emit("A=M", out);
            emit("M=D", out);

            // SP++
            emit("@SP", out);
            emit("M=M+1", out);
        }
        "pointer" => {
            let offset = match opcode.i {
                0 => "THIS",
                1 => "THAT",
                _ => panic!("Unknown pointer offset: {}", opcode.i),
            };

            // D = THIS/THAT
            emit(&format!("@{}", offset), out);
            emit("D=M", out);

            // D -> push
            emit("@SP", out);
            emit("A=M", out);
            emit("M=D", out);

            // SP++
            emit("@SP", out);
            emit("M=M+1", out);
        }
        _ => panic!("Unknown segment name: {}", opcode.segment),
    }
}

fn emit_pop(opcode: &PopOpCode, out: &mut String) {
    // TODO: implement
    emit_comment(&format!("pop {} {}", opcode.segment, opcode.i), out);
}
