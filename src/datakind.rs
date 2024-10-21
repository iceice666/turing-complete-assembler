#![allow(clippy::upper_case_acronyms, dead_code)]

use strum::{EnumString, VariantNames};

pub trait Assemble {
    fn assemble(&self) -> String;
}

pub enum AsmCmd {
    R(RCmd),
    I(ICmd),
    J(JCmd),
}
impl Assemble for AsmCmd {
    fn assemble(&self) -> String {
        match self {
            AsmCmd::R(cmd) => cmd.assemble(),
            AsmCmd::I(cmd) => cmd.assemble(),
            AsmCmd::J(cmd) => cmd.assemble(),
        }
    }
}

#[derive(Copy, Clone, EnumString, VariantNames, PartialEq)]
pub enum RCmndFunc {
    ADD = 0b000001,
    SUB = 0b000010,
    AND = 0b000011,
    OR = 0b000100,
    XOR = 0b000101,
    SLL = 0b000110,
    SRL = 0b000111,
    SRA = 0b001000,
    SLT = 0b001001,
    JR = 0b010000,
}

pub struct RCmd {
    op: u8,          // 6 bits
    rs: u8,          // 5 bits
    rt: u8,          // 5 bits
    rd: u8,          // 5 bits
    shamt: u8,       // 5 bits
    func: RCmndFunc, // 6 bits
}

impl RCmd {
    pub fn new(rs: u8, rt: u8, rd: u8, shamt: u8, func: RCmndFunc) -> Self {
        Self {
            op: 0,
            rs,
            rt,
            rd,
            shamt,
            func,
        }
    }
}

impl Assemble for RCmd {
    fn assemble(&self) -> String {
        // RCmd ignores op field
        let first = self.rs >> 3;
        let second = (self.rs << 5) | self.rt;
        let third = (self.rd << 3) | (self.shamt >> 2);
        let fourth = (self.shamt << 6) | (self.func as u8);

        format!(
            "{:#04X} {:#04X} {:#04X} {:#04X}",
            first, second, third, fourth
        )
    }
}

#[derive(Copy, Clone, EnumString, VariantNames, PartialEq)]
pub enum ICmdOp {
    ADDI = 0b000001,
    SUBI = 0b000010,
    ANDI = 0b000011,
    ORI = 0b000100,
    XORI = 0b000101,
    SLTI = 0b001001,

    LW = 0b010000,
    SW = 0b010001,
    LI = 0b010010,
    SO = 0b010011,
    POP = 0b010100,
    PUSH = 0b010101,

    BEQ = 0b100000,
}

pub struct ICmd {
    op: ICmdOp, // 6 bits
    rs: u8,     // 5 bits
    rt: u8,     // 5 bits
    imm: u16,   // 16 bits
}

impl ICmd {
    pub fn new(rs: u8, rt: u8, imm: u16, op: ICmdOp) -> Self {
        Self { op, rs, rt, imm }
    }
}

impl Assemble for ICmd {
    fn assemble(&self) -> String {
        let first = (self.op as u8) << 2 | (self.rs >> 3);
        let second = (self.rs << 5) | self.rt;
        let third = (self.imm >> 8) as u8;
        let fourth = self.imm as u8;

        format!(
            "{:#04X} {:#04X} {:#04X} {:#04X}",
            first, second, third, fourth
        )
    }
}

#[derive(Copy, Clone, EnumString, VariantNames, PartialEq)]
pub enum JCmdOp {
    J = 0b100001,
}

pub struct JCmd {
    op: JCmdOp, // 6 bits
    addr: u32,  // 26 bits
}

impl JCmd {
    pub fn new(addr: u32, op: JCmdOp) -> Self {
        Self { op, addr }
    }
}

impl Assemble for JCmd {
    fn assemble(&self) -> String {
        let first = (self.op as u8) << 2 | (self.addr >> 24) as u8;
        let second = (self.addr >> 16) as u8;
        let third = (self.addr >> 8) as u8;
        let fourth = self.addr as u8;

        format!(
            "{:#04X} {:#04X} {:#04X} {:#04X}",
            first, second, third, fourth
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_r_cmd_assemble() {
        let cmd = RCmd {
            op: 0,
            rs: 1,
            rt: 2,
            rd: 3,
            shamt: 0,
            func: RCmndFunc::ADD,
        };

        assert_eq!(cmd.assemble(), "0x00 0x22 0x18 0x01");
    }

    #[test]
    fn test_i_cmd_assemble() {
        let cmd = ICmd {
            op: ICmdOp::ADDI,
            rs: 1,
            rt: 2,
            imm: 11,
        };

        assert_eq!(cmd.assemble(), "0x04 0x22 0x00 0x0B");
    }

    #[test]
    fn test_j_cmd_assemble() {
        let cmd = JCmd {
            op: JCmdOp::J,
            addr: 87,
        };

        assert_eq!(cmd.assemble(), "0x10 0x00 0x00 0x57");
    }
}
