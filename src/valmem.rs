use anyhow::{anyhow, Result};
use cxx::UniquePtr;

use crate::ffi;

pub struct ValWord32 {
    pub(crate) ffi_class: UniquePtr<ffi::ValWord32>,
}

impl ValWord32 {
    pub fn value(&mut self) -> Result<u32> {
        if !self.is_valid() {
            return Err(anyhow!("Not Valid"));
        }

        Ok(ffi::value(
            self.ffi_class.as_ref().ok_or(anyhow!("Null field"))?,
        ))
    }

    pub fn is_valid(&mut self) -> bool {
        ffi::ready(self.ffi_class.pin_mut())
    }
}
