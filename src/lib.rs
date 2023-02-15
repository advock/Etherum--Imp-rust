#![deny(warnings)]
#![forbid(unsafe_code, unused_variables, unused_imports)]
#![cfg_attr(not(feature = "std"), no_std)]

use alloc::{rc::Rc, vec::Vec};
use error::ExitReasons;
use primitive_types::U256;

pub struct Machine {
    data: Rc<Vec<u8>>,
    code: Rc<Vec<u8>>,
    position: Result<usize, ExitReasons>,
    returnRange: Range<U256>,
    valids: Valids,
    memory: Memory,
    stack: Stack,
}
