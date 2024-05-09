use crate::{
    ffi::{self, NodePermission},
    valmem::ValWord32,
};
use anyhow::{anyhow, Result};
use cxx::let_cxx_string;

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

    // FIXME: Since bindings are hard, these do not return ValVectors like
    //        they should. Instead these do all the work of sending values
    //        or reading values into a vector.

    pub fn read_block_dispatch(&mut self, size: usize) -> Result<Vec<u32>> {
        Ok(
            unsafe { ffi::resultbind::read_block_from_node(self.ffi_class, size as u32) }?
                .pin_mut()
                .into_iter()
                .map(|item| *item)
                .collect(),
        )
    }

    pub fn write_block_dispatch(&mut self, slice: &[u32]) -> Result<()> {
        unsafe {
            ffi::resultbind::write_block_from_node(
                self.ffi_class,
                slice.as_ptr(),
                slice.len() as u32,
            )?
        }
        Ok(())
    }

    pub fn get_node(&self, node: &str) -> Result<Node<'a>> {
        let_cxx_string!(cxx_string = node);

        let node = ffi::resultbind::get_node_from_node(&self.ffi_class, &cxx_string)?;
        Ok(Node::<'a> { ffi_class: node })
    }

    pub fn dispatch(&self) {
        ffi::resultbind::dispatch_from_node(&self.ffi_class);
    }
}
