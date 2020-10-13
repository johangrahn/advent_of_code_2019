pub enum Opcode {
    Add,
    Multiply,
    Quit,
}

pub fn parse_opcode(input: i64) -> Opcode {
    match input {
        1 => Opcode::Add,
        2 => Opcode::Multiply,
        99 => Opcode::Quit,
        _ => panic!("Can't parse opcode from number: {}", input),
    }
}
