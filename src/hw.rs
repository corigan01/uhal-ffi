use crate::{ffi, node::Node};
use anyhow::Result;
use cxx::{let_cxx_string, UniquePtr};

pub struct HwInterface {
    pub(crate) ffi_class: UniquePtr<ffi::HwInterface>,
}

impl HwInterface {
    pub fn get_node<'a>(&'a mut self, node: &str) -> Result<Node> {
        let_cxx_string!(cxx_node = node);
        let node = self.ffi_class.get_node(&cxx_node)?;

        Ok(Node { ffi_class: node })
    }

    pub fn dispatch(&mut self) {
        self.ffi_class.pin_mut().dispatch();
    }
}
