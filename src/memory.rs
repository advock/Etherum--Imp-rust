use crate::{ExitError, ExitFatal};
use alloc::vec::Vec;
use core::cmp::min;
use core::ops::{BitAnd, Not};
use primitive_types::U256;

#[derive(Clone, Debug)]
pub struct memory {
    data: Vec<u8>,
    effective_len: U256,
    limit: usize,
}
