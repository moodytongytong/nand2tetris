fn write_push_pop(command: &str, segement: &str, index: u16) -> String {
    let mut assembly = String::from("");
    match command {
        "push" => {
            assembly.push_str("@SP\nA=M\nM=");
            assembly.push_str(&index.to_string());
            assembly.push_str("\n@SP\nM=M+1\n");
        }
        _ => {}
    }
    assembly
}

fn write_arithmetic(command: &str) -> String {
    "".to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translates_push_constant_command() {
        let expected = "@SP
A=M
M=7
@SP
M=M+1
".to_string();
        let assembly = write_push_pop("push", "constant", 7);
        assert_eq!(expected, assembly);

        let expected = "@SP
A=M
M=8
@SP
M=M+1
".to_string();
        let assembly = write_push_pop("push", "constant", 8);
        assert_eq!(expected, assembly);
    }
    // don't consider the segment yet

    #[test]
    fn translates_add_command() {
        let expected = "";
        let assembly = write_arithmetic("add");
        assert_eq!(expected, assembly);
    }
}