# turing complete assembler

An assembler for my designed machine in the [Turing Complete Game].

## Before you start

1. You have the **[Turing Complete Game]** installed.
2. Launch the game.
3. Open menu and click *Schematic Hub*.
4. Search `SMIPS` with Creator name `KSHSlime`.
5. Load it.

## Installation

0. Make sure you have [Rust] installed.
1. Clone this repository:
```cmd
git clone https://github.com/iceice666/turing-complete-assembler.git
```
2. Build:
```cmd
cargo build --release
```

Executable will be in `target/release/assembler`

## Usage

```cmd
assembler <asm file>
```
It will generate a `bin` file in the same directory with same name as the `asm` file.

<!-- just a line -->
[Turing Complete Game]: https://store.steampowered.com/app/1444480/Turing_Complete/
[Rust]: https://www.rust-lang.org/
