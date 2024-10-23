use std::collections::HashMap;
use std::str::FromStr;

use datakind::{AsmCmd, Assemble, ICmd, JCmd, RCmd};

pub mod datakind;

fn main() {
    const INPUT_FILENAME: &str = "input.asm";
    const OUTPUT_FILENAME: &str = "output.bin";

    let content = std::fs::read_to_string(INPUT_FILENAME).expect("Failed to read the input file");

    let result = parse(content);

    std::fs::write(OUTPUT_FILENAME, result.join("\n")).expect("Failed to write to the output file");
}

fn parse(input: String) -> Vec<String> {
    let lines = input.lines().map(str::trim);
    let symbols = parse_symbols(lines.clone()).unwrap();

    println!("{:#?}", symbols);

    assemble(lines, symbols).unwrap()
}

fn parse_symbols<'a>(input: impl Iterator<Item = &'a str>) -> Result<HashMap<String, u32>, String> {
    let mut symbols = HashMap::new();
    let mut lineno = 0;

    for line in input {
        if line.starts_with("!") {
            let segments: Vec<&str> = line.trim_start_matches("!").split_whitespace().collect();
            let operation = segments
                .get(0)
                .ok_or_else(|| format!("Empty symbol line at {}", lineno))?
                .to_uppercase();

            match operation.as_str() {
                "LABEL" => {
                    let label = segments
                        .get(1)
                        .ok_or_else(|| format!("Missing label name at {}", lineno))?
                        .trim_end_matches(":");
                    symbols.insert(label.to_string(), lineno);
                }
                "ALIAS" => {
                    let label = segments
                        .get(1)
                        .ok_or_else(|| format!("Missing alias name at {}", lineno))?
                        .to_string();
                    let value = resolve_symbol(
                        segments
                            .get(2)
                            .ok_or_else(|| format!("Missing value for alias at {}", lineno))?,
                        &symbols,
                    )?;
                    symbols.insert(label, value);
                }
                _ => return Err(format!("Unknown declaration: '{}'", operation)),
            }
        } else if !line.is_empty() && !line.starts_with('#') {
            lineno += 1;
        }
    }

    Ok(symbols)
}

fn resolve_symbol<T: FromStr + From<u8>>(
    value: &str,
    sym: &HashMap<String, u32>,
) -> Result<T, String> {
    // imm
    if let Ok(imm) = value.parse() {
        return Ok(imm);
    }

    // reg
    if value.starts_with("r") {
        let reg_num = value[1..]
            .parse::<u8>()
            .map_err(|_| format!("Invalid register: '{}'", value))?;
        if reg_num > 31 {
            return Err(format!("Invalid register: '{}'", value));
        }

        return Ok(reg_num.into());
    }

    // alias

    let reg = sym
        .get(value)
        .map(|v| v.to_string())
        .ok_or(format!("Expected a register, found '{}'", value))?;

    let reg = reg
        .parse::<T>()
        .map_err(|_| format!("Invalid register: '{}'", value))?;

    Ok(reg)
}

