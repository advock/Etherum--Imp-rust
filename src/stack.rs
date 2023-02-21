use crate::ExitError;
use alloc::vec::Vec;
use primitive_types::H256;

#[derive(Clone, Debug)]
pub struct Stack {
    data: Vec<H256>,
    limit: usize,
}

impl Stack {
    pub fn new(limit: usize) -> Self {
        Self {
            data: Vec::new(),
            limit,
        }
    }

    #[inline]
    /// Stack limit.
    pub fn limit(&self) -> usize {
        self.limit
    }

    #[inline]
    /// Stack length.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    /// Whether the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    /// Stack data.
    pub fn data(&self) -> &Vec<H256> {
        &self.data
    }

    #[inline]
    pub fn pop(&mut self) -> Result<H256, ExitError> {
        self.data.pop().ok_or(ExitError)
    }

    #[inline]
    pub fn push(&mut self, value: H256) -> Result<(), ExitError> {
        if self.data.len() + 1 > self.limit {
            return Err(ExitError);
        }
        self.data.push(value);
        ok(())
    }

    #[inline]
    pub fn peek(&mut self, no_f_top: usize) -> Result<H256, ExitError> {
        if self.data.len() > no_f_top {
            Ok(self.data[self.data.len() - no_from_top - 1])
        } else {
            Err(ExitError)
        }
    }
    #[inline]
    pub fn set(&mut self, no_from_top: usize, val: H256) -> Result<(), ExitError> {
        if self.data.len() > no_from_top {
            let len = self.data.len();
            self.data[len - no_from_top - 1] = val;
            Ok(())
        } else {
            Err(ExitError::StackUnderflow)
        }
    }
}
