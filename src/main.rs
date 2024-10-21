use std::str::FromStr;

use datakind::{AsmCmd, Assemble, ICmd, ICmdOp, JCmd, JCmdOp, RCmd, RCmndFunc};
use strum::VariantNames;

pub mod datakind;

fn main() {
    const INPUT_FILENAME: &str = "input.asm";
    const OUTPUT_FILENAME: &str = "output.txt";
    let content = std::fs::read_to_string(INPUT_FILENAME).expect("Failed to read file");
    let result = assemble(&content);
    std::fs::write(OUTPUT_FILENAME, result.join("\n")).expect("Failed to write file");
}

fn string_to_register(input: &str) -> u8 {
    if !input.starts_with("r") {
        panic!("Expect a register, but found {}", input);
    }
    let reg = input[1..].to_string().parse::<u8>().unwrap();

    if reg > 31 {
        panic!("Invalid register: {}", input);
    }
    reg
}

fn parse_a_command(input: &str) -> AsmCmd {
    let segs = input.split_whitespace().collect::<Vec<&str>>();
    let operation = segs[0].to_uppercase();

    if operation == "PUSH" || operation == "POP" {
        let op = ICmdOp::from_str(operation.as_str()).unwrap();
        let rt = string_to_register(segs[1]);

        AsmCmd::I(ICmd::new(0, rt, 0, op))
    }
    // Regular commands
    else if RCmndFunc::VARIANTS.contains(&operation.as_str()) {
        let func = RCmndFunc::from_str(operation.as_str()).unwrap();
        let rs = string_to_register(segs[1]);
        let rt = string_to_register(segs[2]);
        let rd = string_to_register(segs[3]);
        let shamt = segs[4].parse::<u8>().unwrap();

        AsmCmd::R(RCmd::new(rs, rt, rd, shamt, func))
    }
    // Immediate commands
    else if ICmdOp::VARIANTS.contains(&operation.as_str()) {
        let op = ICmdOp::from_str(operation.as_str()).unwrap();
        let rs = string_to_register(segs[1]);
        let rt = string_to_register(segs[2]);
        let imm = segs[3].parse::<u16>().unwrap();

        AsmCmd::I(ICmd::new(rs, rt, imm, op))
    }
    // Jump commands
    else if JCmdOp::VARIANTS.contains(&operation.as_str()) {
        let op = JCmdOp::from_str(segs[1]).unwrap();
        let addr = segs[2].parse::<u32>().unwrap();

        AsmCmd::J(JCmd::new(addr, op))
    }
    // Unknown command
    else {
        panic!("Unknown command: {}", operation);
    }
}

fn assemble(input: &str) -> Vec<String> {
    let mut res = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        } else if line.starts_with("#") {
            res.push(line.to_string());
            continue;
        }
        let cmd = parse_a_command(line);
        res.push(cmd.assemble());
    }

    res
}