fn assemble<'a>(
    input: impl Iterator<Item = &'a str>,
    symbols: HashMap<String, u32>,
) -> Result<Vec<String>, String> {
    let mut res = Vec::new();
    let mut lineno = 0;

    for line in input {
        if line.starts_with('!') {
            res.push("# ".to_string() + line);
            continue;
        } else if line.is_empty() {
            res.push("".to_string());
            continue;
        } else if line.starts_with('#') {
            res.push(line.to_string());
            continue;
        }

        lineno += 1;

        let segments: Vec<&str> = line.split_whitespace().collect();
        let oper = segments[0].to_uppercase();
        let operation = oper.as_str();

        let cmd_result = match operation {
            "GOTO" => {
                let target_line: u32 = resolve_symbol(segments[1], &symbols)?;
                let offset = target_line.overflowing_sub(lineno).0;
                Ok(AsmCmd::I(ICmd::new(0, 0, offset as u16, 0b100000)))
            }

            "MOV" => {
                let rt = resolve_symbol(segments[1], &symbols)?;
                let rs = resolve_symbol(segments[2], &symbols)?;
                Ok(AsmCmd::I(ICmd::new(rs, rt, 0, 1)))
            }
            "INC" => {
                let rt = resolve_symbol(segments[1], &symbols)?;
                Ok(AsmCmd::I(ICmd::new(rt, rt, 1, 1)))
            }

            // R-type commands
            "SLL" | "SRL" | "SRA" => {
                let func = match operation {
                    "SLL" => 0b000110,
                    "SRL" => 0b000111,
                    "SRA" => 0b001000,
                    _ => unreachable!(),
                };
                let rd = resolve_symbol(segments[1], &symbols)?;
                let rt = resolve_symbol(segments[2], &symbols)?;
                let shamt = resolve_symbol(segments[3], &symbols)
                    .map_err(|_| format!("Invalid shift amount: '{}'", segments[3]));

                shamt.map(|shamt| AsmCmd::R(RCmd::new(0, rt, rd, shamt, func)))
            }

            "ADD" | "SUB" | "AND" | "OR" | "XOR" | "SLT" | "JR" => {
                let func = match operation {
                    "ADD" => 0b000001,
                    "SUB" => 0b000010,
                    "AND" => 0b000011,
                    "OR" => 0b000100,
                    "XOR" => 0b000101,
                    "SLT" => 0b001001,
                    "JR" => 0b010000,
                    _ => unreachable!(),
                };
                let rd = resolve_symbol(segments[1], &symbols)?;
                let rs = resolve_symbol(segments[2], &symbols)?;
                let rt = resolve_symbol(segments[3], &symbols)?;
                Ok(AsmCmd::R(RCmd::new(rs, rt, rd, 0, func)))
            }

            // I-type commands
            "PUSH" | "POP" | "LI" | "SO" => {
                let op = match operation {
                    "LI" => 0b010010,
                    "SO" => 0b010011,
                    "POP" => 0b010100,
                    "PUSH" => 0b010101,
                    _ => unreachable!(),
                };
                let rt = resolve_symbol(segments[1], &symbols)?;
                Ok(AsmCmd::I(ICmd::new(0, rt, 0, op)))
            }
            "LW" | "SW" | "ADDI" | "SUBI" | "ANDI" | "ORI" | "XORI" | "SLTI" => {
                let op = match operation {
                    "ADDI" => 0b000001,
                    "SUBI" => 0b000010,
                    "ANDI" => 0b000011,
                    "ORI" => 0b000100,
                    "XORI" => 0b000101,
                    "SLTI" => 0b001001,
                    "LW" => 0b010000,
                    "SW" => 0b010001,
                    _ => unreachable!(),
                };
                let rt = resolve_symbol(segments[1], &symbols)?;
                let rs = resolve_symbol(segments[2], &symbols)?;
                let imm = resolve_symbol(segments[3], &symbols)
                    .map_err(|_| format!("Invalid immediate value: '{}'", segments[3]));

                imm.map(|imm| AsmCmd::I(ICmd::new(rs, rt, imm, op)))
            }
            "BEQ" => {
                let op = 0b100000;
                let rt = resolve_symbol(segments[1], &symbols)?;
                let rs = resolve_symbol(segments[2], &symbols)?;
                let target_line: u32 = resolve_symbol(segments[3], &symbols)?;
                let offset = target_line.overflowing_sub(lineno).0;
                Ok(AsmCmd::I(ICmd::new(rs, rt, offset as u16, op)))
            }

            // J-type commands
            "J" => {
                let op = 0b100001;
                let addr = resolve_symbol(segments[1], &symbols)
                    .map_err(|_| format!("Invalid jump address: '{}'", segments[1]));

                addr.map(|addr| AsmCmd::J(JCmd::new(addr, op)))
            }

            _ => Err(format!("Unknown command: {}", oper)),
        };

        match cmd_result {
            Ok(cmd) => res.push(format!("{} # {} [ln.{}]", cmd.assemble(), line, lineno)),
            Err(err) => return Err(format!("Error on line {}: {}", lineno, err,)),
        }
    }

    Ok(res)
}
