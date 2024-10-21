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

fn assemble(input: &str) -> Vec<String> {
    let segments: Vec<&str> = input.split_whitespace().filter(|x| !x.is_empty()).collect();
    println!("{:?}", segments);
    let mut result = Vec::new();

    let mut iter = segments.iter().peekable();

    while let Some(&seg) = iter.next() {
        let seg = seg.to_uppercase();

        let cmd: AsmCmd = if RCmndFunc::VARIANTS.contains(&seg.as_str()) {
            let func = RCmndFunc::from_str(seg.as_str()).unwrap();
            let rs = iter.next().unwrap().parse::<u8>().unwrap();
            let rt = iter.next().unwrap().parse::<u8>().unwrap();
            let rd = iter.next().unwrap().parse::<u8>().unwrap();
            let shamt = iter.next().unwrap().parse::<u8>().unwrap();

            AsmCmd::R(RCmd::new(rs, rt, rd, shamt, func))
        } else if ICmdOp::VARIANTS.contains(&seg.as_str()) {
            let op = ICmdOp::from_str(seg.as_str()).unwrap();
            let rs = iter.next().unwrap().parse::<u8>().unwrap();
            let rt = iter.next().unwrap().parse::<u8>().unwrap();
            let imm = iter.next().unwrap().parse::<u16>().unwrap();

            AsmCmd::I(ICmd::new(rs, rt, imm, op))
        } else if JCmdOp::VARIANTS.contains(&seg.as_str()) {
            let op = JCmdOp::from_str(seg.as_str()).unwrap();
            let addr = iter.next().unwrap().parse::<u32>().unwrap();

            AsmCmd::J(JCmd::new(addr, op))
        } else {
            panic!("Invalid command: {}", seg);
        };

        result.push(cmd.assemble());
    }

    result
}
