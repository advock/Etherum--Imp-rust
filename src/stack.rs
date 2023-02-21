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
}
