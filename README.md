# nand2tetris

Course material - https://www.nand2tetris.org/

The hardware sections have all been implemented.  The assembler in Chapter 6 is done in Rust.  To use it, one will need to, ideally, have `cargo` installed.  This could be done by following [this](https://doc.rust-lang.org/cargo/getting-started/installation.html).

Once cargo is available:
- `git clone https://github.com/moodytongytong/nand2tetris.git`
- `cd nand2tetris/project/06/assembler`
- `cargo build --release`

This will build the assembler executable `project/06/assembler/target/release/assembler`

To use the executable on a `.asm`, one could run
- `<path to the binary> <path to the .asm file>` such as `target/release/assembler ../rect/Rect.asm`

Alternatively, without buildling the assembler executable, one could also do
- `cargo run <path to the .asm file>` from `project/06/assembler`


Due to time constraint, I'll return to the software portion of the project in 2024, and will hopefully implement everything in Rust :D
