use crate::parser::*;
use crate::parser::InstructionType::*;
use crate::code::*;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::prelude::*;

fn translate(instruction: Instruction) -> String {
    match &instruction.instruction_type {
        AInstruction => { find_a_binary(&instruction) },
        CInstruction => { find_c_binary(&instruction) },
        LInstruction => { "0000000000010000".to_string() }, // need to update this for more complicated cases
    }
}

fn find_a_binary(instruction: &Instruction) -> String {
    let mut translation = String::from("");
    translation.push_str("0");
    let address = instruction.symbol().parse::<u16>().unwrap();
    translation.push_str(&binary(address));
    translation
}

fn find_c_binary(instruction: &Instruction) -> String {
    let mut translation = String::from("");
    translation.push_str("111");
    translation.push_str(comp(&instruction.comp()));
    translation.push_str(dest(&instruction.dest()));
    translation.push_str(jump(&instruction.jump()));
    translation
}

pub fn read_input_from(filepath: &str) -> Vec<String> {
    let mut asm_input = Vec::new();
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(content) = line {
                if !content.contains("/") && !content.is_empty() { asm_input.push(content); }
            }
        }
    }
    asm_input
}

pub fn obtain_hack_binary_from(asm_input: Vec<String>) -> String {
    let mut hack_binary = String::from("");
    for asm in asm_input {
        hack_binary.push_str(&translate(Instruction::from(&asm)));
        hack_binary.push_str("\n");
    }
    hack_binary
}

pub fn write_binary_to_hack_file(filepath: &str, hack_binary: String) -> io::Result<()> {
    let mut hack_file = File::create(filepath)?;
    hack_file.write_all(hack_binary.as_bytes())?;
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use std::fs;


    #[test]
    fn translation_of_a_simple_a_instruction_correctly_done() {
        let asm = Instruction::from("@16");
        let binary = translate(asm);
        assert_eq!("0000000000010000", binary);

        let asm = Instruction::from("@17");
        let binary = translate(asm);
        assert_eq!("0000000000010001", binary);
    }

    #[test]
    fn translation_of_a_c_instruction_correctly_done() {
        let asm = Instruction::from("D=D-M");
        let binary = translate(asm);
        assert_eq!("1111010011010000", binary);

        let asm = Instruction::from("D   ;   JGT   ");
        let binary = translate(asm);
        assert_eq!("1110001100000001", binary);
    }

    #[test]
    fn accessing_the_file_with_asm_and_read_all_useful_lines() {
        let useful_asm_lines = read_input_from("test_data/Add.asm");
        assert_eq!(6, useful_asm_lines.len());
    }

    // #[test]
    // fn corresponding_hack_file_is_created() -> std::io::Result<()> {
    //     if Path::new("test_data/Add.hack").exists() {
    //         fs::remove_file("test_data/Add.hack")?;
    //     }

    //     #[allow(unused_must_use)]
    //     write_binary_to_hack_file("test_data/Add.hack", "foo".to_string());
    //     assert!(Path::new("test_data/Add.hack").exists());
    //     Ok(())
    // }

    #[test]
    fn create_string_with_full_hack_binary() {
        let useful_asm_lines = read_input_from("test_data/Add.asm");
        let hack_binary = obtain_hack_binary_from(useful_asm_lines);
        let expected = "0000000000000010\n1110110000010000\n0000000000000011\n1110000010010000\n0000000000000000\n1110001100001000\n";
        assert_eq!(expected, hack_binary);
    }
}