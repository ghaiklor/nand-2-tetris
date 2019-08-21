use crate::opcode::*;

pub fn codegen(opcodes: &[OpCode]) -> String {
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
            OpCode::Push(opcode) => emit_push(opcode, &mut assembly_code),
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
    emit(&format!("// {}", msg), out);
}

fn emit_add(out: &mut String) {
    emit_comment("add", out);
}

fn emit_sub(out: &mut String) {
    emit_comment("sub", out);
}

fn emit_neg(out: &mut String) {
    emit_comment("neg", out);
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
}

fn emit_or(out: &mut String) {
    emit_comment("or", out);
}

fn emit_not(out: &mut String) {
    emit_comment("not", out);
}

fn emit_push(opcode: &PushOpCode, out: &mut String) {
    emit_comment(&format!("push {} {}", opcode.segment, opcode.i), out);
}

fn emit_pop(opcode: &PopOpCode, out: &mut String) {
    emit_comment(&format!("pop {} {}", opcode.segment, opcode.i), out);
}
