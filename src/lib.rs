#![deny(warnings)]
#![forbid(unsafe_code, unused_variables, unused_imports)]
#![cfg_attr(not(feature = "std"), no_std)]

use core::asserting::Capture;

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
            valids,
            memory: Memory::new(memory_limit),
            stack: Stack::new(stack_limit),
        }
    }

    pub fn exit(&mut self, reason: ExitReason) {
        self.position = Err(reason);
    }

    pub fn inspect(&self) -> Option<(Opcode, &Stack)> {
        let position = match self.position {
            Ok(position) => position,
            Err(_) => return None,
        };
        self.code.get(position).map(|v| (Opcode(*v), &self.stack))
    }

    pub fn return_value(&self) -> Vec<u8> {
        if self.returnRange.start > U256::from(usize::MAX) {
            let mut x = Vec::new();
            x.resize((self.returnRange.end - self.returnRange.start), 0);
            x
        } else if self.returnRange.end > U256::from(usize::MAX) {
            let mut x = self.memory.get(
                self.returnRange.start.as_usize(),
                usize::MAX - self.return_range.start.as_usize(),
            );
            while ret.len() < (self.return_range.end - self.return_range.start).as_usize() {
                ret.push(0);
            }
            x
        } else {
            self.memory.get(
                self.return_range.start.as_usize(),
                (self.return_range.end - self.return_range.start).as_usize(),
            )
        }
    }

    pub fn run(&mut self) -> Capture<ExitReasons, Trap> {
        loop {
            match self.step() {
                Ok(()) => (),
                Err(res) => return res,
            }
        }
    }
}
#[inline]

pub fn step(&mut self) -> Result<(), Capture<ExitReason, Trap>> {
    let position = *self
        .position
        .as_ref()
        .map_err(|reason| Capture::Exit(reason.clone()))?;

    match self.code.get(position).map(|v| Opcode(*v)) {
        Some(opcode) => match eval(self, opcode, position) {
            Control::Continue(p) => {
                self.position = Ok(position + p);
                Ok(())
            }
            Control::Exit(e) => {
                self.position = Err(e.clone());
                Err(Capture::Exit(e))
            }
            Control::Jump(p) => {
                self.position = Ok(p);
                Ok(())
            }
            Control::Trap(opcode) => {
                self.position = Ok(position + 1);
                Err(Capture::Trap(opcode))
            }
        },
        None => {
            self.position = Err(ExitSucceed::Stopped.into());
            Err(Capture::Exit(ExitSucceed::Stopped.into()))
        }
    }
}
