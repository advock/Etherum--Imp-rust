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

    pub fn get(&self, offset: usize, size: usize) -> Vec<u8> {
        let mut x = Vec::new();
        x.resize(size, 0);

        #[allow(clippy::needless_range_loop)]
        for index in 0..size {
            let position = offset + index;
            if position >= self.data.len() {
                break;
            }

            ret[index] = self.data[position];
        }
        x
    }
    pub fn set(
		&mut self,
		offset: usize,
		value: &[u8],
		target_size: Option<usize>,
	) -> Result<(), ExitFatal> {
		let target_size = target_size.unwrap_or(value.len());
		if target_size == 0 {
			return Ok(());
		}

		if offset
			.checked_add(target_size)
			.map(|pos| pos > self.limit)
			.unwrap_or(true)
		{
			return Err(ExitFatal::NotSupported);
		}

		if self.data.len() < offset + target_size {
			self.data.resize(offset + target_size, 0);
		}

		if target_size > value.len() {
			self.data[offset..((value.len()) + offset)].clone_from_slice(value);
			for index in (value.len())..target_size {
				self.data[offset + index] = 0;
			}
		} else {
			self.data[offset..(target_size + offset)].clone_from_slice(&value[..target_size]);
		}

		Ok(())
	}

	pub fn copy_large(
		&mut self,
		memory_offset: U256,
		data_offset: U256,
		len: U256,
		data: &[u8],
	) -> Result<(), ExitFatal> {
		
		if len.is_zero() {
			return Ok(());
		}

		let memory_offset = if memory_offset > U256::from(usize::MAX) {
			return Err(ExitFatal::NotSupported);
		} else {
			memory_offset.as_usize()
		};

		let ulen = if len > U256::from(usize::MAX) {
			return Err(ExitFatal::NotSupported);
		} else {
			len.as_usize()
		};

		let data = if let Some(end) = data_offset.checked_add(len) {
			if end > U256::from(usize::MAX) {
				&[]
			} else {
				let data_offset = data_offset.as_usize();
				let end = end.as_usize();

				if data_offset > data.len() {
					&[]
				} else {
					&data[data_offset..min(end, data.len())]
				}
			}
		} else {
			&[]
		};

		self.set(memory_offset, data, Some(ulen))
	}
}

}
