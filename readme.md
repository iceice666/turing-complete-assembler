# Turing Complete Assembler

An assembler for my machine which designed in the [Turing Complete Game].

## Prerequisites

1. Ensure the **[Turing Complete Game]** is installed.
2. Launch the game, open the *Schematic Hub*, search for `SMIPS` by creator `KSHSlime`, and load the schematic.

## Installation

0. **[Install Rust]** if not already installed.
1. **Clone the Repository:**
   ```bash
   git clone https://github.com/iceice666/turing-complete-assembler.git
   ```
2. **Build the Assembler:**
   ```bash
   cargo build --release
   ```
   The executable will be in `target/release/assembler`.

## Usage

### Basic Command
```bash
assembler <asm_file>
```
This will generate a binary (`.bin`) file in the same directory with the same name as `<asm_file>`.

### Options and Help
For more options, use:
```bash
assembler -h
```

## Language Syntax

- **Registers:** `<rs>`, `<rt>`, `<rd>` are registers from `$0` to `$31`.
- **Immediate Values:** `<imm>` is a 16-bit immediate; `<shamt>` is a 5-bit shift amount.
- **Comments:** Start with `#`. Inline comments after instructions are ignored. Other comments will be copied to the output.

### Pseudo Instructions
Pseudo instructions start with `!` and case-insensitive.
All pseudo instructions will be copied to the output as comments.

- `!alias <name> <register / imm value>`: Defines an alias.
- `!label <name>`: Defines a label.

### Instruction Set

All instructions are case-insensitive.

#### R-type Instructions
| Instruction | Syntax                  | Description              |
| ----------- | ----------------------- | ------------------------ |
| `add`       | `add <rd> <rs> <rt>`    | `rd = rs + rt`           |
| `sub`       | `sub <rd> <rs> <rt>`    | `rd = rs - rt`           |
| `and`       | `and <rd> <rs> <rt>`    | `rd = rs & rt`           |
| `or`        | `or <rd> <rs> <rt>`     | `rd = rs \| rt`          |
| `xor`       | `xor <rd> <rs> <rt>`    | `rd = rs ^ rt`           |
| `slt`       | `slt <rd> <rs> <rt>`    | `rd = (rs < rt) ? 1 : 0` |
| `sll`       | `sll <rd> <rt> <shamt>` | `rd = rt << shamt`       |
| `srl`       | `srl <rd> <rt> <shamt>` | `rd = rt >>> shamt`      |
| `sra`       | `sra <rd> <rt> <shamt>` | `rd = rt >> shamt`       |
| `jr`        | `jr <rs>`               | `PC = rs`                |

#### I-type Instructions
| Instruction | Syntax                 | Description                            |
| ----------- | ---------------------- | -------------------------------------- |
| `addi`      | `addi <rt> <rs> <imm>` | `rt = rs + imm`                        |
| `subi`      | `subi <rt> <rs> <imm>` | `rt = rs - imm`                        |
| `andi`      | `andi <rt> <rs> <imm>` | `rt = rs & imm`                        |
| `ori`       | `ori <rt> <rs> <imm>`  | `rt = rs \| imm`                       |
| `xori`      | `xori <rt> <rs> <imm>` | `rt = rs ^ imm`                        |
| `slti`      | `slti <rt> <rs> <imm>` | `rt = (rs < imm) ? 1 : 0`              |
| `lw`        | `lw <rt> <rs> <imm>`   | `rt = Memory[rs + imm]`                |
| `sw`        | `sw <rt> <rs> <imm>`   | `Memory[rs + imm] = rt`                |
| `li`        | `li <rt>`              | Load input to `rt`.                    |
| `so`        | `so <rt>`              | Store `rt` to output.                  |
| `pop`       | `pop <rt>`             | Set `rt` to stack popped value.        |
| `push`      | `push <rt>`            | Push `rt` in stack.                    |
| `beq`       | `beq <rs> <rt> <imm>`  | `if (rs == rt) PC = PC + 4 + imm << 2` |

#### Special Instructions
These are aliases for commonly used instruction sequences.

| Instruction | Syntax          | Translated Instructions |
| ----------- | --------------- | ----------------------- |
| `GOTO`      | `goto <imm>`    | `beq 0 0 <imm>`         |
| `MOV`       | `mov <rd> <rs>` | `addi <rd> <rs> 0`      |
| `INC`       | `inc <rd>`      | `addi <rd> <rd> 1`      |

### Example

There's some solution for the game in the `solution` directory.




[Turing Complete Game]: https://store.steampowered.com/app/1444480/Turing_Complete/
[Install Rust]: https://www.rust-lang.org/

