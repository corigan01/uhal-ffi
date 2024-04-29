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

pub struct ValVector32 {
    pub(crate) ffi_class: UniquePtr<ffi::ValVector32>,
}

impl ValVector32 {
    pub fn value(&mut self) -> Result<Vec<u32>> {
        if !self.ready() {
            return Err(anyhow!("Not Valid"));
        }

        let pinned = self.ffi_class.pin_mut();
        let cxx_vector = unsafe {
            ffi::value_valvec((pinned.get_unchecked_mut() as *mut ffi::ValVector32).cast())
        };

        Ok(cxx_vector.into_iter().copied().collect())
    }

    pub fn ready(&mut self) -> bool {
        let pinned = self.ffi_class.pin_mut();
        unsafe { ffi::ready_valvec((pinned.get_unchecked_mut() as *mut ffi::ValVector32).cast()) }
    }
}
