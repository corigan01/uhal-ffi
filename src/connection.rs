use crate::{
    ffi::{self, rawbind},
    hw::HwInterface,
};
use anyhow::Result;
use cxx::{let_cxx_string, UniquePtr};

pub struct ConnectionManager {
    ffi_class: UniquePtr<ffi::ConnectionManager>,
}

impl ConnectionManager {
    pub fn new(xml_path: &str) -> Result<Self> {
        rawbind::disable_logging();
        let_cxx_string!(cxx_path = xml_path);
        let connection_manager = crate::ffi::resultbind::new_connection_manager_result(&cxx_path)?;

        Ok(Self {
            ffi_class: connection_manager,
        })
    }

    pub fn get_device(&mut self, device: &str) -> Result<HwInterface> {
        let_cxx_string!(cxx_name = device);
        let hw_interface = crate::ffi::resultbind::get_device_from_connection_manager_result(
            &mut self.ffi_class,
            &cxx_name,
        )?;

        Ok(HwInterface {
            ffi_class: hw_interface,
        })
    }
}
