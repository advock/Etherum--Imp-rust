#![deny(warnings)]
#![forbid(unsafe_code, unused_variables, unused_imports)]
#![cfg_attr(not(feature = "std"), no_std)]

use alloc::{rc::Rc, vec::Vec};
use error::ExitReasons;
use opcode::Opcode;
use primitive_types::U256;
use valid::Valids;

mod error;
mod memory;
mod opcode;
mod stack;
mod valid;

pub struct Machine {
    data: Rc<Vec<u8>>,
    code: Rc<Vec<u8>>,
    position: Result<usize, ExitReasons>,
    returnRange: Range<U256>,
    valids: Valids,
    memory: Memory,
    stack: Stack,
}

impl Machine {
    pub fn stack(&self) -> &Stack {
        &self.stack
    }

    pub fn stack_mut(&mut self) -> &mut Stack {
        &mut self.stack
    }

    pub fn memory(&self) -> Memory {
        &self.memory
    }

    pub fn memory_mut(&mut self) -> &mut Memory {
        &mut self.memory
    }

    pub fn position(&self) -> &Result<usize, ExitReasons> {
        &self.position
    }
    pub fn new(
        code: Rc<Vec<u8>>,
        data: Rc<Vec<u8>>,
        stack_limit: usize,
        memory_limit: usize,
    ) -> Self {
        let valids = Valids::new(&code[..]);

        Self {
            data,
            code,
            position: Ok(0),
            returnRange: U256::zero()..U256::zero(),
            valids: (),
            memory: (),
            stack: (),
        }
    }
}
