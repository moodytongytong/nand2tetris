use crate::parser::*;
use crate::parser::InstructionType::*;
use crate::code::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::prelude::*;

pub struct Assembler{
    symbol_table: HashMap<String, u16>,
    pub asm_input: Vec<String>,
}

impl Assembler{
    pub fn new()-> Self {
        let asm_input = Vec::new();
        let mut symbol_table = HashMap::new();
        for address in 0..=15 {
            let mut key = String::from("R");
            let address_str = address.to_string();
            key.push_str(&address_str);
            symbol_table.insert(key, address);
        }
        symbol_table.insert("SP".to_owned(), 0);
        symbol_table.insert("LCL".to_owned(), 1);
        symbol_table.insert("ARG".to_owned(), 2);
        symbol_table.insert("THIS".to_owned(), 3);
        symbol_table.insert("THAT".to_owned(), 4);
        symbol_table.insert("SCREEN".to_owned(), 16384);
        symbol_table.insert("KBD".to_owned(), 24576);
        Self {
            symbol_table,
            asm_input,
        }
    }

    pub fn read_input_from(&mut self, filepath: &str) {
        let mut address = 0;
        let mut allocation = 16;
        if let Ok(lines) = read_lines(filepath) {
            for line in lines {
                if let Ok(content) = line {
                    if !content.is_empty() && &content.trim()[0..1] == "(" {
                        self.symbol_table.insert(Instruction::from(&content).symbol().to_owned(), address); 
                    } else if !content.is_empty() && &content.trim()[0..=1] != "//" { 
                        if content.contains("@") && self.is_new_variable(&content) {
                            self.symbol_table.insert(Instruction::from(&content).symbol().to_owned(), allocation);
                            allocation += 1;
                        }
                        self.asm_input.push(content); 
                        address += 1;
                    }
                }
            }
        }
    }

    pub fn obtain_hack_binary(&mut self) -> String {
        let mut hack_binary = String::from("");
        for asm in &self.asm_input {
            hack_binary.push_str(&self.translate(Instruction::from(&asm)));
            hack_binary.push_str("\n");
        }
        hack_binary
    }
    
    fn translate(&self, instruction: Instruction) -> String {
        match &instruction.instruction_type {
            AInstruction => self.find_a_binary(&instruction),
            CInstruction => self.find_c_binary(&instruction),
            LInstruction => "".to_string(),
        }
    }

    fn find_a_binary(&self, instruction: &Instruction) -> String {
        let mut translation = String::from("");
        translation.push_str("0");
        let address = if let Ok(address) = instruction.symbol().parse::<u16>() { address } 
        else { self.symbol_table[instruction.symbol()] };
        translation.push_str(&binary(address));
        translation
    }

    fn find_c_binary(&self, instruction: &Instruction) -> String {
        let mut translation = String::from("");
        translation.push_str("111");
        translation.push_str(comp(&instruction.comp()));
        translation.push_str(dest(&instruction.dest()));
        translation.push_str(jump(&instruction.jump()));
        translation
    }

    fn is_new_variable(&self, asm: &String) -> bool {
        let instruction = Instruction::from(asm);
        if let Ok(_) = instruction.symbol().parse::<u16>() { false }
        else {
            if instruction.symbol()[0..1].chars().next().unwrap().is_uppercase() { false }
            else if self.symbol_table.contains_key(instruction.symbol()) { false }
            else { true }
        }
    }
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
        let assembler = Assembler::new();

        let asm = Instruction::from("@16");
        let binary = assembler.translate(asm);
        assert_eq!("0000000000010000", binary);

