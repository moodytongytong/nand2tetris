use assembler::assembler::Assembler;
use assembler::assembler::write_binary_to_hack_file;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut assembler = Assembler::new();
    assembler.read_input_from(&args[1]);
    let hack_binary = assembler.obtain_hack_binary();
    write_binary_to_hack_file(&args[1].replace(".asm", ".hack"), hack_binary)?;

    Ok(())
}

// might want to add a README if there's time
