use crate::opcode::*;

pub fn parse(vm_code: &str) -> Vec<OpCode> {
    let mut opcodes = Vec::new();

    for instruction in vm_code.lines() {
        let instruction = instruction.trim().splitn(2, '/').next().unwrap().trim();
        if instruction.is_empty() {
            continue;
        }

        if instruction.starts_with("push") {
            let parts: Vec<&str> = instruction.splitn(3, ' ').collect();
            let opcode = OpCode::Push(PushOpCode {
                segment: parts[1],
                i: parts[2]
                    .parse()
                    .expect("Expected a number in push segment i"),
            });

            opcodes.push(opcode);
            continue;
        };

        if instruction.starts_with("pop") {
            let parts: Vec<&str> = instruction.splitn(3, ' ').collect();
            let opcode = OpCode::Pop(PopOpCode {
                segment: parts[1],
                i: parts[2]
                    .parse()
                    .expect("Expected a number in pop segment i"),
            });

            opcodes.push(opcode);
            continue;
        };

        let opcode = match instruction {
            "add" => OpCode::Add,
            "sub" => OpCode::Sub,
            "neg" => OpCode::Neg,
            "eq" => OpCode::Eq,
            "gt" => OpCode::Gt,
            "lt" => OpCode::Lt,
            "and" => OpCode::And,
            "or" => OpCode::Or,
            "not" => OpCode::Not,
            &_ => panic!("Unknown opcode {}", instruction),
        };

        opcodes.push(opcode);
    }

    opcodes
}
