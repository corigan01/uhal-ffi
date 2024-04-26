use crate::{
    ffi::{self, NodePermission},
    valmem::ValWord32,
};
use anyhow::{anyhow, Result};

pub struct Node<'a> {
    pub(crate) ffi_class: &'a ffi::Node,
}

impl<'a> Node<'a> {
    pub fn read(&mut self) -> Result<ValWord32> {
        // This is to avoid having to bind with Result in C++, which
        // would be a pain.
        //
        // However other parts of C++ can still fail, and we would not
        // be able to handle that. FIXME: Bind with result.
        if self.ffi_class.getPermission() != &NodePermission::READ
            && self.ffi_class.getPermission() != &NodePermission::READWRITE
        {
            return Err(anyhow!("Not Readable"));
        }

        Ok(ValWord32 {
            ffi_class: self.ffi_class.read(),
        })
    }

    pub fn write(&mut self, value: u32) -> Result<()> {
        if self.ffi_class.getPermission() != &NodePermission::WRITE
            && self.ffi_class.getPermission() != &NodePermission::READWRITE
        {
            return Err(anyhow!("Not Writeable"));
        }

        let _ = self.ffi_class.write(&value);
        Ok(())
    }
}
