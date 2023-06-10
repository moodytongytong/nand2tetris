fn dest(argument: &str) -> &str {
    match argument {
        "" => "000",
        "M" => "001",
        "D" => "010",
        "DM" => "011",
        "A" => "100",
        "AM" => "101",
        "AD" => "110",
        "ADM" => "111",
        _ => "wrong dest",
    }
}

fn comp(argument: &str) -> &str {
    match argument {
        "0" => "0101010",
        "1" => "0111111",
        "-1" => "0111010",
        "D" => "0001100",
        "A" => "0110000",
        "!D" => "0001101",
        "!A" => "0110001",
        "-D" => "0001111",
        "-A" => "0110011",
        "D+1" => "0011111",
        "A+1" => "0110111",
        "D-1" => "0001110",
        "A-1" => "0110010",
        "D+A" => "0000010",
        "D-A" => "0010011",
        "A-D" => "0000111",
        "D&A" => "0000000",
        "D|A" => "0010101",
        "M" => "1110000",
        "!M" => "1110001",
        "-M" => "1110011",
        "M+1" => "1110111",
        "M-1" => "1110010",
        "D+M" => "1000010",
        "D-M" => "1010011",
        "M-D" => "1000111",
        "D&M" => "1000000",
        "D|M" => "1010101",
        _ => "wrong comp"
    }
}
// see if there are ways to simpify this.  If not, might just enumerate all possibilities

fn jump(argument: &str) -> &str {
    match argument {
        "" => "000",
        "JGT" => "001",
        "JEQ" => "010",
        "JGE" => "011",
        "JLT" => "100",
        "JNE" => "101",
        "JLE" => "110",
        "JMP" => "111",
        _ => "wrong jump"
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_code_of_dest_mnemonic_correctly_returned() {
        let dest_asm = "M";
        let dest_binary = dest(dest_asm);
        assert_eq!("001", dest_binary);

        let destination = "AD";
        let dest_binary = dest(destination);
        assert_eq!("110", dest_binary);
    }

    #[test]
    fn binary_code_of_comp_mnemonic_correctly_returned() {
        let comp_asm = "0";
        let comp_binary = comp(comp_asm);
        assert_eq!("0101010", comp_binary);

        let comp_asm = "D";
        let comp_binary = comp(comp_asm);
        assert_eq!("0001100", comp_binary);
    }

    #[test]
    fn binary_code_of_jump_mnemonic_correctly_returned() {
        let jump_asm = "JGT";
        let jump_binary = jump(jump_asm);
        assert_eq!("001", jump_binary);

        let jump_asm = "JMP";
        let jump_binary = jump(jump_asm);
        assert_eq!("111", jump_binary);
    }
}