use std::collections::HashMap;
use std::str::FromStr;

use datakind::{AsmCmd, Assemble, ICmd, ICmdOp, JCmd, JCmdOp, RCmd, RCmndFunc};
use strum::VariantNames;

pub mod datakind;

fn main() -> Result<(), String> {
    const INPUT_FILENAME: &str = "input.asm";
    const OUTPUT_FILENAME: &str = "output.asm";

    // Use more descriptive error messages
    let content = std::fs::read_to_string(INPUT_FILENAME).expect("Failed to read the input file");
    let result = parse(content)?;

    std::fs::write(OUTPUT_FILENAME, result.join("\n")).expect("Failed to write to the output file");

    Ok(())
}

fn parse(input: String) -> Result<Vec<String>, String> {
    let symbols = parse_symbols(&input)?;
    let result = assemble(&input, symbols)?;

    Ok(result)
}

fn parse_symbols(input: &str) -> Result<HashMap<String, u32>, String> {
    let mut symbols: HashMap<String, u32> = HashMap::new();

    input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !(x.is_empty() || x.starts_with("#")))
        .try_fold(0, |acc, line| {
            let segments: Vec<&str> = line.split_whitespace().collect();
            let operation = segments[0].to_uppercase();

            if operation == "LABEL" {
                let label = segments[1];
                if label.ends_with(":") || segments[2] == ":" {
                    symbols.insert(label.trim_end_matches(":").to_string(), acc);
                } else {
                    return Err("Require a ':' after an label symbol".to_string());
                }
            }

            Ok(acc + 1)
        })?;

    Ok(symbols)
}

fn string_to_register(input: &str) -> Result<u8, String> {
    if !input.starts_with("r") {
        return Err(format!("Expected a register, but found '{}'", input));
    }

    let reg = input[1..]
        .parse::<u8>()
        .map_err(|_| format!("Invalid register format: '{}'", input))?;

    if reg > 31 {
        return Err(format!("Invalid register number: '{}'", input));
    }

    Ok(reg)
}

fn parse_shift_operation(operation: &str, segments: &[&str]) -> Result<AsmCmd, String> {
    let func = RCmndFunc::from_str(operation)
        .map_err(|_| format!("Invalid shift operation: '{}'", operation))?;
    let rd = string_to_register(segments[1])?;
    let rt = string_to_register(segments[2])?;
    let shamt = segments[3]
        .parse::<u8>()
        .map_err(|_| format!("Invalid shift amount: '{}'", segments[3]))?;

    Ok(AsmCmd::R(RCmd::new(0, rt, rd, shamt, func)))
}

fn assemble(input: &str, symbols: HashMap<String, u32>) -> Result<Vec<String>, String> {
    let mut res = Vec::new();
    let mut lineno: u32 = 0;

    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            // Directly push empty or comment lines to the result
            res.push(line.to_string());
            continue;
        }

        lineno += 1;
        let segments: Vec<&str> = line.split_whitespace().collect();
        let operation = segments[0].to_uppercase();

        let cmd_result = match operation.as_str() {
            "GOTO" => {
                let op = JCmdOp::J;
                let label = segments[1];
                let target_line = symbols
                    .get(label)
                    .unwrap_or_else(|| panic!("Undefined symbol '{}'", label))
                    .to_owned();

                Ok(AsmCmd::J(JCmd::new(target_line * 4, op)))
            }
            "PUSH" | "POP" => {
                let op = ICmdOp::from_str(&operation).unwrap();
                let rt = string_to_register(segments[1])?;
                Ok(AsmCmd::I(ICmd::new(0, rt, 0, op)))
            }
            "MOV" => {
                let rt = string_to_register(segments[1])?;
                let rs = string_to_register(segments[2])?;
                Ok(AsmCmd::I(ICmd::new(rs, rt, 0, ICmdOp::ADDI)))
            }
            "SLL" | "SRL" | "SRA" => parse_shift_operation(&operation, &segments),
            _ if RCmndFunc::VARIANTS.contains(&operation.as_str()) => {
                let func = RCmndFunc::from_str(&operation).unwrap();
                let rd = string_to_register(segments[1])?;
                let rs = string_to_register(segments[2])?;
                let rt = string_to_register(segments[3])?;
                Ok(AsmCmd::R(RCmd::new(rs, rt, rd, 0, func)))
            }
            _ if ICmdOp::VARIANTS.contains(&operation.as_str()) => {
                let op = ICmdOp::from_str(&operation).unwrap();
                let rs = string_to_register(segments[1])?;
                let rt = string_to_register(segments[2])?;
                let imm = segments[3]
                    .parse::<u16>()
                    .map_err(|_| format!("Invalid immediate value: '{}'", segments[3]))?;
                Ok(AsmCmd::I(ICmd::new(rs, rt, imm, op)))
            }
            _ if JCmdOp::VARIANTS.contains(&operation.as_str()) => {
                let op = JCmdOp::from_str(&operation).unwrap();
                let addr = segments[1]
                    .parse::<u32>()
                    .map_err(|_| format!("Invalid jump address: '{}'", segments[1]))?;
                Ok(AsmCmd::J(JCmd::new(addr, op)))
            }
            "LABEL" => continue,
            _ => Err(format!("Unknown command: {}", operation)),
        };

        match cmd_result {
            Ok(cmd) => res.push(cmd.assemble()),
            Err(err) => panic!("Error on line {}: {}", lineno, err),
        }
    }

    Ok(res)
}
