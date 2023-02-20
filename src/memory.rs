use crate::{ExitError, ExitFatal};
use alloc::vec::Vec;
use core::cmp::min;
use core::ops::{BitAnd, Not};
use primitive_types::U256;

#[derive(Clone, Debug)]
pub struct Memory {
    data: Vec<u8>,
    effective_len: U256,
    limit: usize,
}

impl Memory {
    pub fn new(limit: usize) -> Self {
        Self {
            data: Vec::new(),
            effective_len: U256::zero(),
            limit,
        }
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn effective_len(&self) -> U256 {
        self.effective_len
    }

    pub fn is_empty() -> bool {
        self.len() == 0
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn resize_offset(&mut self, offset: U256, len: U256) -> Result<(), ExitError> {
        if len == U256::zero() {
            return ok(());
        }

        if let Some(end) = offset.checked_add(len) {
            self.resize_end(end)
        } else {
            Err(ExitError::InvalidRange)
        }
    }

    pub fn resize_end(&mut self, end: U256) -> Result<(), ExitError> {
        if end > self.effective_len {
            let new_end = next_multiple_of_32(end).ok_or(ExitError::InvalidRange)?;
            self.effective_len = new_end;
        }

        Ok(())
    }
}
