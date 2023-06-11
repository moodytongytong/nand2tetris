
use InstructionType::*;

#[derive(Debug)]
pub struct Instruction {
    pub symbolic_code: String,
    pub instruction_type: InstructionType,
}

impl Instruction {
    pub fn from(line_asm: &str) -> Self {
        let symbolic_code = Instruction::remove_whitespaces(line_asm);
        let instruction_type = Instruction::find_type(&symbolic_code);
        Self {
            symbolic_code,
            instruction_type,
        }
    }

    fn find_type(instruction: &str) -> InstructionType {
        if instruction.contains("@") {
            AInstruction
        } else if instruction.contains("(") {
            LInstruction
        } else {
            CInstruction
        }
    }   

    fn remove_whitespaces(raw_instruction: &str) -> String {
        raw_instruction.replace(" ", "")
    }

    pub fn dest(&self) -> &str {
        if let Some(equal_index) = self.symbolic_code.find("=") {
            &self.symbolic_code[0..equal_index]
        } else {
            ""
        }
    }

    pub fn comp(&self) -> &str {
        let start_index = if let Some(equal_index) = self.symbolic_code.find("=") {
            equal_index + 1
        } else {
            0
        };
        if let Some(semicolon_index) = self.symbolic_code.find(";") {
            &self.symbolic_code[start_index..semicolon_index]
        } else {
            &self.symbolic_code[start_index..]
        }
    }

    pub fn jump(&self) -> &str {
        if let Some(semicolon_index) = self.symbolic_code.find(";") {
            &self.symbolic_code[semicolon_index+1 ..]
        } else {
            ""
        }
    }

    pub fn symbol(&self) -> &str {
        if let Some(closing_index) = self.symbolic_code.find(")") {
            &self.symbolic_code[1..closing_index]
        } else {
            &self.symbolic_code[1..]
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum InstructionType {
    AInstruction,
    CInstruction,
    LInstruction,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbolic_dest_correctly_picked_up_from_c_instruction() {
        let c_instruction = Instruction::from("M=D+M");
        let dest_asm = c_instruction.dest();
        assert_eq!("M", dest_asm);

        let c_instruction = Instruction::from("AD=M;JMP");
        let dest_asm = c_instruction.dest();
        assert_eq!("AD", dest_asm);
    }

    #[test]
    fn empty_symbolic_dest_correctly_returned_from_c_instruction_without_dest() {
        let c_instruction = Instruction::from("D;JGT");
        let dest_asm = c_instruction.dest();
        assert_eq!("", dest_asm);
    }

    #[test]
    fn symbolic_comp_correctly_picked_up_from_c_instruction_without_jump() {
        let c_instruction = Instruction::from("M=D+M");
        let comp_asm = c_instruction.comp();
        assert_eq!("D+M", comp_asm);

        let c_instruction = Instruction::from("M=M+1");
        let comp_asm = c_instruction.comp();
        assert_eq!("M+1", comp_asm);
    }

    #[test]
    fn symbolic_comp_correctly_picked_up_from_c_instruction_without_dest() {
        let c_instruction = Instruction::from("D;JMP");
        let comp_asm = c_instruction.comp();
        assert_eq!("D", comp_asm);
    }

    #[test]
    fn symbolic_comp_correctly_picked_up_from_full_c_instruction() {
        let c_instruction = Instruction::from("A=D+1;JMP");
        let comp_asm = c_instruction.comp();
        assert_eq!("D+1", comp_asm);
    }

    #[test]
    fn symbolic_jump_correctly_picked_up_from_full_c_instruction() {
        let c_instruction = Instruction::from("A=D+1;JMP");
        let jump_asm = c_instruction.jump();
        assert_eq!("JMP", jump_asm);

        let c_instruction = Instruction::from("D;JGT");
        let jump_asm = c_instruction.jump();
        assert_eq!("JGT", jump_asm);
    }

    #[test]
    fn empty_symbolic_jump_corretly_returned_from_partial_c_instruction() {
        let c_instruction = Instruction::from("M=M+1");
        let jump_asm = c_instruction.jump();
        assert_eq!("", jump_asm);
    }

    #[test]
    fn symbol_correctly_returned_from_a_instruction() {
        let a_instruction = Instruction::from("@sum");
        let symbol_asm = a_instruction.symbol();
        assert_eq!("sum", symbol_asm);

        let a_instruction = Instruction::from("@i");
        let symbol_asm = a_instruction.symbol();
        assert_eq!("i", symbol_asm);
    }

    #[test]
    fn symbol_correctly_returned_from_l_nstruction() {
        let l_instruction = Instruction::from("(LOOP)");
        let symbol_asm = l_instruction.symbol();
        assert_eq!("LOOP", symbol_asm);
    }

    #[test]
    fn correctly_identify_a_instruction_type() {
        let a_instruction = Instruction::from("@sum");
        assert_eq!(AInstruction, a_instruction.instruction_type);
    }

    #[test]
    fn correctly_identify_l_instruction_type() {
        let a_instruction = Instruction::from("(LOOP)");
        assert_eq!(LInstruction, a_instruction.instruction_type);
    }

    #[test]
    fn remove_whitespace_from_instruction() {
        let instruction = "  M = M + 1  ";
        let cleaned_instruction = Instruction::remove_whitespaces(instruction);
        assert_eq!("M=M+1", cleaned_instruction);
    }

    #[test]
    fn instruction_with_whitespaces_are_clean_at_instantiation() {
        let instruction = Instruction::from("    D   ;    JGT    ");
        assert_eq!("D;JGT", instruction.symbolic_code);
    }
}