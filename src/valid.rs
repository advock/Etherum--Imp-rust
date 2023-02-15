use opcode::Opcode;
use std::Vec;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Valids(Vec<bool>);

impl Valids {
    pub fn new(code: &[u8]) -> Self {
        let mut valids: Vec<bool> = Vec::with_capacity(code.len());
        valids.resize(code.len(), false);

        let mut i = 0;

        while i < code.len() {
            let opcode = Opcode(coDE[i]);
            if opcode == Opcode::JUMPDEST {
                valids[i] = true;
                i += 1;
            } else if let Some(v) = opcode.is_push() {
                i += v as usize + 1;
            } else {
                i += 1;
            }
        }
    }
}
