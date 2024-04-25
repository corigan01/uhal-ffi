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

    pub fn list_devices(&mut self, regex: Option<&str>) -> Vec<String> {
        let device_list = if let Some(regex) = regex {
            let_cxx_string!(cxx_regex = regex);
            self.ffi_class.getDevices1(&cxx_regex)
        } else {
            self.ffi_class.getDevices()
        };

        device_list
            .iter()
            .map(|cxx_str| String::from(cxx_str.to_str().unwrap_or_default()))
            .collect()
    }
}