        let asm = Instruction::from("@17");
        let binary = assembler.translate(asm);
        assert_eq!("0000000000010001", binary);
    }

    #[test]
    fn translation_of_a_c_instruction_correctly_done() {
        let assembler = Assembler::new();

        let asm = Instruction::from("D=D-M");
        let binary = assembler.translate(asm);
        assert_eq!("1111010011010000", binary);

        let asm = Instruction::from("D   ;   JGT   ");
        let binary = assembler.translate(asm);
        assert_eq!("1110001100000001", binary);
    }

    #[test]
    fn accessing_the_file_with_asm_and_read_all_useful_lines() {
        let mut assembler = Assembler::new();
        assembler.read_input_from("test_data/Add.asm");
        assert_eq!(6, assembler.asm_input.len());
    }

    #[test]
    #[allow(unused_must_use)]
    fn corresponding_hack_file_is_created() -> std::io::Result<()> {
        if Path::new("test_data/Add.hack").exists() {
            fs::remove_file("test_data/Add.hack")?;
        }

        write_binary_to_hack_file("test_data/Add.hack", "foo".to_string());
        assert!(Path::new("test_data/Add.hack").exists());
        Ok(())
    }

    #[test]
    fn create_string_with_full_hack_binary() {
        let mut assembler = Assembler::new();
        assembler.read_input_from("test_data/Add.asm");
        let hack_binary = assembler.obtain_hack_binary();
        let expected = "0000000000000010\n1110110000010000\n0000000000000011\n1110000010010000\n0000000000000000\n1110001100001000\n";
        assert_eq!(expected, hack_binary);
    }

    #[test]
    fn predefined_symbols_correctly_created_when_instantiating_an_assembler_object() {
        let assembler = Assembler::new();
        assert_eq!(0, assembler.symbol_table["R0"]);
        assert_eq!(1, assembler.symbol_table["R1"]);
        assert_eq!(2, assembler.symbol_table["R2"]);
        assert_eq!(3, assembler.symbol_table["R3"]);
        assert_eq!(4, assembler.symbol_table["R4"]);
        assert_eq!(5, assembler.symbol_table["R5"]);
        assert_eq!(6, assembler.symbol_table["R6"]);
        assert_eq!(7, assembler.symbol_table["R7"]);
        assert_eq!(8, assembler.symbol_table["R8"]);
        assert_eq!(9, assembler.symbol_table["R9"]);
        assert_eq!(10, assembler.symbol_table["R10"]);
        assert_eq!(11, assembler.symbol_table["R11"]);
        assert_eq!(12, assembler.symbol_table["R12"]);
        assert_eq!(13, assembler.symbol_table["R13"]);
        assert_eq!(14, assembler.symbol_table["R14"]);
        assert_eq!(15, assembler.symbol_table["R15"]);
        assert_eq!(0, assembler.symbol_table["SP"]);
        assert_eq!(1, assembler.symbol_table["LCL"]);
        assert_eq!(2, assembler.symbol_table["ARG"]);
        assert_eq!(3, assembler.symbol_table["THIS"]);
        assert_eq!(4, assembler.symbol_table["THAT"]);
        assert_eq!(16384, assembler.symbol_table["SCREEN"]);
        assert_eq!(24576, assembler.symbol_table["KBD"]);
    }

    #[test]
    fn read_input_and_populate_symbol_table() {
        let mut assembler = Assembler::new();
        assembler.read_input_from("test_data/Max.asm");
        assert_eq!(16, assembler.asm_input.len());
        assert_eq!(10, assembler.symbol_table["OUTPUT_FIRST"]);
        assert_eq!(12, assembler.symbol_table["OUTPUT_D"]);
        assert_eq!(14, assembler.symbol_table["INFINITE_LOOP"]);
    }

    #[test]
    fn translation_of_an_a_instruction_with_predefined_label() {
        let assembler = Assembler::new();

        let asm = Instruction::from("@KBD");
        let binary = assembler.translate(asm);
        assert_eq!("0110000000000000", binary);
    }

    #[test]
    fn translation_of_an_a_instruction_with_user_label() {
        let mut assembler = Assembler::new();
        assembler.read_input_from("test_data/Max.asm");

        let asm = Instruction::from("@OUTPUT_FIRST");
        let binary = assembler.translate(asm);
        assert_eq!("0000000000001010", binary);
    }

    #[test]
    fn variables_are_correctly_registered() {
        let mut assembler = Assembler::new();
        assembler.read_input_from("test_data/Rect.asm");

        assert_eq!(16, assembler.symbol_table["counter"]);
        assert_eq!(17, assembler.symbol_table["address"]);
    }
}